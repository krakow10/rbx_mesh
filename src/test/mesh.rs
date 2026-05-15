use super::{readonly, roundtrip};
use crate::mesh::{Mesh2, Mesh3, Mesh4, Mesh5, Revision2, Revision3, Revision4, Revision5};
use std::fs::read;

#[cfg(feature = "mesh-v1")]
#[test]
fn mesh_100() {
	use crate::mesh::{Mesh1, Revision1};
	let bytes = read("meshes/158071912").unwrap();
	let mesh = readonly::<Mesh1>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision1::Version100));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_200() {
	let bytes = read("meshes/torso.mesh").unwrap();
	let mesh = roundtrip::<Mesh2>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision2::Version200));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_300() {
	let bytes = read("meshes/5115672913").unwrap();
	let mesh = roundtrip::<Mesh3>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision3::Version300));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_301() {
	let bytes = read("meshes/5648093777").unwrap();
	let mesh = roundtrip::<Mesh3>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision3::Version301));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_401() {
	let bytes = read("meshes/sphere.mesh").unwrap();
	let mesh = roundtrip::<Mesh4>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision4::Version401));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_401_random_padding() {
	let bytes = read("meshes/7665777615").unwrap();
	let mesh = roundtrip::<Mesh4>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision4::Version401));
	insta::assert_debug_snapshot!(mesh);
}
//the only three v5.00 meshes I could find in bhop and surf
#[test]
fn mesh_500() {
	let bytes = read("meshes/13674780763").unwrap();
	let mesh = roundtrip::<Mesh5>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision5::Version500));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_500_alt1() {
	let bytes = read("meshes/14818281896").unwrap();
	let mesh = roundtrip::<Mesh5>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision5::Version500));
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn mesh_500_alt2() {
	let bytes = read("meshes/15256456161").unwrap();
	let mesh = roundtrip::<Mesh5>(bytes).unwrap();
	assert!(matches!(mesh.revision, Revision5::Version500));
	insta::assert_debug_snapshot!(mesh);
}
//also tested against ~2500 meshes from bhop and surf maps
