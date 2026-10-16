mod attributes;
mod conventions;
mod metadata;
mod sequential_connectivity;

pub use attributes::*;
pub use metadata::*;
pub use sequential_connectivity::*;

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

/// https://google.github.io/draco/spec/
#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Draco {
	pub len: u32, // 10177
	pub header: DracoHeader,
	#[br(if(header.flags&METADATA_FLAG_MASK!=0))]
	pub metadata: Option<Metadatas>,
	pub connectivity_header: ConnectivityHeader,
	#[br(args_raw(&connectivity_header))]
	pub connectivity: Connectivity,
	#[br(args(&header,&connectivity_header))]
	pub attributes: Attributes,
}
