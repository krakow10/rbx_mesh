use super::v3::{Mesh, PhysicsInfo};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CSGPHS7 {
	#[brw(magic = 3u8)]
	pub physics_info: PhysicsInfo,
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes: Vec<Mesh>,
}
