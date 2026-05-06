// v1 based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs LoadGeometry_Ascii function
#[cfg(feature = "mesh-v1")]
mod v1;
#[cfg(feature = "mesh-v1")]
pub use v1::*;
// the rest are based on https://devforum.roblox.com/t/roblox-mesh-format/326114
mod v2;
pub use v2::*;
mod v3;
pub use v3::*;
mod v4;
pub use v4::*;
mod v5;
pub use v5::*;

use std::io::{Read, Seek};

pub const DEFAULT_VERTEX_TANGENT: [i8; 4] = [0, 0, -128, 127];

#[derive(Debug)]
pub enum Error {
	Io(std::io::Error),
	UnknownVersion([u8; 12]),
	//1.00
	#[cfg(feature = "mesh-v1")]
	V1(Error1),
	//2.00+
	BinRead(binrw::Error),
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub enum Mesh {
	#[cfg(feature = "mesh-v1")]
	V1(Mesh1),
	V2(Mesh2),
	V3(Mesh3),
	V4(Mesh4),
	V5(Mesh5),
	//Version6(Mesh6),
	//Version7(Mesh7),
}

pub fn read_versioned<R: Read + Seek>(mut read: R) -> Result<Mesh, Error> {
	let mut peek = [0u8; 12];
	read.read_exact(&mut peek).map_err(Error::Io)?;
	read.seek(std::io::SeekFrom::Start(0)).map_err(Error::Io)?;
	match &peek {
		#[cfg(feature = "mesh-v1")]
		b"version 1.00" => Ok(Mesh::V1(
			read_100(binrw::io::BufReader::new(read)).map_err(Error::V1)?,
		)),
		#[cfg(feature = "mesh-v1")]
		b"version 1.01" => Ok(Mesh::V1(
			read_101(binrw::io::BufReader::new(read)).map_err(Error::V1)?,
		)),
		b"version 2.00" => Ok(Mesh::V2(read_200(read).map_err(Error::BinRead)?)),
		b"version 3.00" | b"version 3.01" => Ok(Mesh::V3(read_300(read).map_err(Error::BinRead)?)),
		b"version 4.00" | b"version 4.01" => Ok(Mesh::V4(read_400(read).map_err(Error::BinRead)?)),
		b"version 5.00" => Ok(Mesh::V5(read_500(read).map_err(Error::BinRead)?)),
		//b"version 6.00"=>Ok(VersionedMesh::Version6(read_600(read)?)),
		//b"version 7.00"=>Ok(VersionedMesh::Version7(read_700(read)?)),
		&other => Err(Error::UnknownVersion(other)),
	}
}
