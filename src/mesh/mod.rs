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

use binrw::BinReaderExt;

pub const DEFAULT_VERTEX_TANGENT: [i8; 4] = [0, 0, -128, 127];

pub type Error = binrw::Error;

#[binrw::binread]
#[br(little)]
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

pub fn read_versioned<R: BinReaderExt>(mut read: R) -> Result<Mesh, Error> {
	read.read_le()
}
