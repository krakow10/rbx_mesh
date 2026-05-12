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
impl QuantizedMatrix5 {
	/// The serialization length of this struct in bytes.
	fn len(&self) -> usize {
		// magic number
		size_of::<u16>()
		// enum variant
		+ match self {
			QuantizedMatrix5::Raw { x, y, .. } => {
				// x, y
				2 * size_of::<u32>()
				// matrix
				+ (x * y) as usize * size_of::<f32>()
			}
			QuantizedMatrix5::Quantized { x, y, .. } => {
				// x, y
				2 * size_of::<u32>()
				// lerp0, lerp1
				+ 2 * size_of::<f32>()
				// matrix
				+ (x * y) as usize * size_of::<u16>()
			}
		}
	}
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
impl QuantizedTransforms5 {
	/// The serialization length of this struct in bytes.
	fn len(&self) -> usize {
		self.px.len()
			+ self.py.len()
			+ self.pz.len()
			+ self.rx.len()
			+ self.ry.len()
			+ self.rz.len()
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ControlId5(pub u16);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct TwoPoseCorrective5(pub [ControlId5; 2]);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct ThreePoseCorrective5(pub [ControlId5; 3]);

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Facs5 {
	#[br(temp)]
	#[bw(try_calc=face_bone_names.len().try_into())]
	pub face_bone_names_len: u32,
	#[br(temp)]
	#[bw(try_calc=face_control_names.len().try_into())]
	pub face_control_names_len: u32,
	#[br(temp)]
	#[bw(try_calc=quantized_transforms.len().try_into())]
	pub quantized_transforms_len: u64,
	#[br(temp)]
	#[bw(try_calc=(two_pose_correctives.len()*size_of::<TwoPoseCorrective5>()).try_into())]
	pub two_pose_correctives_len: u32,
	#[br(temp)]
	#[bw(try_calc=(three_pose_correctives.len()*size_of::<ThreePoseCorrective5>()).try_into())]
	pub three_pose_correctives_len: u32,
	#[br(count=face_bone_names_len)]
	pub face_bone_names: Vec<u8>,
	#[br(count=face_control_names_len)]
	pub face_control_names: Vec<u8>,
	//is this not a list?
	pub quantized_transforms: QuantizedTransforms5,
	#[br(count=two_pose_correctives_len as usize/size_of::<TwoPoseCorrective5>())]
	pub two_pose_correctives: Vec<TwoPoseCorrective5>,
	#[br(count=three_pose_correctives_len as usize/size_of::<ThreePoseCorrective5>())]
	pub three_pose_correctives: Vec<ThreePoseCorrective5>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// envelopes has the same length as vertices when header.bone_count!=0
pub struct Mesh5 {
	pub revision: Revision5,
	#[brw(magic = b"\n\x20\0")] //newline,sizeof_header
	//sizeof_header:u16,//32=0x0020
	pub lod_type: LodType4,
	#[br(temp)]
	#[bw(try_calc=vertices.len().try_into())]
	pub vertex_count: u32,
	#[br(temp)]
	#[bw(try_calc=faces.len().try_into())]
	pub face_count: u32,
	#[br(temp)]
	#[bw(try_calc=lods.len().try_into())]
	pub lod_count: u16,
	#[br(temp)]
	#[bw(try_calc=bones.len().try_into())]
	pub bone_count: u16,
	#[br(temp)]
	#[bw(try_calc=bone_names.len().try_into())]
	pub bone_names_len: u32,
	#[br(temp)]
	#[bw(try_calc=subsets.len().try_into())]
	pub subset_count: u16,
	pub lod_hq_count: u8,
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic = 0u8)]
	_padding: (),
	pub facs_format: FacsFormat5,
	pub sizeof_facs: u32,
	#[br(count=vertex_count)]
	pub vertices: Vec<Vertex2>,
	#[br(count=if bone_count==0{0}else{vertex_count})]
	pub envelopes: Vec<Envelope4>,
	#[br(count=face_count)]
	pub faces: Vec<Face2>,
	#[br(count=lod_count)]
	pub lods: Vec<Lod3>,
	#[br(count=bone_count)]
	pub bones: Vec<Bone4>,
	#[br(count=bone_names_len)]
	pub bone_names: Vec<u8>,
	#[br(count=subset_count)]
	pub subsets: Vec<Subset4>,
	pub facs: Facs5,
}
