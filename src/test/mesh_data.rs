use crate::mesh_data::{Error,CSGPHS};

fn get_version(mesh_data:&CSGPHS)->&str{
	match mesh_data{
		CSGPHS::CSGK(_)=>"CSGK",
		CSGPHS::CSGPHS2(_)=>"CSGPHS2",
		CSGPHS::CSGPHS4(_)=>"CSGPHS4",
	}
}

fn read_mesh_data(data:&[u8])->Result<CSGPHS,Error>{
	let mut cursor=std::io::Cursor::new(data);
	let mesh_data=crate::read_mesh_data_versioned(&mut cursor)?;
	assert_eq!(cursor.position(),data.len() as u64);
	Ok(mesh_data)
}
fn dbg_mesh_data(mesh_data:CSGPHS,expected_version:&str){
	assert_eq!(get_version(&mesh_data),expected_version);
	//println!("header.version={:?}",mesh_data.header.version);
	// println!("header.hash={:?}",mesh_data.header.hash);
	// println!("header._unknown={:?}",mesh_data.header._unknown);
	// for (i,mesh) in mesh_data.vertices.into_iter().enumerate(){
	// 	println!("===VERTEX NUMBER {i}===");
	// 	println!("pos={:?}",mesh.pos);
	// 	println!("norm={:?}",mesh.norm);
	// 	println!("normal_id={:?}",mesh.normal_id);
	// 	println!("tex={:?}",mesh.tex);
	// 	println!("tangent={:?}",mesh.tangent);
	// }
	match mesh_data{
		CSGPHS::CSGK(_)=>(),
		CSGPHS::CSGPHS2(_)=>(),
		CSGPHS::CSGPHS4(mesh_data4)=>{
			println!("==V4");
			println!("_unknown1={:?}",mesh_data4._unknown1_count);
			for (i,thing) in mesh_data4._unknown1_list.into_iter().enumerate(){
				println!("u6 row={i} list={thing:?}");
			}
		},
}
}
#[test]
fn meshdata_385416572_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/385416572.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGPHS2");
	// unknown = [179, 166, 219, 60, 135, 12, 62, 153, 36, 94, 13, 28, 6, 183, 71, 222]
}
#[test]
fn meshdata_394453730_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/394453730.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGPHS2");
	// unknown = [44, 128, 126, 197, 153, 213, 233, 128, 178, 234, 201, 204, 83, 191, 103, 214]
}
#[test]
fn meshdata_5692112940_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/5692112940_2.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGPHS2");
}
#[test]
fn meshdata_4500696697_4(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/4500696697_4.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGPHS4");
}
