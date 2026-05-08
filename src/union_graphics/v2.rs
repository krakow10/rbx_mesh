use super::{NormalIDError, NormalId, Obfuscator};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Hash {
	pub hash: [u8; 16], //784f216c8b49e5f6
	pub _unknown: [u8; 16],
}

#[binrw::binrw]
#[brw(little,repr=u32)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct NormalId2(pub NormalId);
impl From<&NormalId2> for u32 {
	#[inline]
	fn from(&NormalId2(value): &NormalId2) -> u32 {
		value as u32
	}
}
impl TryFrom<u32> for NormalId2 {
	type Error = NormalIDError;
	#[inline]
	fn try_from(value: u32) -> Result<NormalId2, NormalIDError> {
		Ok(NormalId2(match value {
			1 => NormalId::Right,
			2 => NormalId::Top,
			3 => NormalId::Back,
			4 => NormalId::Left,
			5 => NormalId::Bottom,
			6 => NormalId::Front,
			_ => return Err(NormalIDError),
		}))
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Vertex {
	pub pos: [f32; 3],
	pub norm: [f32; 3],
	pub color: [u8; 4],
	// NormalId is redundant and can simply be computed
	// from the normal axis with the largest magnitude.
	// Primarily used for textures.
	pub normal_id: NormalId2,
	pub tex: [f32; 2],
	#[brw(magic = 0u128)]
	pub tangent: [f32; 3],
	// This field does not exist in the final struct and
	// exists purely to de/serialize the magic number.
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u128)]
	_magic: (),
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct VertexId(pub u32);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh2 {
	pub vertex_count: u32,
	// vertex data length
	#[brw(magic = 84u32)]
	#[br(count=vertex_count)]
	pub vertices: Vec<Vertex>,
	pub face_count: u32,
	#[br(count=face_count/3)]
	pub faces: Vec<[VertexId; 3]>,
}

#[binrw::binrw]
#[brw(little)]
// CSGMDL4 is obfuscated
#[brw(map_stream = Obfuscator::new)]
// Magic does not have obfuscator applied
// reversible_obfuscate(0, concat_bytes!(b"CSGMDL", 2u32))
#[brw(magic = b"\x15\x7d\x29\x15\x75\x6c\x32\x04\x34\x69")]
#[derive(Debug, Clone)]
pub struct CSGMDL2 {
	pub hash: Hash,
	pub mesh: Mesh2,
}
