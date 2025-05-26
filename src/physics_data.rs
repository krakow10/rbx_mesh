pub type Error=binrw::Error;

#[inline]
pub fn read_versioned<R:binrw::BinReaderExt>(mut read:R)->Result<PhysicsData,Error>{
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct PhysicsInfo{
	pub volume:f32,
	pub center_of_gravity:[f32;3],
	// upper triangular matrix read left to right top to bottom
	pub moment_of_inertia_packed:[f32;6],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct VertexId(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Mesh{
	// concat_bytes!(16u32,0u128,16u32,0x3F800000000000000000000000000000u128)
	#[brw(magic=b"\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x3F")]
	pub vertex_count:u32,
	// vertex_width
	#[brw(magic=4u32)]
	#[br(count=vertex_count/3)]
	pub vertices:Vec<[f32;3]>,
	pub face_count:u32,
	#[br(count=face_count/3)]
	pub faces:Vec<[VertexId;3]>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct CSGPHS3{
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes:Vec<Mesh>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct CSGPHS6{
	pub physics_info:PhysicsInfo,
	pub mesh:Mesh,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct CSGPHS7{
	#[brw(magic=3u8)]
	pub physics_info:PhysicsInfo,
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes:Vec<Mesh>,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic=b"CSGPHS")]
#[derive(Debug,Clone)]
pub enum CSGPHS{
	// concat_bytes!(0u32,b"BLOCK")
	#[brw(magic=b"\0\0\0\0BLOCK")]
	Block,
	#[brw(magic=3u32)]
	V3(CSGPHS3),
	#[brw(magic=5u32)]
	V5(CSGPHS3),
	#[brw(magic=6u32)]
	V6(CSGPHS6),
	#[brw(magic=7u32)]
	V7(CSGPHS7),
}
#[binrw::binrw]
#[brw(little)]
#[brw(magic=b"CSGK")]
#[derive(Debug,Clone)]
pub struct CSGK{
	pub uuid_ascii_hex:[u8;32],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum PhysicsData{
	CSGK(CSGK),
	CSGPHS(CSGPHS),
}
