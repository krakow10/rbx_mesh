use binrw::{BinRead, BinReaderExt};

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

fn read_var_u32<R: BinReaderExt>(
	reader: &mut R,
	endian: binrw::Endian,
	args: (),
) -> binrw::BinResult<u32> {
	let mut result = 0;
	let mut shift = 0;
	loop {
		let byte = u8::read_options(reader, endian, args)?;
		result |= ((byte & 0b01111111) as u32) << shift;
		if byte & 0b10000000 == 0 {
			return Ok(result);
		}
		shift += 7;
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct ConnectivityHeader {
	#[br(parse_with = read_var_u32)]
	pub face_count: u32,
	#[br(parse_with = read_var_u32)]
	pub pos_count: u32,
}

#[binrw::binrw]
#[brw(little)]
#[br(import_raw(header:&ConnectivityHeader))]
#[derive(Debug, Clone)]
pub enum Connectivity {
	#[brw(magic = 1u8)]
	Sequential(#[br(args_raw(header))] SequentialConnectivity),
	// Edgebreaker
}

#[binrw::binrw]
#[brw(little)]
#[br(import_raw(header:&ConnectivityHeader))]
#[derive(Debug, Clone)]
pub struct SequentialConnectivity {
	#[br(count = header.face_count)]
	pub faces: Vec<[u16; 3]>, // index into float_triples
	// <- 0x684
	pub unknown4: [u8; 32],
	#[br(count = header.pos_count)]
	pub positions: Vec<[f32; 3]>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Draco {
	pub len: u32, // 10177
	pub header: Header,
	pub connectivity_header: ConnectivityHeader,
	#[br(args_raw(&connectivity_header))]
	pub connectivity: Connectivity,
	// <- 0x19b9
	pub unknown5: [u8; 5],
	#[br(count = 290)]
	pub unknown6: Vec<u8>,
	// <- 0x1ae0
	// kinda gave up here, there may be more subdivisions in unknown7
	#[br(count = 3330)]
	pub unknown7: Vec<u8>,
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

fn _math() {
	const _A: u32 = 192 - (40 * 4 + 12 * 2);
}

#[test]
fn read_mesh7_127279296594138() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_127279296594138.bin").unwrap();
	let mut bytes = std::io::Cursor::new(data.as_slice());
	let mesh: Mesh7 = bytes.read_le().unwrap();
	println!("data.len() = {}", data.len());
	assert_eq!(data.len() as u64, bytes.position());

	let Coremesh::V2(coremesh2) = mesh.coremesh else {
		panic!();
	};
	let mut cursor = std::io::Cursor::new(coremesh2.draco.as_slice());
	let draco: Draco = cursor.read_le().unwrap();
	macro_rules! print_first_8_and_last_8 {
		($field:ident) => {
			println!(
				"{}: first = {:?} last = {:?}",
				stringify!($field),
				draco.$field.get(0..8),
				draco.$field.get(draco.$field.len() - 8..)
			);
		};
	}
	println!("len = {:?}", draco.len);
	println!("header = {:?}", draco.header);
	println!("face_count = {:?}", draco.connectivity_header.face_count);
	println!("pos_count = {:?}", draco.connectivity_header.pos_count);
	println!("connectivity = {:?}", draco.connectivity);
	println!("unknown5 = {:?}", draco.unknown5);
	print_first_8_and_last_8!(unknown6);
	print_first_8_and_last_8!(unknown7);
	println!("lods = {:?}", mesh.lods);
}

#[test]
fn read_mesh7_86389496539231() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_86389496539231.bin").unwrap();
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
