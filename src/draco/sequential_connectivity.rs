use super::conventions::read_var_u32;

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
pub enum SequentialConnectivity {
	#[br(pre_assert(header.pos_count<(1<<8)))]
	U8 {
		#[br(count = header.face_count)]
		faces: Vec<[u8; 3]>,
	},
	#[br(pre_assert(header.pos_count<(1<<16)))]
	U16 {
		#[br(count = header.face_count)]
		faces: Vec<[u16; 3]>,
	},
	#[br(pre_assert(header.pos_count<(1<<21)))]
	VarU32 {
		#[br(count = header.face_count)]
		//TODO: #[br(parse_with = read_var_u32)]
		faces: Vec<[u32; 3]>,
	},
	U32 {
		#[br(count = header.face_count)]
		faces: Vec<[u32; 3]>,
	},
}
