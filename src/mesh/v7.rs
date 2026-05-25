use super::v2::{Face2, Vertex2};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision7 {
	#[brw(magic = b"version 7.00")]
	Version700,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"DRACO")]
#[derive(Debug, Clone)]
pub struct Header {
	pub major_version: u8,
	pub minor_version: u8,
	pub encoder_type: u8,
	pub encoder_method: u8,
	pub flags: u16,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Coremesh {
	V1(Coremesh1),
	V2(Coremesh2),
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"COREMESH\x01\0\0\0")]
#[derive(Debug, Clone)]
pub struct Coremesh1 {
	#[br(temp)]
	#[bw(try_calc = (vertices.len()*size_of::<Vertex2>() + faces.len()*size_of::<Face2>()).try_into())]
	pub len: u32,
	#[br(temp)]
	#[bw(try_calc = vertices.len().try_into())]
	pub vertex_count: u32,
	#[br(count = vertex_count)]
	pub vertices: Vec<Vertex2>,
	#[br(temp)]
	#[bw(try_calc = faces.len().try_into())]
	pub face_count: u32,
	#[br(count = face_count)]
	pub faces: Vec<Face2>,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"COREMESH\x02\0\0\0")]
#[derive(Debug, Clone)]
pub struct Coremesh2 {
	pub draco_len: u32,
	#[br(count = draco_len)]
	pub draco: Vec<u8>,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"LODS")]
#[derive(Debug, Clone)]
pub struct Lods {
	// version 6.00 LODS
	// pub lod_type: u16,
	// pub num_high_quality_lods: u8,
	// pub lod_offsets_count: u32,
	// #[br(count = lod_offsets_count)]
	// pub lod_offsets: Vec<u32>,
	pub unknown1: u32,     // 0, 0, 0, 0,
	pub unknown2: u32,     // 1, 0, 0, 0,
	pub unknown3: u32,     // 15, 0, 0, 0,
	pub unknown4: [u8; 3], // 0, 0, 1,
	pub unknown5: u32,     // 2, 0, 0, 0,
	pub unknown6: u32,     // 0, 0, 0, 0,
	pub unknown7: u32,     // 0, 0, 0, 0
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh7 {
	pub revision: Revision7,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = b"\n")]
	_newline: (),
	pub coremesh: Coremesh,
	// <- 0x27E2
	pub lods: Lods,
}

fn _math(){
	const _A:u32=192-(40*4+12*2);
}

#[test]
fn read_mesh7() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_127279296594138.bin").unwrap();
	let mut bytes = std::io::Cursor::new(data.as_slice());
	let _mesh: Mesh7 = bytes.read_le().unwrap();
	println!("data.len() = {}", data.len());
	assert_eq!(data.len() as u64, bytes.position());
}

#[test]
fn read_mesh7_112807239761722() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_112807239761722.bin").unwrap();
	let mut bytes = std::io::Cursor::new(data.as_slice());
	let _mesh: Mesh7 = bytes.read_le().unwrap();
	println!("data.len() = {}", data.len());
	assert_eq!(data.len() as u64, bytes.position());
}
