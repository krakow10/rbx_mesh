mod v2;
pub use v2::*;
mod v4;
pub use v4::*;
mod v5;
pub use v5::*;

mod obfuscate;

pub use super::csgk::CSGK;

use binrw::BinReaderExt;

pub type Error = binrw::Error;

#[inline]
pub fn read_versioned<R: BinReaderExt>(mut read: R) -> Result<UnionGraphics, Error> {
	read.read_le()
}

#[derive(Debug)]
pub struct NormalIDError;
impl std::fmt::Display for NormalIDError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl core::error::Error for NormalIDError {}

// Why does this differ from Roblox's own standard?
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum NormalId {
	Right = 1,
	Top = 2,
	Back = 3,
	Left = 4,
	Bottom = 5,
	Front = 6,
}

#[binrw::binread]
#[br(little)]
#[derive(Debug, Clone)]
pub enum CSGMDL {
	V2(CSGMDL2),
	V4(CSGMDL4),
	V5(CSGMDL5),
}

#[binrw::binread]
#[br(little)]
#[derive(Debug, Clone)]
pub enum UnionGraphics {
	CSGK(CSGK),
	CSGMDL(CSGMDL),
}
