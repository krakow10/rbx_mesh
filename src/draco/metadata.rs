use super::conventions::read_var_u32;

pub const METADATA_FLAG_MASK: u16 = 1 << 15;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct BinaryString {
	#[br(temp)]
	#[bw(try_calc = bytes.len().try_into())]
	pub len: u8,
	#[br(count = len)]
	pub bytes: Vec<u8>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Entry {
	pub key: BinaryString,
	pub value: BinaryString,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct SubMetadata {
	pub key: BinaryString,
	pub metadata: Metadata,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Metadata {
	#[br(temp)]
	#[br(parse_with = read_var_u32)]
	#[bw(try_calc = entries.len().try_into())]
	pub entry_count: u32,
	#[br(count = entry_count)]
	pub entries: Vec<Entry>,
	#[br(temp)]
	#[br(parse_with = read_var_u32)]
	#[bw(try_calc = sub_metadatas.len().try_into())]
	pub sub_metadata_count: u32,
	#[br(count = sub_metadata_count)]
	pub sub_metadatas: Vec<SubMetadata>,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CustomMetadata {
	#[br(parse_with = read_var_u32)]
	pub id: u32,
	pub metadata: Metadata,
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Metadatas {
	#[br(temp)]
	#[br(parse_with = read_var_u32)]
	#[bw(try_calc = custom_metadatas.len().try_into())]
	pub custom_metadatas_count: u32,
	#[br(count = custom_metadatas_count)]
	pub custom_metadatas: Vec<CustomMetadata>,
	pub file_metadatas: Metadata,
}
