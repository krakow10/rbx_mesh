#[test]
fn meshdata(){
	let data=include_bytes!("../../meshes/394453730.meshdata");
	let mut cursor=std::io::Cursor::new(data);
	let mesh_data=crate::mesh_data::read(&mut cursor).unwrap();
	println!("header._unknown={:?}",mesh_data.header._unknown);
	for (i,mesh) in mesh_data.vertices.into_iter().enumerate(){
		println!("===VERTEX NUMBER {i}===");
		println!("pos={:?}",mesh.pos);
		println!("norm={:?}",mesh.norm);
		println!("normal_id={}",mesh.normal_id);
		println!("tex={:?}",mesh.tex);
		println!("tangent={:?}",mesh.tangent);
	}
	assert_eq!(cursor.position(),data.len() as u64);
}
