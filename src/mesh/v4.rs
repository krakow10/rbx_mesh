use super::v2::{Face2, Vertex2};
use super::v3::Lod3;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision4 {
	#[brw(magic = b"version 4.00")]
	Version400,
	#[brw(magic = b"version 4.01")]
	Version401,
}

#[binrw::binrw]
#[brw(little,repr=u16)]
#[derive(Debug, Clone)]
pub enum LodType4 {
	None = 0,
	Unknown = 1,
	RbxSimplifier = 2,
	ZeuxMeshOptimizer = 3,
	Type4 = 4, //shows up in sphere.mesh, don't know what it is
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Header4 {
	pub revision: Revision4,
	#[brw(magic = b"\n\x18\0")] //newline,sizeof_header
	//sizeof_header:u16,//24
	pub lod_type: LodType4,
	pub vertex_count: u32,
	pub face_count: u32,
	pub lod_count: u16,
	pub bone_count: u16,
	pub bone_names_len: u32,
	pub subset_count: u16,
	pub lod_hq_count: u8,
	pub _padding: u8,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Envelope4 {
	pub bones: [u8; 4],
	pub weights: [u8; 4],
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct BoneId4(u16);
impl BoneId4 {
	pub fn new(value: Option<u16>) -> Self {
		Self(value.unwrap_or(0xFFFF))
	}
	pub fn get(&self) -> Option<u16> {
		match self.0 {
			0xFFFF => None,
			other => Some(other),
		}
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CFrame4 {
	pub r00: f32,
	pub r01: f32,
	pub r02: f32,
	pub r10: f32,
	pub r11: f32,
	pub r12: f32,
	pub r20: f32,
	pub r21: f32,
	pub r22: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Bone4 {
	pub bone_name_pos: u32,
	pub parent: BoneId4,
	pub lod_parent: BoneId4,
	pub cull_distance: f32,
	pub cframe: CFrame4,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Subset4 {
	pub faces_offset: u32,
	pub faces_len: u32,
	pub vertices_offset: u32,
	pub vertices_len: u32,
	pub bone_count: u32,
	pub bones: [BoneId4; 26],
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// envelopes has the same length as vertices when header.bone_count!=0
pub struct Mesh4 {
	pub header: Header4,
	#[br(count=header.vertex_count)]
	pub vertices: Vec<Vertex2>,
	#[br(count=if header.bone_count==0{0}else{header.vertex_count})]
	pub envelopes: Vec<Envelope4>,
	#[br(count=header.face_count)]
	pub faces: Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods: Vec<Lod3>,
	#[br(count=header.bone_count)]
	pub bones: Vec<Bone4>,
	#[br(count=header.bone_names_len)]
	pub bone_names: Vec<u8>,
	#[br(count=header.subset_count)]
	pub subsets: Vec<Subset4>,
}
