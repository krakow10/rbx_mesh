use super::v2::{Face2, Vertex2};
use super::v4::{Bone4, Envelope4, Subset4};
use super::v5::{QuantizedTransforms5, ThreePoseCorrective5, TwoPoseCorrective5};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision7 {
	#[brw(magic = b"version 7.00")]
	Version700,
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
	pub unknown3_len: u32, // 15, 0, 0, 0,
	#[br(count = unknown3_len)]
	pub unknown3: Vec<u8>, // [0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Skinning {
	pub len: u32,
	#[br(temp)]
	#[bw(try_calc = envelopes.len().try_into())]
	pub envelope_count: u32,
	#[br(count = envelope_count)]
	pub envelopes: Vec<Envelope4>,

	#[br(temp)]
	#[bw(try_calc=bones.len().try_into())]
	pub bone_count: u32,
	#[br(count=bone_count)]
	pub bones: Vec<Bone4>,

	#[br(temp)]
	#[bw(try_calc=bone_names.len().try_into())]
	pub bone_names_len: u32,
	#[br(count=bone_names_len)]
	pub bone_names: Vec<u8>,

	#[br(temp)]
	#[bw(try_calc=subsets.len().try_into())]
	pub subset_count: u32,
	#[br(count=subset_count)]
	pub subsets: Vec<Subset4>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Facs7 {
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u32)]
	_ignore1: (),
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 1u32)]
	_ignore2: (),
	pub bytes_remaining1: u32, // 59186 -> remaining bytes in file after this number
	pub bytes_remaining2: u32, // 59182 -> remaining bytes in file after this number
	pub face_bone_names_len: u32, // 576
	pub face_control_names_len: u32, // 280
	pub unknown_count5: u32,   // 58068
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u32)]
	_ignore3: (),
	pub two_pose_correctives_len: u32,   // 192
	pub three_pose_correctives_len: u32, // 42
	#[br(count=face_bone_names_len)]
	pub face_bone_names: Vec<u8>,
	#[br(count=face_control_names_len)]
	pub face_control_names: Vec<u8>,
	pub quantized_transforms: QuantizedTransforms5,
	#[br(count=two_pose_correctives_len as usize/size_of::<TwoPoseCorrective5>())]
	pub two_pose_correctives: Vec<TwoPoseCorrective5>,
	#[br(count=three_pose_correctives_len as usize/size_of::<ThreePoseCorrective5>())]
	pub three_pose_correctives: Vec<ThreePoseCorrective5>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Mesh7Ext {
	#[brw(magic = b"SKINNING")]
	#[br(temp)]
	#[bw(try_calc = skinnings.len().try_into())]
	pub skinning_count: u32,
	#[br(count = skinning_count)]
	pub skinnings: Vec<Skinning>,
	#[brw(magic = b"FACS")]
	pub facs: Facs7,
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
	#[br(try)]
	pub ext: Option<Mesh7Ext>,
}

fn _math() {
	const _A: u32 = 1660;
}

#[test]
fn read_mesh7_127279296594138() {
	use crate::draco::{Connectivity, Draco};
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
	println!("len = {:?}", draco.len);
	println!("header = {:?}", draco.header);
	println!("face_count = {:?}", draco.connectivity_header.face_count);
	println!("pos_count = {:?}", draco.connectivity_header.pos_count);
	let Connectivity::Sequential(connectivity) = draco.connectivity;
	// println!("unknown4 = {:?}", connectivity.unknown4);
	// println!("connectivity = {:?}", draco.connectivity);
	println!("attributes = {:?}", draco.attributes);

	// let first_attribute: Attribute = cursor.read_le().unwrap();
	// println!("first_attribute = {first_attribute:?}");

	let pos = cursor.position();
	println!(
		"rest of data = {:?}",
		&coremesh2.draco.as_slice()[pos as usize..pos as usize + 16]
	);

	println!("lods = {:?}", mesh.lods);

	println!("draco.len() = {}", coremesh2.draco.len());
	assert_eq!(coremesh2.draco.len() as u64, cursor.position());
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

#[test]
fn read_mesh7_100025761449828_skinning() {
	use binrw::BinReaderExt;
	let data = std::fs::read("meshes/mesh7_100025761449828.bin").unwrap();
	let mut cursor = std::io::Cursor::new(data.as_slice());
	let _mesh: Mesh7 = cursor.read_le().unwrap();
	assert_eq!(data.len() as u64, cursor.position());
}
