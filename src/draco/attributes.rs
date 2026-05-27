use super::conventions::read_var_u32;
use super::sequential_connectivity::ConnectivityHeader;
use super::{DracoHeader, EncoderMethod};

#[binrw::binrw]
#[brw(little)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttDecDecoderType {
	#[brw(magic = 0u8)]
	MeshVertexAttribute,
	#[brw(magic = 1u8)]
	MeshCornerAttribute,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
	#[brw(magic = 0u8)]
	Invalid,
	#[brw(magic = 1u8)]
	Int8,
	#[brw(magic = 2u8)]
	Uint8,
	#[brw(magic = 3u8)]
	Int16,
	#[brw(magic = 4u8)]
	Uint16,
	#[brw(magic = 5u8)]
	Int32,
	#[brw(magic = 6u8)]
	Uint32,
	#[brw(magic = 7u8)]
	Int64,
	#[brw(magic = 8u8)]
	Uint64,
	#[brw(magic = 9u8)]
	Float32,
	#[brw(magic = 10u8)]
	Float64,
	#[brw(magic = 11u8)]
	Bool,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct AttributeMetadata {
	pub att_dec_att_type: u8,
	pub att_dec_data_type: DataType,
	pub att_dec_num_components: u8,
	pub att_dec_normalized: u8,
	#[br(parse_with = read_var_u32)]
	pub att_dec_unique_id: u32,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[br(import_raw(metadata:&AttributeDecoderMetadata))]
#[derive(Debug, Clone)]
pub struct SequentialIntegerAttributeDecoderDecodeIntegerValues {
	// num_components = GetNumComponents();
	// num_entries = att_dec_num_values_to_decode[curr_att_dec][curr_att];
	// num_values = num_entries * num_components;
	// if (seq_int_att_dec_compressed[curr_att_dec][curr_att] > 0) {
	//   DecodeSymbols(num_values, num_components, &decoded_symbols);
	// }
	// seq_int_att_dec_decoded_values[curr_att_dec][curr_att] = decoded_symbols;
	// if (num_values > 0) {
	//   if (seq_att_dec_prediction_transform_type[curr_att_dec][curr_att] ==
	//         PREDICTION_TRANSFORM_NORMAL_OCTAHEDRON_CANONICALIZED) {
	//     decoded_symbols = seq_int_att_dec_decoded_values[curr_att_dec][curr_att];
	//     for (i = 0; i < decoded_symbols.size(); ++i) {
	//       signed_vals[i] = decoded_symbols[i];
	//     }
	//     seq_int_att_dec_symbols_to_signed_ints[curr_att_dec][curr_att] = signed_vals;
	//   } else {
	//     ConvertSymbolsToSignedInts();
	//   }
	// }
	// if (seq_att_dec_prediction_scheme[curr_att_dec][curr_att] != PREDICTION_NONE) {
	//   DecodePredictionData(seq_att_dec_prediction_scheme[curr_att_dec][curr_att]);
	//   PredictionScheme_ComputeOriginalValues(
	//       seq_att_dec_prediction_scheme[curr_att_dec][curr_att], num_entries);
	// }
}

// void SequentialQuantizationAttributeDecoder_DequantizeValues() {
#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct GenericValues {
	// quantized_data_quantization_bits is uninitialized memory for Generic decoder???
	//
	// quantization_bits = quantized_data_quantization_bits[curr_att_dec][curr_att];
	// max_quantized_value = (1 << (quantization_bits)) - 1;
	// num_components = GetNumComponents();
	// quant_val_id = 0;
	// range_ = quantized_data_max_value_df[curr_att_dec][curr_att];
	// max_quantized_value_factor_ = 1.f / max_quantized_value;
	// min_value_ = quantized_data_min_values[curr_att_dec][curr_att];
	// original_values = seq_int_att_dec_original_values[curr_att_dec][curr_att];
	// num_values = att_dec_num_values_to_decode[curr_att_dec][curr_att];
	// for (i = 0; i < num_values; ++i) {
	//   for (c = 0; c < num_components; ++c) {
	//     value = DequantizeFloat(original_values[quant_val_id++],
	//                             max_quantized_value_factor_, range_);
	//     value = value + min_value_[c];
	//     att_val[c] = value;
	//     dequantized_data.push_back(value);
	//   }
	// }
	// seq_int_att_dec_dequantized_values[curr_att_dec][curr_att] = dequantized_data;
}

// void TransformAttributesToOriginalFormat() {
#[binrw::binrw]
#[brw(little)]
#[br(import_raw(decoder_type:SeqAttDecDecoderType))]
#[derive(Debug, Clone)]
pub enum Values {
	// for (i = 0; i < att_dec_num_attributes.back(); ++i) {
	//   curr_att = i;
	//   dec_type = seq_att_dec_decoder_type[curr_att_dec][curr_att];
	//   if (dec_type == SEQUENTIAL_ATTRIBUTE_ENCODER_NORMALS) {
	//     TransformAttributesToOriginalFormat_Normal();
	//   } else if (dec_type == SEQUENTIAL_ATTRIBUTE_ENCODER_INTEGER) {
	//     TransformAttributesToOriginalFormat_StoreValues();
	//   } else {
	//     SequentialQuantizationAttributeDecoder_DequantizeValues();
	//   }
	// }
	#[br(pre_assert(decoder_type==SeqAttDecDecoderType::SequentialAttributeEncoderGeneric))]
	Generic(GenericValues),
	#[br(pre_assert(decoder_type==SeqAttDecDecoderType::SequentialAttributeEncoderInteger))]
	Integer(),
	#[br(pre_assert(decoder_type==SeqAttDecDecoderType::SequentialAttributeEncoderQuantization))]
	Quantization(),
	#[br(pre_assert(decoder_type==SeqAttDecDecoderType::SequentialAttributeEncoderNormals))]
	Normals(),
	// SequentialAttributeEncoderGeneric
	// SequentialAttributeEncoderInteger
	// SequentialAttributeEncoderQuantization
	// SequentialAttributeEncoderNormals
}

#[binrw::binrw]
#[brw(little)]
#[br(import_raw(metadata:&AttributeDecoderMetadata))]
#[derive(Debug, Clone)]
pub struct Attribute {
	// DecodePortableAttributes();
	// pub prediction_data: PredictionData,
	// #[br(args_raw(metadata))]
	// #[br(if(prediction_data.prediction_scheme != SeqAttDecPredictionScheme::PredictionNone))]
	// pub sequential_integer_attribute_decoder_decode_integer_values:
	// 	Option<SequentialIntegerAttributeDecoderDecodeIntegerValues>,
	// DecodeDataNeededByPortableTransforms();
	// TransformAttributesToOriginalFormat();
	#[br(parse_with = binrw::helpers::args_iter(metadata.seq_att_dec_decoder_type.iter().copied()))]
	pub values: Vec<Values>,
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
	#[br(parse_with = binrw::helpers::args_iter(&atribute_decoder_metadatas))]
	pub portable_attributes: Vec<Attribute>,
}
