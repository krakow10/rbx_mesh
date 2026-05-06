use binrw::BinReaderExt;

pub const DEFAULT_VERTEX_TANGENT: [i8; 4] = [0, 0, -128, 127];

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
pub struct Header2 {
	pub revision: Revision2,
	#[brw(magic = b"\n\x0C\0")] //newline,sizeof_header
	//sizeof_header:u16,//12=0x000C
	pub sizeof_vertex: SizeOfVertex2,
	#[brw(magic = b"\x0C")] //sizeof_face
	//sizeof_face:u8,//12=0x0C
	pub vertex_count: u32,
	pub face_count: u32,
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
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct VertexId2(pub u32);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Face2(pub VertexId2, pub VertexId2, pub VertexId2);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// Only one of {vertices,vertices_truncated} is populated based on header.sizeof_vertex
pub struct Mesh2 {
	pub header: Header2,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Full=>header.vertex_count,_=>0})]
	pub vertices: Vec<Vertex2>,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Truncated=>header.vertex_count,_=>0})]
	pub vertices_truncated: Vec<Vertex2Truncated>,
	#[br(count=header.face_count)]
	pub faces: Vec<Face2>,
}

#[inline]
pub fn fix2(mesh: &mut Mesh2) {
	for vertex in &mut mesh.vertices {
		match vertex.tangent {
			[-128, -128, -128, -128] => vertex.tangent = DEFAULT_VERTEX_TANGENT,
			_ => (),
		}
	}
}

#[inline]
pub fn read_200<R: BinReaderExt>(read: R) -> Result<Mesh2, binrw::Error> {
	let mut mesh = read2(read)?;
	fix2(&mut mesh);
	Ok(mesh)
}

pub fn read2<R: BinReaderExt>(mut read: R) -> Result<Mesh2, binrw::Error> {
	read.read_le()
}
