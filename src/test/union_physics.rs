use super::{readonly, roundtrip};
use crate::union_physics::{CSGK, CSGPHS3, CSGPHS5, CSGPHS7, CSGPHS8};
use std::fs::read;

#[test]
fn csgk() {
	let bytes = read("meshes/CSGK.data").unwrap();
	let mesh = roundtrip::<CSGK>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn csgphs_3() {
	let bytes = read("meshes/CSGPHS_3.data").unwrap();
	let mesh = roundtrip::<CSGPHS3>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn csgphs_5() {
	let bytes = read("meshes/CSGPHS_5.data").unwrap();
	let mesh = roundtrip::<CSGPHS5>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);
}
#[test]
fn csgphs_7() {
	let bytes = read("meshes/CSGPHS_7.data").unwrap();
	let mesh = roundtrip::<CSGPHS7>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);
}
#[cfg(feature = "csgphs-v8")]
#[test]
fn csgphs_8() {
	let bytes = read("meshes/CSGPHS_8_00.data").unwrap();
	let mesh = readonly::<CSGPHS8>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);

	let hulls = mesh.mesh.hulls();
	insta::assert_debug_snapshot!(hulls);
}
