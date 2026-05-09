use super::v2::{Face2, SizeOfVertex2, Vertices2};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision3 {
	#[brw(magic = b"version 3.00")]
	Version300,
	#[brw(magic = b"version 3.01")]
	Version301,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
/// Lods are indices into faces, representing the start of the range of
/// faces to be drawn for a particular level of detail, with the end of
/// the range represented by the next id in the list.
pub struct Lod3(pub u32);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh3 {
	pub revision: Revision3,
	#[brw(magic = b"\n\x10\0")] //newline,sizeof_header
	//sizeof_header:u16,//16=0x0010
	#[br(temp)]
	#[bw(calc = vertices.size())]
	sizeof_vertex: SizeOfVertex2,
	#[brw(magic = b"\x0C\x04\0")] //sizeof_face,sizeof_lod
	//sizeof_face:u8,//12=0x0C
	//sizeof_lod:u16,//4=0x0004
	#[br(temp)]
	#[bw(try_calc=lods.len().try_into())]
	pub lod_count: u16,
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
	#[br(count=lod_count)]
	pub lods: Vec<Lod3>,
}
