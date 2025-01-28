use crate::mesh_data::{Error,VersionedMesh,Version};

fn get_version(mesh_data:&VersionedMesh)->Version{
	match mesh_data{
		VersionedMesh::Version2(_)=>Version::Version2,
		VersionedMesh::Version4(_)=>Version::Version4,
		VersionedMesh::Version5(_)=>Version::Version5,
	}
}

fn read_mesh_data(data:&[u8])->Result<VersionedMesh,Error>{
	let mut cursor=std::io::Cursor::new(data);
	let mesh_data=crate::read_mesh_data_versioned(&mut cursor)?;
	assert_eq!(cursor.position(),data.len() as u64);
	Ok(mesh_data)
}
fn dbg_mesh_data(mesh_data:VersionedMesh,expected_version:Version){
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
		VersionedMesh::Version2(_)=>(),
		VersionedMesh::Version4(mesh_data4)=>{
			println!("==V4");
			println!("_unknown1={:?}",mesh_data4._unknown1_count);
			for (i,thing) in mesh_data4._unknown1_list.into_iter().enumerate(){
				println!("u6 row={i} list={thing:?}");
			}
		},
		VersionedMesh::Version5(mesh_data5)=>{
			println!("===V5===");
			println!("pos_count={}",mesh_data5.pos_count);
			println!("_unknown1_count={}",mesh_data5._unknown1_count);
			println!("_unknown1_len={}",mesh_data5._unknown1_len);
			println!("color_count={}",mesh_data5.color_count);
			println!("normal_id_count={}",mesh_data5.normal_id_count);
			println!("tex_count={}",mesh_data5.tex_count);
			println!("_unknown4_count={}",mesh_data5._unknown4_count);
			println!("_unknown5_count1={} bytes={:?}",mesh_data5._unknown5_count1,mesh_data5._unknown5_count1.to_le_bytes());
			println!("_unknown5_count2={} bytes={:?}",mesh_data5._unknown5_count2,mesh_data5._unknown5_count2.to_le_bytes());
			for (i,thing) in mesh_data5._unknown5_list.into_iter().enumerate().skip(mesh_data5._unknown5_count2 as usize-10){
				println!("u5 row={i} list={thing:?}");
			}
			println!("_unknown6={:?}",mesh_data5._unknown6_count);
			for (i,thing) in mesh_data5._unknown6_list.into_iter().enumerate(){
				println!("u6 row={i} list={thing:?}");
			}
			// println!("===REST===");
			// let len=mesh_data5.rest.len();
			// let e=len.min(64);
			// println!("len={len} {:?}",&mesh_data5.rest[..e]);
		},
}
}
#[test]
fn meshdata_385416572_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/385416572.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version2);
	// unknown = [179, 166, 219, 60, 135, 12, 62, 153, 36, 94, 13, 28, 6, 183, 71, 222]
}
#[test]
fn meshdata_394453730_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/394453730.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version2);
	// unknown = [44, 128, 126, 197, 153, 213, 233, 128, 178, 234, 201, 204, 83, 191, 103, 214]
}
#[test]
fn meshdata_5692112940_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/5692112940.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version2);
}
#[test]
fn meshdata_4500696697_4(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/4500696697_4.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version4);
}
#[test]
fn meshdata_15124417947_5(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/15124417947_5.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version5);
}
#[test]
fn meshdata_14846974687_5(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/14846974687_5.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,Version::Version5);
}
