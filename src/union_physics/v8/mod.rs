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
	pub positions_count: u32,
	pub faces_count: u32,
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
	#[br(count = positions_count)]
	pub positions: Vec<[f32; 3]>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
	// 1 bit
	Continue, // 0b_0
	// 3 bits
	Split, // 0b00_1
	Left,  // 0b01_1
	Right, // 0b10_1
	End,   // 0b11_1
}

impl CSGPHS8Body {
	pub fn hulls(&self) -> Result<Vec<Hull>, EdgebreakerError> {
		decode_edgebreaker_hulls(
			&self.clers_buffer,
			self.clers_bit_count,
			self.hull_count,
			&self.positions,
			self.faces_count,
		)
	}
	pub fn decode_symbols(&self) -> Result<Vec<Symbol>, EdgebreakerError> {
		let mut symbols = Vec::new();
		let mut bits = edgebreaker::BitReader::new(&self.clers_buffer, self.clers_bit_count)?;
		// CLERS decoding
		while let Ok(bit) = bits.read_bit() {
			let symbol = if bit == 0 {
				Symbol::Continue
			} else {
				let b2 = bits.read_bit()? != 0;
				let b3 = bits.read_bit()? != 0;
				match (b2, b3) {
					(false, false) => Symbol::Split,
					(false, true) => Symbol::Left,
					(true, false) => Symbol::Right,
					(true, true) => Symbol::End,
				}
			};
			symbols.push(symbol);
		}
		Ok(symbols)
	}
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
