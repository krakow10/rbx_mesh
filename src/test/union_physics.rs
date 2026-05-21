use super::roundtrip;
use crate::union_physics::{CSGK, CSGPHS3, CSGPHS5, CSGPHS7};
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
#[cfg(any(feature = "csgphs-v8-zstd", feature = "csgphs-v8-ruzstd"))]
#[test]
fn csgphs_8() {
	use crate::union_physics::CSGPHS8;
	let bytes = read("meshes/CSGPHS_8_00.data").unwrap();
	let mesh = super::readonly::<CSGPHS8>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);

	let mut symbols = Vec::new();
	let mut symbol_reader = mesh.mesh.symbol_reader().unwrap();
	while let Ok(symbol) = symbol_reader.read() {
		symbols.push(symbol);
	}
	insta::assert_debug_snapshot!(symbols);

	#[expect(dead_code)]
	#[derive(Debug)]
	struct Hull {
		positions: Vec<[f32; 3]>,
		faces: Vec<[u32; 3]>,
	}

	let mut hull_decoder = mesh.mesh.hull_decoder().unwrap();
	let hulls: Vec<_> = (0..mesh.mesh.hull_count)
		.map(|_| {
			let hull = hull_decoder.decode_hull().unwrap();
			Hull {
				positions: hull.positions.to_vec(),
				faces: hull.faces.to_vec(),
			}
		})
		.collect();
	insta::assert_debug_snapshot!(hulls);
}
#[cfg(any(feature = "csgphs-v8-zstd", feature = "csgphs-v8-ruzstd"))]
#[test]
fn csgphs_8_raw_hull_1() {
	use crate::union_physics::CSGPHS8;
	let bytes = read("meshes/CSGPHS_8_raw_hulls_206.data").unwrap();
	let mesh = super::readonly::<CSGPHS8>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);

	let hulls: Vec<_> = mesh.mesh.raw_hulls.iter_hulls().collect();
	insta::assert_debug_snapshot!(hulls);
}
#[cfg(any(feature = "csgphs-v8-zstd", feature = "csgphs-v8-ruzstd"))]
#[test]
fn csgphs_8_raw_hull_2() {
	use crate::union_physics::CSGPHS8;
	let bytes = read("meshes/CSGPHS_8_raw_hulls_972.data").unwrap();
	let mesh = super::readonly::<CSGPHS8>(bytes).unwrap();
	insta::assert_debug_snapshot!(mesh);

	let hulls: Vec<_> = mesh.mesh.raw_hulls.iter_hulls().collect();
	insta::assert_debug_snapshot!(hulls);
}
