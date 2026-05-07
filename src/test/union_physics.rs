use crate::union_physics::UnionPhysics;

fn read_union_physics(data: &[u8]) -> UnionPhysics {
	let mut cursor = std::io::Cursor::new(data);
	let mesh = crate::read_union_physics_versioned(&mut cursor).unwrap();
	assert_eq!(cursor.position(), data.len() as u64);
	mesh
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
fn csgphs_8() {
	fn d(mesh: UnionPhysics) {
		if let UnionPhysics::CSGPHS(crate::union_physics::CSGPHS::V8(mesh)) = mesh {
			let hulls = crate::union_physics::decode_edgebreaker_hulls(
				&mesh.body.clers_buffer,
				mesh.body.clers_bit_count,
				mesh.body.hull_count,
				&mesh.body.vertices,
				mesh.body.total_faces,
			)
			.unwrap();
			println!("hulls.len() = {}", hulls.len());
		}
	}
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_00.data"
	)));
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_01.data"
	)));
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_02.data"
	)));
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_03.data"
	)));
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_04.data"
	)));
	d(read_union_physics(include_bytes!(
		"../../meshes/CSGPHS_8_05.data"
	)));
}
#[test]
fn csgk() {
	read_union_physics(include_bytes!("../../meshes/CSGK.data"));
}
