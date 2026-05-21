mod bit_buffer;
mod clers_symbol;
mod edgebreaker;
mod raw_hulls;
mod roblox_bit_reader;

use binrw::BinReaderExt;

use clers_symbol::SymbolReader;
pub use edgebreaker::{Hull, HullDecoder};
pub use raw_hulls::RawHulls;
pub use roblox_bit_reader::BitCounterError;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum GeomType {
	#[brw(magic = 0u8)]
	Type0,
	#[brw(magic = 2u8)]
	Type2,
	#[brw(magic = 3u8)]
	Type3,
}

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
	pub raw_hulls: RawHulls,
	#[br(count = clers_buffer_len)]
	pub clers_buffer: Vec<u8>,
	#[br(count = position_count)]
	pub positions: Vec<[f32; 3]>,
}

impl Mesh8 {
	pub(crate) fn symbol_reader(&self) -> Result<SymbolReader<'_>, BitCounterError> {
		SymbolReader::new(&self.clers_buffer, self.clers_bit_count as usize)
	}
	pub fn hull_decoder(&self) -> Result<HullDecoder<'_>, BitCounterError> {
		let symbol_reader = self.symbol_reader()?;
		let capacity = self.face_count as usize * 3;
		Ok(HullDecoder::new(symbol_reader, &self.positions, capacity))
	}
}
