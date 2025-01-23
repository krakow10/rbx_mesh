pub type Error=binrw::Error;

#[inline]
pub fn read<R:binrw::BinReaderExt>(mut read:R)->Result<PhysicsData,Error>{
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
pub struct MysteryInfo{
	pub _unknown1:u32,
	#[brw(magic=0u128)]
	pub _unknown2:u32,
	#[brw(magic=0x3F800000000000000000000000000000u128)]
	pub _nothing:(),
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct VertexId(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Mesh{
	pub mystery_info:MysteryInfo,
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
pub struct PhysicsInfoMesh{
	pub physics_info:PhysicsInfo,
	pub mesh:Mesh,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Meshes{
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes:Vec<Mesh>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum CollisionData{
	#[brw(magic=b"\0\0\0\0BLOCK")]
	Block,
	#[brw(magic=3u32)]
	Meshes(Meshes),
	#[brw(magic=6u32)]
	PhysicsInfoMesh(PhysicsInfoMesh),
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct PhysicsData{
	#[brw(magic=b"CSGPHS")]
	pub collision_data:CollisionData,
}
