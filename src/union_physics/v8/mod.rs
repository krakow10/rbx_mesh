mod bit_buffer;
mod clers_symbol;
mod edgebreaker;
mod raw_hulls;
mod roblox_bit_reader;

use binrw::{BinRead, BinReaderExt};

pub use edgebreaker::Hull;
pub use raw_hulls::Hulls;
pub use roblox_bit_reader::BitCounterError;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum GeomType {
	#[brw(magic = 0u8)]
	Type0,
	#[brw(magic = 1u8)]
	Type1,
	#[brw(magic = 2u8)]
	Type2,
	#[brw(magic = 3u8)]
	Type3,
}

/// Hull information is accessed via mesh.hulls.iter_hulls() and mesh.raw_hulls.iter_hulls()
/// both of which can contain hull information simultaneously.
#[binrw::binread]
#[br(little)]
#[br(magic = b"CSGPHS\x08\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS8 {
	pub geom_type: GeomType,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	#[br(parse_with = read_mesh)]
	pub mesh: Mesh8,
}

fn read_mesh<R: BinReaderExt>(
	reader: &mut R,
	_endian: binrw::Endian,
	_: (),
) -> binrw::BinResult<Mesh8> {
	let pos = reader.stream_position()?;
	use std::io::Read;
	let mut decoded = Vec::new();
	#[cfg(feature = "csgphs-v8-ruzstd")]
	let mut decoder =
		ruzstd::decoding::StreamingDecoder::new(reader).map_err(|e| binrw::Error::Custom {
			pos,
			err: Box::new(e),
		})?;
	#[cfg(feature = "csgphs-v8-zstd")]
	let mut decoder = zstd::stream::Decoder::new(reader).map_err(|e| binrw::Error::Custom {
		pos,
		err: Box::new(e),
	})?;
	decoder.read_to_end(&mut decoded)?;
	std::io::Cursor::new(decoded).read_le()
}

#[binrw::binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct Aabb {
	pub min: [f32; 3],
	pub max: [f32; 3],
}

#[binrw::binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct Mesh8 {
	pub hull_count: u32,
	#[br(temp)]
	position_count: u32,
	pub face_count: u32,
	pub first_hull_pos_count: u32,
	pub first_hull_face_count: u32,
	pub raw_hulls_len: u32,
	pub clers_bit_count: u32,
	#[br(temp)]
	clers_buffer_len: u32,
	pub positions_len: u32,
	pub aabb: Aabb,
	#[br(if(raw_hulls_len != 0))]
	pub raw_hulls: Hulls,
	#[br(parse_with = decode_edgebreaker_hulls, args_raw(EdgebreakerArgs{face_count,hull_count,clers_bit_count,position_count,clers_buffer_len}))]
	pub hulls: Hulls,
}

struct EdgebreakerArgs {
	hull_count: u32,
	face_count: u32,
	clers_bit_count: u32,
	position_count: u32,
	clers_buffer_len: u32,
}

#[binrw::binread]
#[br(little)]
#[br(import_raw(edgebreaker_args: &EdgebreakerArgs))]
struct Edgebreaker {
	#[br(count = edgebreaker_args.clers_buffer_len)]
	clers_buffer: Vec<u8>,
	#[br(count = edgebreaker_args.position_count * 3)]
	positions: Vec<f32>,
}

fn decode_edgebreaker_hulls<R: BinReaderExt>(
	reader: &mut R,
	endian: binrw::Endian,
	args: EdgebreakerArgs,
) -> binrw::BinResult<Hulls> {
	use clers_symbol::SymbolReader;
	use edgebreaker::HullDecoder;
	let edgebreaker = Edgebreaker::read_options(reader, endian, &args)?;
	let symbol_reader =
		SymbolReader::new(&edgebreaker.clers_buffer, args.clers_bit_count as usize).unwrap();
	let capacity = args.face_count as usize * 3;
	let mut hull_decoder = HullDecoder::new(symbol_reader, capacity);

	let mut face_ranges = Vec::with_capacity(args.hull_count as usize + 1);
	let mut pos_ranges = Vec::with_capacity(args.hull_count as usize + 1);
	face_ranges.push(0);
	pos_ranges.push(0);

	for _ in 0..args.hull_count {
		hull_decoder.decode_hull().unwrap();
		face_ranges.push(hull_decoder.current_face() * 3);
		pos_ranges.push(hull_decoder.vertex_offset() * 3);
	}

	Ok(Hulls {
		face_ranges,
		faces: hull_decoder.into_indices().into_vec(),
		pos_ranges,
		positions: edgebreaker.positions,
	})
}
