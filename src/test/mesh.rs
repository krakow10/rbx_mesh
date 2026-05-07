use binrw::BinWriterExt;

use crate::mesh::*;
fn load_mesh(name: &str) -> Result<Mesh, Error> {
	read_versioned(std::fs::File::open(name).map_err(Error::Io)?)
}
fn get_mesh_id(mesh: Mesh) -> u16 {
	match mesh {
		#[cfg(feature = "mesh-v1")]
		Mesh::V1(mesh) => match mesh.header.revision {
			Revision1::Version100 => 100,
			Revision1::Version101 => 101,
		},
		Mesh::V2(mesh) => match mesh.header.revision {
			Revision2::Version200 => 200,
		},
		Mesh::V3(mesh) => match mesh.header.revision {
			Revision3::Version300 => 300,
			Revision3::Version301 => 301,
		},
		Mesh::V4(mesh) => match mesh.header.revision {
			Revision4::Version400 => 400,
			Revision4::Version401 => 401,
		},
		Mesh::V5(mesh) => match mesh.header.revision {
			Revision5::Version500 => 500,
		},
	}
}
//Mesh1 has no round trip since there is no writer
fn round_trip(name: &str) {
	let mut rbuf = binrw::io::Cursor::new(std::fs::read(name).unwrap());
	let mut wbuf = binrw::io::Cursor::new(Vec::new());
	//read and then write mesh
	let mesh: Mesh = read_versioned(&mut rbuf).unwrap();
	match mesh {
		Mesh::V1(_mesh) => panic!("Cannot round trip Mesh v1"),
		Mesh::V2(mesh) => wbuf.write_le(&mesh).unwrap(),
		Mesh::V3(mesh) => wbuf.write_le(&mesh).unwrap(),
		Mesh::V4(mesh) => wbuf.write_le(&mesh).unwrap(),
		Mesh::V5(mesh) => wbuf.write_le(&mesh).unwrap(),
	}
	assert_eq!(rbuf, wbuf);
}

#[cfg(feature = "mesh-v1")]
#[test]
fn mesh_100() {
	assert_eq!(get_mesh_id(load_mesh("meshes/158071912").unwrap()), 100);
}
#[test]
fn mesh_200() {
	assert_eq!(get_mesh_id(load_mesh("meshes/torso.mesh").unwrap()), 200)
}
#[test]
fn roundtrip_200() {
	round_trip("meshes/torso.mesh");
}
#[test]
fn mesh_300() {
	assert_eq!(get_mesh_id(load_mesh("meshes/5115672913").unwrap()), 300);
}
#[test]
fn roundtrip_300() {
	round_trip("meshes/5115672913");
}
#[test]
fn mesh_301() {
	assert_eq!(get_mesh_id(load_mesh("meshes/5648093777").unwrap()), 301)
}
#[test]
fn roundtrip_301() {
	round_trip("meshes/5648093777");
}
#[test]
fn mesh_401() {
	assert_eq!(get_mesh_id(load_mesh("meshes/sphere.mesh").unwrap()), 401)
}
#[test]
fn roundtrip_401() {
	round_trip("meshes/sphere.mesh");
}
#[test]
fn mesh_401_random_padding() {
	assert_eq!(get_mesh_id(load_mesh("meshes/7665777615").unwrap()), 401)
}
#[test]
fn roundtrip_401_random_padding() {
	round_trip("meshes/7665777615");
}
//the only three v5.00 meshes I could find in bhop and surf
#[test]
fn mesh_500() {
	assert_eq!(get_mesh_id(load_mesh("meshes/13674780763").unwrap()), 500)
}
#[test]
fn roundtrip_500() {
	round_trip("meshes/13674780763");
}
#[test]
fn mesh_500_alt1() {
	assert_eq!(get_mesh_id(load_mesh("meshes/14818281896").unwrap()), 500)
}
#[test]
fn roundtrip_500_alt1() {
	round_trip("meshes/14818281896");
}
#[test]
fn mesh_500_alt2() {
	assert_eq!(get_mesh_id(load_mesh("meshes/15256456161").unwrap()), 500)
}
#[test]
fn roundtrip_500_alt2() {
	round_trip("meshes/15256456161");
}
//also tested against ~2500 meshes from bhop and surf maps
