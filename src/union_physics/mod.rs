mod block;
pub use block::*;
mod v3;
pub use v3::*;
mod v5;
pub use v5::*;
mod v6;
pub use v6::*;
mod v7;
pub use v7::*;

pub use super::csgk::CSGK;

pub type Error = binrw::Error;

#[inline]
pub fn read_versioned<R: binrw::BinReaderExt>(mut read: R) -> Result<UnionPhysics, Error> {
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum UnionPhysics {
	CSGK(CSGK),
	Block(Block),
	V3(CSGPHS3),
	V5(CSGPHS5),
	V6(CSGPHS6),
	V7(CSGPHS7),
}
