mod bit_buffer;
mod clers_symbol;
mod edgebreaker;
mod raw_hulls;
mod roblox_bit_reader;

use binrw::{BinRead, BinReaderExt};

use super::v7::GeomType7;
pub use edgebreaker::Hull;
pub use raw_hulls::Hulls;
pub use roblox_bit_reader::BitCounterError;

/// Hull information is accessed via mesh.hulls.iter_hulls()
#[binrw::binread]
#[br(little)]
#[br(magic = b"CSGPHS\x08\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS8 {
	pub geom_type: GeomType7,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	#[br(parse_with = read_mesh)]
	pub mesh: Mesh8,
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

fn read_mesh<R: BinReaderExt>(
	reader: &mut R,
	endian: binrw::Endian,
	args: (),
) -> binrw::BinResult<Mesh8> {
	// decode zstd
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

	let mut reader = std::io::Cursor::new(decoded);
	let pos = reader.position();
	let mesh = RawMesh8::read_options(&mut reader, endian, args)?;

	let symbol_reader = clers_symbol::SymbolReader::new(&mesh.clers_buffer, mesh.clers_bit_count)
		.map_err(|e| binrw::Error::Custom {
		pos,
		err: Box::new(e),
	})?;
	let capacity = mesh.face_count as usize * 3;
	let mut hull_decoder = edgebreaker::HullDecoder::new(symbol_reader, capacity);

	// extend raw_hulls with edgebreaker hulls
	let Hulls {
		mut face_ranges,
		mut faces,
		mut pos_ranges,
		mut positions,
	} = mesh.raw_hulls;

	let raw_hull_count = face_ranges.len().min(pos_ranges.len()).saturating_sub(1) as u32;

	let start_face = if let Some(&start_face) = face_ranges.last() {
		face_ranges.reserve_exact(mesh.hull_count as usize);
		start_face
	} else {
		// mesh has no raw hulls, leading zero must be added
		face_ranges.reserve_exact(mesh.hull_count as usize + 1);
		face_ranges.push(0);
		0
	};
	let start_pos = if let Some(&start_pos) = pos_ranges.last() {
		pos_ranges.reserve_exact(mesh.hull_count as usize);
		start_pos
	} else {
		// mesh has no raw hulls, leading zero must be added
		pos_ranges.reserve_exact(mesh.hull_count as usize + 1);
		pos_ranges.push(0);
		0
	};

	// decode the hulls and record the ranges
	for _ in 0..mesh.hull_count {
		hull_decoder
			.decode_hull()
			.map_err(|e| binrw::Error::Custom {
				pos: pos + (mesh.clers_bit_count - hull_decoder.remaining_bits() / u8::BITS) as u64,
				err: Box::new(e),
			})?;
		face_ranges.push(start_face + hull_decoder.current_face() * 3);
		pos_ranges.push(start_pos + hull_decoder.vertex_offset() * 3);
	}

	// This is the final form of the list before it is passed to the user, so allocate exactly
	faces.reserve_exact(capacity);
	positions.reserve_exact(mesh.positions.len());

	faces.extend(hull_decoder.into_indices());
	positions.extend(mesh.positions);

	Ok(Mesh8 {
		hulls: Hulls {
			face_ranges,
			faces,
			pos_ranges,
			positions,
		},
		raw_hull_count,
		aabb: mesh.aabb,
	})
}
