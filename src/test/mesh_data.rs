use crate::mesh_data::{Error,MeshData};

fn read_mesh_data(data:&[u8])->Result<MeshData,Error>{
	let mut cursor=std::io::Cursor::new(data);
	let mesh_data=crate::read_mesh_data(&mut cursor)?;
	assert_eq!(cursor.position(),data.len() as u64);
	Ok(mesh_data)
}
fn dbg_mesh_data(mesh_data:MeshData){
	println!("header.version={:?}",mesh_data.header.version);
	println!("header.hash={:?}",mesh_data.header.hash);
	println!("header._unknown={:?}",mesh_data.header._unknown);
	// for (i,mesh) in mesh_data.vertices.into_iter().enumerate(){
	// 	println!("===VERTEX NUMBER {i}===");
	// 	println!("pos={:?}",mesh.pos);
	// 	println!("norm={:?}",mesh.norm);
	// 	println!("normal_id={}",mesh.normal_id);
	// 	println!("tex={:?}",mesh.tex);
	// 	println!("tangent={:?}",mesh.tangent);
	// }
}
#[test]
fn meshdata_385416572(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/385416572.meshdata")).unwrap();
	dbg_mesh_data(mesh_data);
	// unknown = [179, 166, 219, 60, 135, 12, 62, 153, 36, 94, 13, 28, 6, 183, 71, 222]
}
#[test]
fn meshdata_394453730(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/394453730.meshdata")).unwrap();
	dbg_mesh_data(mesh_data);
	// unknown = [44, 128, 126, 197, 153, 213, 233, 128, 178, 234, 201, 204, 83, 191, 103, 214]
}
