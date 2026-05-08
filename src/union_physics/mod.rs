mod v3;
pub use v3::*;
mod v5;
pub use v5::*;
mod v6;
pub use v6::*;
mod v7;
pub use v7::*;

pub use super::union::*;

pub type Error = binrw::Error;

#[inline]
pub fn read_versioned<R: binrw::BinReaderExt>(mut read: R) -> Result<UnionPhysics, Error> {
	read.read_le()
}

/// This mesh is a rectangular prism, also known as a block.
#[binrw::binrw]
#[brw(little)]
// concat_bytes!(b"CSGPHS",0u32,b"BLOCK")
#[brw(magic = b"CSGPHS\0\0\0\0BLOCK")]
#[derive(Debug, Clone)]
pub struct Block;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum CSGPHS {
	Block(Block),
	V3(CSGPHS3),
	V5(CSGPHS5),
	V6(CSGPHS6),
	V7(CSGPHS7),
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum UnionPhysics {
	CSGK(CSGK),
	CSGPHS(CSGPHS),
}
