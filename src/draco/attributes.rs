use super::conventions::read_var_u32;
use super::sequential_connectivity::ConnectivityHeader;
use super::{DracoHeader, EncoderMethod};

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
pub enum SeqAttDecDecoderType {
	#[brw(magic = 0u8)]
	SequentialAttributeEncoderGeneric,
	#[brw(magic = 1u8)]
	SequentialAttributeEncoderInteger,
	#[brw(magic = 2u8)]
	SequentialAttributeEncoderQuantization,
	#[brw(magic = 3u8)]
	SequentialAttributeEncoderNormals,
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
	pub seq_att_dec_decoder_type: Vec<SeqAttDecDecoderType>,
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

// fn get_num_components(header:)

// void SequentialIntegerAttributeDecoder_DecodeIntegerValues() {
#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct SequentialIntegerAttributeDecoderDecodeIntegerValues {
	pub prediction_scheme: SeqAttDecPredictionScheme,
	#[br(if(prediction_scheme != SeqAttDecPredictionScheme::PredictionNone))]
	pub ext: Option<PredictionDataExt>,
}
//   num_components = GetNumComponents();
//   num_entries = att_dec_num_values_to_decode[curr_att_dec][curr_att];
//   num_values = num_entries * num_components;
//   if (seq_int_att_dec_compressed[curr_att_dec][curr_att] > 0) {
//     DecodeSymbols(num_values, num_components, &decoded_symbols);
//   }
//   seq_int_att_dec_decoded_values[curr_att_dec][curr_att] = decoded_symbols;
//   if (num_values > 0) {
//     if (seq_att_dec_prediction_transform_type[curr_att_dec][curr_att] ==
//           PREDICTION_TRANSFORM_NORMAL_OCTAHEDRON_CANONICALIZED) {
//       decoded_symbols = seq_int_att_dec_decoded_values[curr_att_dec][curr_att];
//       for (i = 0; i < decoded_symbols.size(); ++i) {
//         signed_vals[i] = decoded_symbols[i];
//       }
//       seq_int_att_dec_symbols_to_signed_ints[curr_att_dec][curr_att] = signed_vals;
//     } else {
//       ConvertSymbolsToSignedInts();
//     }
//   }
//   if (seq_att_dec_prediction_scheme[curr_att_dec][curr_att] != PREDICTION_NONE) {
//     DecodePredictionData(seq_att_dec_prediction_scheme[curr_att_dec][curr_att]);
//     PredictionScheme_ComputeOriginalValues(
//         seq_att_dec_prediction_scheme[curr_att_dec][curr_att], num_entries);
//   }
// }

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Attribute {
	pub prediction_data: PredictionData,
	#[br(if(prediction_data.prediction_scheme != SeqAttDecPredictionScheme::PredictionNone))]
	pub sequential_integer_attribute_decoder_decode_integer_values:
		Option<SequentialIntegerAttributeDecoderDecodeIntegerValues>,
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
	#[br(args{
		count:attributes_decoders_count as usize,
		inner:connectivity_header,
	})]
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
