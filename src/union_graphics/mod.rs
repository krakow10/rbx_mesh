mod v2;
pub use v2::*;
mod v4;
pub use v4::*;
mod v5;
pub use v5::*;

use binrw::BinReaderExt;
use std::io::{Read, Seek, Write};

pub const OBFUSCATION_NOISE_CYCLE_XOR: [u8; 31] = [
	86, 46, 110, 88, 49, 32, 48, 4, 52, 105, 12, 119, 12, 1, 94, 0, 26, 96, 55, 105, 29, 82, 43, 7,
	79, 36, 89, 101, 83, 4, 122,
];
fn reversible_obfuscate(offset: u64, buf: &mut [u8]) {
	const LEN: u64 = OBFUSCATION_NOISE_CYCLE_XOR.len() as u64;
	for (i, b) in buf.iter_mut().enumerate() {
		*b ^= OBFUSCATION_NOISE_CYCLE_XOR[((offset + i as u64) % LEN) as usize];
	}
}

pub struct Obfuscator<R> {
	inner: R,
}
impl<R> Obfuscator<R> {
	pub fn new(read: R) -> Self {
		Self { inner: read }
	}
}
impl<R: Read + Seek> Read for Obfuscator<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let pos = self.inner.stream_position()?;
		let read_amount = self.inner.read(buf)?;
		reversible_obfuscate(pos, &mut buf[..read_amount]);
		Ok(read_amount)
	}
}
impl<R: Write + Seek> Write for Obfuscator<R> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		// avoiding allocation in Read was fortunate, but not possible here
		let mut copy = buf.to_owned();
		let pos = self.inner.stream_position()?;
		reversible_obfuscate(pos, &mut copy);
		self.inner.write(&copy)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}
impl<R: Seek> Seek for Obfuscator<R> {
	fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
		self.inner.seek(pos)
	}
}

pub type Error = binrw::Error;

#[inline]
pub fn read_versioned<R: BinReaderExt>(mut read: R) -> Result<UnionGraphics, Error> {
	read.read_le()
}
#[inline]
pub fn read_header<R: BinReaderExt>(mut read: R) -> Result<Header, Error> {
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
// #[brw(magic=b"CSGMDL")]
#[brw(magic = b"\x15\x7d\x29\x15\x75\x6c")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HeaderVersion {
	// #[brw(magic=2u32)]
	#[brw(magic = b"\x32\x04\x34\x69")]
	CSGMDL2,
	// #[brw(magic=4u32)]
	#[brw(magic = b"\x34\x04\x34\x69")]
	CSGMDL4,
	// #[brw(magic=5u32)]
	#[brw(magic = b"\x35\x04\x34\x69")]
	CSGMDL5,
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
#[derive(Debug, Clone)]
pub enum Header {
	CSGK(CSGK),
	CSGMDL(HeaderVersion),
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

#[derive(Debug, Clone)]
pub enum UnionGraphics {
	CSGK(CSGK),
	CSGMDL(CSGMDL),
}
impl binrw::BinRead for UnionGraphics {
	type Args<'a> = ();
	fn read_options<R: Read + Seek>(
		reader: &mut R,
		endian: binrw::Endian,
		args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		let header = Header::read_options(reader, endian, args)?;
		Ok(match header {
			Header::CSGK(csgk) => UnionGraphics::CSGK(csgk),
			Header::CSGMDL(header_version) => {
				reader.seek(std::io::SeekFrom::Start(0))?;
				match header_version {
					HeaderVersion::CSGMDL2 => UnionGraphics::CSGMDL(CSGMDL::V2(
						CSGMDL2::read_options(&mut Obfuscator::new(reader), endian, args)?,
					)),
					HeaderVersion::CSGMDL4 => UnionGraphics::CSGMDL(CSGMDL::V4(
						CSGMDL4::read_options(&mut Obfuscator::new(reader), endian, args)?,
					)),
					// in version 5 only the header is obfuscated.
					HeaderVersion::CSGMDL5 => UnionGraphics::CSGMDL(CSGMDL::V5(
						CSGMDL5::read_options(reader, endian, args)?,
					)),
				}
			}
		})
	}
}

#[derive(Debug)]
pub enum WriteMeshDataError {
	CSGMDL5,
}
impl std::fmt::Display for WriteMeshDataError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CSGMDL5 => write!(f, "Writing CSGMDL5 is not supported"),
		}
	}
}
impl core::error::Error for WriteMeshDataError {}
impl binrw::BinWrite for UnionGraphics {
	type Args<'a> = ();
	fn write_options<W: Write + Seek>(
		&self,
		writer: &mut W,
		endian: binrw::Endian,
		args: Self::Args<'_>,
	) -> binrw::BinResult<()> {
		match self {
			UnionGraphics::CSGK(csgk) => csgk.write_options(writer, endian, args),
			UnionGraphics::CSGMDL(CSGMDL::V2(mesh_data2)) => {
				mesh_data2.write_options(&mut Obfuscator::new(writer), endian, args)
			}
			UnionGraphics::CSGMDL(CSGMDL::V4(mesh_data4)) => {
				mesh_data4.write_options(&mut Obfuscator::new(writer), endian, args)
			}
			UnionGraphics::CSGMDL(CSGMDL::V5(_mesh_data5)) => {
				//mesh_data5.write_options(writer,endian,args),
				Err(Error::Custom {
					pos: 0,
					err: Box::new(WriteMeshDataError::CSGMDL5),
				})
			}
		}
	}
}
