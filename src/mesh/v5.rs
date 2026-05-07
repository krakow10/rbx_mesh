use super::v2::{Face2, Vertex2};
use super::v3::Lod3;
use super::v4::{Bone4, Envelope4, LodType4, Subset4};

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision5 {
	#[brw(magic = b"version 5.00")]
	Version500,
}

#[binrw::binrw]
#[brw(little,repr=u32)]
#[derive(Debug, Clone)]
pub enum FacsFormat5 {
	Format1 = 1,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Header5 {
	pub revision: Revision5,
	#[brw(magic = b"\n\x20\0")] //newline,sizeof_header
	//sizeof_header:u16,//32=0x0020
	pub lod_type: LodType4,
	pub vertex_count: u32,
	pub face_count: u32,
	pub lod_count: u16,
	pub bone_count: u16,
	pub bone_names_len: u32,
	pub subset_count: u16,
	pub lod_hq_count: u8,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	pub facs_format: FacsFormat5,
	pub sizeof_facs: u32,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// Quantized means interpolated from lerp0 to lerp1 based on [0-65535]
pub enum QuantizedMatrix5 {
	#[brw(magic = 1u16)]
	Raw {
		x: u32,
		y: u32,
		#[br(count=x*y)]
		matrix: Vec<f32>,
	},
	#[brw(magic = 2u16)]
	Quantized {
		x: u32,
		y: u32,
		lerp0: f32,
		lerp1: f32,
		#[br(count=x*y)]
		matrix: Vec<u16>,
	},
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct QuantizedTransforms5 {
	pub px: QuantizedMatrix5,
	pub py: QuantizedMatrix5,
	pub pz: QuantizedMatrix5,
	pub rx: QuantizedMatrix5,
	pub ry: QuantizedMatrix5,
	pub rz: QuantizedMatrix5,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ControlId5(pub u16);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct TwoPoseCorrective5(pub ControlId5, pub ControlId5);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct ThreePoseCorrective5(pub ControlId5, pub ControlId5, pub ControlId5);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Facs5 {
	pub face_bone_names_len: u32,
	pub face_control_names_len: u32,
	pub quantized_transforms_len: u64,
	pub two_pose_correctives_len: u32,
	pub three_pose_correctives_len: u32,
	#[br(count=face_bone_names_len)]
	pub face_bone_names: Vec<u8>,
	#[br(count=face_control_names_len)]
	pub face_control_names: Vec<u8>,
	//is this not a list?
	pub quantized_transforms: QuantizedTransforms5,
	#[br(count=two_pose_correctives_len as usize/std::mem::size_of::<TwoPoseCorrective5>())]
	pub two_pose_correctives: Vec<TwoPoseCorrective5>,
	#[br(count=three_pose_correctives_len as usize/std::mem::size_of::<ThreePoseCorrective5>())]
	pub three_pose_correctives: Vec<ThreePoseCorrective5>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// envelopes has the same length as vertices when header.bone_count!=0
pub struct Mesh5 {
	pub header: Header5,
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
	pub facs: Facs5,
}
