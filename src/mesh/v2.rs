#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision2 {
	#[brw(magic = b"version 2.00")]
	Version200,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum SizeOfVertex2 {
	#[brw(magic = 36u8)]
	Truncated,
	#[brw(magic = 40u8)]
	Full,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Vertex2 {
	pub pos: [f32; 3],
	pub norm: [f32; 3],
	pub tex: [f32; 2],
	pub tangent: [i8; 4], // Tangent Vector & Bi-Normal Direction
	pub color: [u8; 4],
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Vertex2Truncated {
	pub pos: [f32; 3],
	pub norm: [f32; 3],
	pub tex: [f32; 2],
	pub tangent: [i8; 4], // Tangent Vector & Bi-Normal Direction
}

#[binrw::binrw]
#[brw(little)]
#[br(import(sizeof_vertex:&SizeOfVertex2,vertex_count:u32))]
#[derive(Debug, Clone)]
pub enum Vertices2 {
	#[br(pre_assert(matches!(sizeof_vertex,SizeOfVertex2::Full)))]
	Full(#[br(count=vertex_count)] Vec<Vertex2>),
	#[br(pre_assert(matches!(sizeof_vertex,SizeOfVertex2::Truncated)))]
	Truncated(#[br(count=vertex_count)] Vec<Vertex2Truncated>),
}
impl Vertices2 {
	pub fn size(&self) -> SizeOfVertex2 {
		match self {
			Vertices2::Full(_) => SizeOfVertex2::Full,
			Vertices2::Truncated(_) => SizeOfVertex2::Truncated,
		}
	}
	pub fn len(&self) -> usize {
		match self {
			Vertices2::Full(vertices) => vertices.len(),
			Vertices2::Truncated(vertices_truncated) => vertices_truncated.len(),
		}
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct VertexId2(pub u32);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Face2(pub [VertexId2; 3]);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// Only one of {vertices,vertices_truncated} is populated based on header.sizeof_vertex
pub struct Mesh2 {
	pub revision: Revision2,
	#[brw(magic = b"\n\x0C\0")] //newline,sizeof_header
	//sizeof_header:u16,//12=0x000C
	#[br(temp)]
	#[bw(calc = vertices.size())]
	sizeof_vertex: SizeOfVertex2,
	#[brw(magic = b"\x0C")] //sizeof_face
	//sizeof_face:u8,//12=0x0C
	#[br(temp)]
	#[bw(try_calc=vertices.len().try_into())]
	pub vertex_count: u32,
	#[br(temp)]
	#[bw(try_calc=faces.len().try_into())]
	pub face_count: u32,
	#[br(args(&sizeof_vertex, vertex_count))]
	pub vertices: Vertices2,
	#[br(count=face_count)]
	pub faces: Vec<Face2>,
}
