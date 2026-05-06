use super::v3::{Mesh, PhysicsInfo};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CSGPHS6 {
	pub physics_info: PhysicsInfo,
	pub mesh: Mesh,
}
