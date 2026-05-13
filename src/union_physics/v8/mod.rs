mod bit_counter;
mod clers_symbol;
mod edgebreaker;
mod roblox_bit_reader;

use binrw::BinReaderExt;

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
	let mut decoder =
		ruzstd::decoding::StreamingDecoder::new(reader).map_err(|e| binrw::Error::Custom {
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
	pub position_count: u32,
	pub face_count: u32,
	pub first_hull_vert_count: u32,
	pub first_hull_face_count: u32,
	pub raw_hulls_len: u32,
	pub clers_bit_count: u32,
	pub clers_buffer_len: u32,
	pub positions_len: u32,
	pub aabb: Aabb,
	#[br(count = raw_hulls_len)]
	pub raw_hulls: Vec<u8>,
	#[br(count = clers_buffer_len)]
	pub clers_buffer: Vec<u8>,
	#[br(count = position_count)]
	pub positions: Vec<[f32; 3]>,
}

use bit_counter::BitCounterError;
use edgebreaker::Hull;

impl Mesh8 {
	pub fn hulls(&self) -> Result<Vec<Hull>, BitCounterError> {
		edgebreaker::decode_clers_buffer(
			&self.clers_buffer,
			self.clers_bit_count as usize,
			self.hull_count,
			self.face_count,
			self.position_count,
		)
	}
}
