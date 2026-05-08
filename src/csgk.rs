/// CSGK contains no actual mesh data.  rbx_mesh does not have a method
/// to extract any meaningful information from it.
#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGK")]
#[derive(Debug, Clone)]
pub struct CSGK {
	pub uuid_ascii_hex: [u8; 32],
}
