use binrw::BinReaderExt;

use std::io::{Read, Seek};

use super::v2::{Face2, SizeOfVertex2, Vertex2, Vertex2Truncated};
use super::DEFAULT_VERTEX_TANGENT;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision3 {
	#[brw(magic = b"3.00")]
	Version300,
	#[brw(magic = b"3.01")]
	Version301,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Header3 {
	#[brw(magic = b"version ")]
	pub revision: Revision3,
	#[brw(magic = b"\n\x10\0")] //newline,sizeof_header
	//sizeof_header:u16,//16=0x0010
	pub sizeof_vertex: SizeOfVertex2,
	#[brw(magic = b"\x0C\x04\0")] //sizeof_face,sizeof_lod
	//sizeof_face:u8,//12=0x0C
	//sizeof_lod:u16,//4=0x0004
	pub lod_count: u16,
	pub vertex_count: u32,
	pub face_count: u32,
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
/// Only one of {vertices,vertices_truncated} is populated based on header.sizeof_vertex
pub struct Mesh3 {
	pub header: Header3,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Full=>header.vertex_count,_=>0})]
	pub vertices: Vec<Vertex2>,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Truncated=>header.vertex_count,_=>0})]
	pub vertices_truncated: Vec<Vertex2Truncated>,
	#[br(count=header.face_count)]
	pub faces: Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods: Vec<Lod3>,
}

#[inline]
pub fn fix3(mesh: &mut Mesh3) {
	for vertex in &mut mesh.vertices {
		match vertex.tangent {
			[-128, -128, -128, -128] => vertex.tangent = DEFAULT_VERTEX_TANGENT,
			_ => (),
		}
	}
}

#[inline]
pub fn read_300<R: Read + Seek>(read: R) -> Result<Mesh3, binrw::Error> {
	let mut mesh = read3(read)?;
	fix3(&mut mesh);
	Ok(mesh)
}

pub fn read3<R: BinReaderExt>(mut read: R) -> Result<Mesh3, binrw::Error> {
	read.read_le()
}
