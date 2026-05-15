use super::v3::Mesh;

// v3 and v5 are identical
#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS\x05\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS5 {
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes: Vec<Mesh>,
}
