use binrw::{BinRead, BinReaderExt};

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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EncoderType {
	#[brw(magic = 0u8)]
	PointCloud,
	#[brw(magic = 1u8)]
	TriangularMesh,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EncoderMethod {
	#[brw(magic = 0u8)]
	MeshSequentialEncoding,
	#[brw(magic = 1u8)]
	MeshEdgebreakerEncoding,
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"DRACO")]
#[derive(Debug, Clone)]
pub struct DracoHeader {
	pub major_version: u8,
	pub minor_version: u8,
	pub encoder_type: EncoderType,
	pub encoder_method: EncoderMethod,
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
pub enum AttDecDecoderType {
	#[brw(magic = 0u8)]
	MeshVertexAttribute,
	#[brw(magic = 1u8)]
	MeshCornerAttribute,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum AttDecTraversalMethod {
	#[brw(magic = 0u8)]
	MeshTraversalDepthFirst,
	#[brw(magic = 1u8)]
	MeshTraversalPredictionDegree,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct AttributeDecoderConfig {
	pub id: u8,
	pub decoder_type: AttDecDecoderType,
	pub traversal_mesthod: AttDecTraversalMethod,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct AttributeMetadata {
	pub att_dec_att_type: u8,
	pub att_dec_data_type: u8,
	pub att_dec_num_components: u8,
	pub att_dec_normalized: u8,
	#[br(parse_with = read_var_u32)]
	pub att_dec_unique_id: u32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct SequentialAttributeDecoderType {
	#[br(parse_with = read_var_u32)]
	pub seq_att_dec_decoder_type: u32,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct AttributeDecoderMetadata {
	#[br(temp)]
	#[br(parse_with = read_var_u32)]
	#[bw(try_calc = attribute_metadatas.len().try_into())]
	att_dec_num_attributes: u32,
	#[br(count = att_dec_num_attributes)]
	pub attribute_metadatas: Vec<AttributeMetadata>,
	#[br(count = att_dec_num_attributes)]
	pub seq_att_dec_decoder_type: Vec<SequentialAttributeDecoderType>,
}

// void SequentialGenerateSequence() {
//   for (i = 0; i < num_points; ++i) {
//     encoded_attribute_value_index_to_corner_map[curr_att_dec][i] = i;
//   }
// }
#[binrw::binrw]
#[brw(little)]
#[br(import_raw(header:&ConnectivityHeader))]
#[derive(Debug, Clone)]
pub struct AttributeCornerMap {
	// This is only mutated for edgebreaker, otherwise it's just t[i] = i
	// #[br(count = header.pos_count)]
	// pub encoded_attribute_value_index_to_corner_map: Vec<usize>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SeqAttDecPredictionScheme {
	#[brw(magic = -2i8)]
	PredictionNone,
	#[brw(magic = 0i8)]
	PredictionDifference,
	#[brw(magic = 1i8)]
	MeshPredictionParallelogram,
	#[brw(magic = 4i8)]
	MeshPredictionConstrainedMultiParallelogram,
	#[brw(magic = 5i8)]
	MeshPredictionTexCoordsPortable,
	#[brw(magic = 6i8)]
	MeshPredictionGeometricNormal,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SeqAttDecPredictionTransformType {
	#[brw(magic = 1u8)]
	PredictionTransformWrap,
	#[brw(magic = 3u8)]
	PredictionTransformNormalOctahedronCanonicalized,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct PredictionDataExt {
	pub seq_att_dec_prediction_transform_type: SeqAttDecPredictionTransformType,
	#[br(map=|value:u8|value!=0)]
	#[bw(map=|&value:&bool|value as u8)]
	pub seq_int_att_dec_compressed: bool,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct PredictionData {
	pub prediction_scheme: SeqAttDecPredictionScheme,
	#[br(if(prediction_scheme != SeqAttDecPredictionScheme::PredictionNone))]
	pub ext: Option<PredictionDataExt>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Attribute {
	pub prediction_data: PredictionData,
}

#[binrw::binrw]
#[brw(little)]
#[br(import(header:&DracoHeader,connectivity_header:&ConnectivityHeader))]
#[derive(Debug, Clone)]
// void DecodeAttributeData() {
pub struct Attributes {
	//   ParseAttributeDecodersData();
	#[br(temp)]
	#[bw(try_calc = atribute_decoder_configs.len().try_into())]
	pub attributes_decoders_count: u8,
	#[br(if(header.encoder_method == EncoderMethod::MeshEdgebreakerEncoding))]
	#[br(count = attributes_decoders_count)]
	pub atribute_decoder_configs: Vec<AttributeDecoderConfig>,
	#[br(count = attributes_decoders_count)]
	pub atribute_decoder_metadatas: Vec<AttributeDecoderMetadata>,
	// === SKIP ===
	// vertex_visited_point_ids.assign(num_attributes_decoders, 0);
	//  curr_att_dec = 0;
	//  if (encoder_method == MESH_EDGEBREAKER_ENCODING) {
	//    DecodeAttributeSeams();
	//    for (i = 0; i < num_encoded_vertices + num_encoded_split_symbols; ++i) {
	//      if (is_vert_hole_[i]) {
	//        UpdateVertexToCornerMap(i);
	//      }
	//    }
	//    for (i = 1; i < num_attributes_decoders; ++i) {
	//      curr_att_dec = i;
	//      RecomputeVerticesInternal();
	//    }
	//    Attribute_AssignPointsToCorners();
	//  }
	// ===========
	//  for (i = 0; i < num_attributes_decoders; ++i) {
	//    curr_att_dec = i;
	//    is_face_visited_.assign(num_faces, false);
	//    is_vertex_visited_.assign(num_faces * 3, false);
	//    GenerateSequence();
	//    if (encoder_method == MESH_EDGEBREAKER_ENCODING) {
	//      UpdatePointToAttributeIndexMapping();
	//    }
	//  }
	#[br(args_raw(binrw::VecArgs{count:attributes_decoders_count as usize,inner:connectivity_header}))]
	pub sequences: Vec<AttributeCornerMap>,
	// === SKIP ===
	//  for (i = 0; i < num_attributes_decoders; ++i) {
	//    for (j = 0; j < att_dec_num_attributes[i]; ++j) {
	//      att_dec_num_values_to_decode[i][j] =
	//          encoded_attribute_value_index_to_corner_map[i].size();
	//    }
	//  }
	// ===========
	//  for (i = 0; i < num_attributes_decoders; ++i) {
	//    curr_att_dec = i;
	//    DecodePortableAttributes();
	//    DecodeDataNeededByPortableTransforms();
	//    TransformAttributesToOriginalFormat();
	//  }
	// }
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Draco {
	pub len: u32, // 10177
	pub header: DracoHeader,
	pub connectivity_header: ConnectivityHeader,
	#[br(args_raw(&connectivity_header))]
	pub connectivity: Connectivity,
	#[br(args(&header,&connectivity_header))]
	pub attributes: Attributes,
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
	const _A: u32 = 126247;
	const _S: &str = unsafe { str::from_utf8_unchecked(&[70, 65, 67, 83]) };
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
	println!("len = {:?}", draco.len);
	println!("header = {:?}", draco.header);
	println!("face_count = {:?}", draco.connectivity_header.face_count);
	println!("pos_count = {:?}", draco.connectivity_header.pos_count);
	// println!("connectivity = {:?}", draco.connectivity);
	println!("attributes = {:?}", draco.attributes);

	let first_attribute: Attribute = cursor.read_le().unwrap();
	println!("first_attribute = {first_attribute:?}");

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
