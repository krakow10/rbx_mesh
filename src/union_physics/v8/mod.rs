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
pub enum GeomType8 {
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
	pub geom_type: GeomType8,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	#[br(parse_with = read_mesh)]
	pub mesh: Mesh8,
}

fn read_mesh<R: BinReaderExt>(
	reader: &mut R,
	endian: binrw::Endian,
	args: (),
) -> binrw::BinResult<Mesh8> {
	let pos = reader.stream_position()?;
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
	std::io::copy(&mut decoder, &mut decoded)?;
	Mesh8::read_options(&mut std::io::Cursor::new(decoded), endian, args)
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
struct RawMesh8 {
	hull_count: u32,
	#[br(temp)]
	position_count: u32,
	face_count: u32,
	#[br(temp)]
	first_hull_pos_count: u32,
	#[br(temp)]
	first_hull_face_count: u32,
	#[br(temp)]
	raw_hulls_len: u32,
	clers_bit_count: u32,
	#[br(temp)]
	clers_buffer_len: u32,
	#[br(temp)]
	positions_len: u32,
	aabb: Aabb,
	#[br(if(raw_hulls_len != 0))]
	raw_hulls: Hulls,
	#[br(count = clers_buffer_len)]
	clers_buffer: Vec<u8>,
	#[br(count = position_count * 3)]
	positions: Vec<f32>,
}

/// The mesh format encodes hulls in two different ways: raw, and edgebreaker.
/// This is an implementation detail of the mesh format.  The edgebreaker
/// hulls have been appended to the raw hulls for simplicity, meaning the
/// raw hulls come first in hulls.iter_hulls().
#[derive(Debug, Clone)]
pub struct Mesh8 {
	pub raw_hull_count: u32,
	pub aabb: Aabb,
	pub hulls: Hulls,
}

impl binrw::BinRead for Mesh8 {
	type Args<'a> = ();
	fn read_options<R: BinReaderExt>(
		reader: &mut R,
		endian: binrw::Endian,
		args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		use clers_symbol::SymbolReader;
		use edgebreaker::HullDecoder;
		let pos = reader.stream_position()?;
		let raw_mesh = RawMesh8::read_options(reader, endian, args)?;
		let symbol_reader = SymbolReader::new(&raw_mesh.clers_buffer, raw_mesh.clers_bit_count)
			.map_err(|e| binrw::Error::Custom {
				pos,
				err: Box::new(e),
			})?;
		let capacity = raw_mesh.face_count as usize * 3;
		let mut hull_decoder = HullDecoder::new(symbol_reader, capacity);

		let Hulls {
			mut face_ranges,
			mut faces,
			mut pos_ranges,
			mut positions,
		} = raw_mesh.raw_hulls;

		let raw_hull_count = face_ranges.len().min(pos_ranges.len()).saturating_sub(1) as u32;

		let start_face = if let Some(&start_face) = face_ranges.last() {
			face_ranges.reserve_exact(raw_mesh.hull_count as usize);
			start_face
		} else {
			// mesh has no raw hulls, leading zero must be added
			face_ranges.reserve_exact(raw_mesh.hull_count as usize + 1);
			face_ranges.push(0);
			0
		};
		let start_pos = if let Some(&start_pos) = pos_ranges.last() {
			pos_ranges.reserve_exact(raw_mesh.hull_count as usize);
			start_pos
		} else {
			// mesh has no raw hulls, leading zero must be added
			pos_ranges.reserve_exact(raw_mesh.hull_count as usize + 1);
			pos_ranges.push(0);
			0
		};

		for _ in 0..raw_mesh.hull_count {
			hull_decoder
				.decode_hull()
				.map_err(|e| binrw::Error::Custom {
					pos: pos
						+ (raw_mesh.clers_bit_count - hull_decoder.remaining_bits() / u8::BITS)
							as u64,
					err: Box::new(e),
				})?;
			face_ranges.push(start_face + hull_decoder.current_face() * 3);
			pos_ranges.push(start_pos + hull_decoder.vertex_offset() * 3);
		}

		// This is the final form of the list before it is passed to the user, so allocate exactly
		faces.reserve_exact(capacity);
		positions.reserve_exact(raw_mesh.positions.len());

		faces.extend(hull_decoder.into_indices());
		positions.extend(raw_mesh.positions);

		Ok(Mesh8 {
			hulls: Hulls {
				face_ranges,
				faces,
				pos_ranges,
				positions,
			},
			raw_hull_count,
			aabb: raw_mesh.aabb,
		})
	}
}
