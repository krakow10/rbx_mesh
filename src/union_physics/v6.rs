use super::v3::{Mesh, PhysicsInfo};

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS\x06\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS6 {
	pub physics_info: PhysicsInfo,
	pub mesh: Mesh,
}
