use super::v2::{Hash, Mesh2};

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGMDL")]
#[derive(Debug, Clone)]
pub struct CSGMDL4 {
	#[brw(magic = 4u32)]
	pub hash: Hash,
	pub mesh: Mesh2,
	pub _unknown1_count: u32,
	#[br(count=_unknown1_count)]
	pub _unknown1_list: Vec<u32>,
}
