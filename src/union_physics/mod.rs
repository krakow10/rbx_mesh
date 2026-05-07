mod v3;
pub use v3::*;
mod v5;
pub use v5::*;
mod v6;
pub use v6::*;
mod v7;
pub use v7::*;
mod v8;
pub use v8::*;

pub type Error = binrw::Error;

#[inline]
pub fn read_versioned<R: binrw::BinReaderExt>(mut read: R) -> Result<UnionPhysics, Error> {
	read.read_le()
}

/// CSGK contains no actual mesh data.  rbx_mesh does not have a method
/// to extract any meaningful information from it.
#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGK")]
#[derive(Debug, Clone)]
pub struct CSGK {
	pub uuid_ascii_hex: [u8; 32],
}

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS")]
#[derive(Debug, Clone)]
pub enum CSGPHS {
	// concat_bytes!(0u32,b"BLOCK")
	#[brw(magic = b"\0\0\0\0BLOCK")]
	Block,
	#[brw(magic = 3u32)]
	V3(CSGPHS3),
	#[brw(magic = 5u32)]
	V5(CSGPHS5),
	#[brw(magic = 6u32)]
	V6(CSGPHS6),
	#[brw(magic = 7u32)]
	V7(CSGPHS7),
	#[brw(magic = 8u32)]
	V8(CSGPHS8),
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum UnionPhysics {
	CSGK(CSGK),
	CSGPHS(CSGPHS),
}
