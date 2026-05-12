use binrw::{BinRead, BinResult, Endian};
use std::io::{Cursor, Read, Seek, SeekFrom};

mod edgebreaker;
mod raw_hulls;

pub use edgebreaker::{decode_edgebreaker_hulls, EdgebreakerError, Hull};
pub use raw_hulls::decode_raw_hulls;

const ZSTD_FRAME_MAGIC: u32 = 0xFD2FB528;

#[binrw::binread]
#[br(little)]
#[br(magic = b"CSGPHS\x08\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS8 {
	pub geom_type: u8,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	#[br(parse_with = parse_body)]
	pub body: CSGPHS8Body,
}

#[binrw::binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct CSGPHS8Body {
	pub hull_count: u32,
	pub total_verts: u32,
	pub total_faces: u32,
	pub first_hull_vert_count: u32,
	pub first_hull_face_count: u32,
	pub raw_hulls_size: u32,
	pub clers_bit_count: u32,
	pub clers_buffer_size: u32,
	pub positions_size: u32,
	pub bbox_min: [f32; 3],
	pub bbox_max: [f32; 3],
	#[br(count = raw_hulls_size)]
	pub raw_hulls: Vec<u8>,
	#[br(count = clers_buffer_size / 4)]
	pub clers_buffer: Vec<u32>,
	#[br(count = total_verts)]
	pub positions: Vec<[f32; 3]>,
}

fn parse_body<R: Read + Seek>(reader: &mut R, endian: Endian, _: ()) -> BinResult<CSGPHS8Body> {
	// peek the next 4 bytes to detect a Zstd frame, then rewind
	let body_start = reader.stream_position()?;
	let maybe_magic = u32::read_options(reader, endian, ())?;
	reader.seek(SeekFrom::Start(body_start))?;

	if maybe_magic == ZSTD_FRAME_MAGIC {
		let decompressed = decompress_zstd(reader).map_err(|e| binrw::Error::Custom {
			pos: body_start,
			err: Box::new(e),
		})?;
		CSGPHS8Body::read_options(&mut Cursor::new(decompressed), endian, ())
	} else {
		CSGPHS8Body::read_options(reader, endian, ())
	}
}

fn decompress_zstd<R: Read>(reader: R) -> std::io::Result<Vec<u8>> {
	use ruzstd::decoding::StreamingDecoder;
	let mut decoder = StreamingDecoder::new(reader).map_err(std::io::Error::other)?;
	let mut out = Vec::new();
	decoder.read_to_end(&mut out)?;
	Ok(out)
}
