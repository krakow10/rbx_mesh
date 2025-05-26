fn read_physics_data(data:&[u8]){
	let mut cursor=std::io::Cursor::new(data);
	crate::read_physics_data_versioned(&mut cursor).unwrap();
	assert_eq!(cursor.position(),data.len() as u64);
}

#[test]
fn csgphs_3(){
	read_physics_data(include_bytes!("../../meshes/CSGPHS_3.data"));
}
#[test]
fn csgphs_5(){
	read_physics_data(include_bytes!("../../meshes/CSGPHS_5.data"));
}
#[test]
fn csgphs_7() {
	read_physics_data(include_bytes!("../../meshes/CSGPHS_7.data"));
}
#[test]
fn csgk(){
	read_physics_data(include_bytes!("../../meshes/CSGK.data"));
}
