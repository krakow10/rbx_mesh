use super::{readonly, roundtrip};
use crate::union_graphics::{CSGMDL2, CSGMDL4, CSGMDL5};
use std::fs::read;

#[test]
fn meshdata_385416572_2() {
	let bytes = read("meshes/385416572.meshdata").unwrap();
	let mesh = readonly::<CSGMDL2>(bytes).unwrap();
}
#[test]
fn meshdata_394453730_2() {
	let bytes = read("meshes/394453730.meshdata").unwrap();
	let mesh = roundtrip::<CSGMDL2>(bytes).unwrap();
}
#[test]
fn meshdata_5692112940_2() {
	let bytes = read("meshes/5692112940_2.meshdata").unwrap();
	let mesh = roundtrip::<CSGMDL2>(bytes).unwrap();
}
#[test]
fn meshdata_4500696697_4() {
	let bytes = read("meshes/4500696697_4.meshdata").unwrap();
	let mesh = roundtrip::<CSGMDL4>(bytes).unwrap();
}
#[test]
fn meshdata_15124417947_5() {
	let bytes = read("meshes/15124417947_5.meshdata").unwrap();
	let mesh = readonly::<CSGMDL5>(bytes).unwrap();
}
#[test]
fn meshdata_14846974687_5() {
	let bytes = read("meshes/14846974687_5.meshdata").unwrap();
	let mesh = readonly::<CSGMDL5>(bytes).unwrap();
}
#[test]
fn meshdata_13626979828_5() {
	let bytes = read("meshes/13626979828.meshdata5").unwrap();
	let mesh = readonly::<CSGMDL5>(bytes).unwrap();
}
