use super::v2::{Hash, Mesh2};
use super::Obfuscator;

#[binrw::binrw]
#[brw(little)]
// CSGMDL4 is obfuscated
#[brw(map_stream = Obfuscator::new)]
// Magic does not have obfuscator applied
// reversible_obfuscate(0, concat_bytes!(b"CSGMDL", 4u32))
#[brw(magic = b"\x15\x7d\x29\x15\x75\x6c\x34\x04\x34\x69")]
#[derive(Debug, Clone)]
pub struct CSGMDL4 {
	pub hash: Hash,
	pub mesh: Mesh2,
	#[br(temp)]
	#[bw(try_calc=_unknown1_list.len().try_into())]
	pub _unknown1_count: u32,
	#[br(count=_unknown1_count)]
	pub _unknown1_list: Vec<u32>,
}
