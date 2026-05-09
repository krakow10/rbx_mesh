use super::roundtrip;
use crate::union_physics::{CSGK, CSGPHS3, CSGPHS5, CSGPHS7};
use std::fs::read;

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
#[test]
fn csgk() {
	let bytes = read("meshes/CSGK.data").unwrap();
	let mesh = roundtrip::<CSGK>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);
}
