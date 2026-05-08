#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct PhysicsInfo {
	pub volume: f32,
	pub center_of_gravity: [f32; 3],
	// upper triangular matrix read left to right top to bottom
	pub moment_of_inertia_packed: [f32; 6],
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct VertexId(pub u32);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh {
	// concat_bytes!(16u32,0u128,16u32,0x3F800000000000000000000000000000u128)
	#[brw(
		magic = b"\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x3F"
	)]
	pub vertex_count: u32,
	// vertex_width
	#[brw(magic = 4u32)]
	#[br(count=vertex_count/3)]
	pub vertices: Vec<[f32; 3]>,
	pub face_count: u32,
	#[br(count=face_count/3)]
	pub faces: Vec<[VertexId; 3]>,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS\x03\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS3 {
	#[br(parse_with=binrw::helpers::until_eof)]
	pub meshes: Vec<Mesh>,
}
