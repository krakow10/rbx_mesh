fn read_union_physics(data: &[u8]) {
	let mut cursor = std::io::Cursor::new(data);
	crate::read_union_physics_versioned(&mut cursor).unwrap();
	assert_eq!(cursor.position(), data.len() as u64);
}

#[test]
fn csgphs_3() {
	read_union_physics(include_bytes!("../../meshes/CSGPHS_3.data"));
}
#[test]
fn csgphs_5() {
	read_union_physics(include_bytes!("../../meshes/CSGPHS_5.data"));
}
#[test]
fn csgphs_7() {
	read_union_physics(include_bytes!("../../meshes/CSGPHS_7.data"));
}
#[test]
fn csgk() {
	read_union_physics(include_bytes!("../../meshes/CSGK.data"));
}
