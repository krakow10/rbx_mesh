use super::v3::{Mesh, PhysicsInfo};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum GeomType7 {
	#[brw(magic = 0u8)]
	Type0,
	#[brw(magic = 1u8)]
	Type1,
	#[brw(magic = 2u8)]
	Type2,
	#[brw(magic = 3u8)]
	Type3,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS\x07\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS7 {
	pub geom_type: GeomType7,
	pub physics_info: PhysicsInfo,
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes: Vec<Mesh>,
}
