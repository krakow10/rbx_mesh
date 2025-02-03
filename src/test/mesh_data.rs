use crate::mesh_data::{Error,MeshData,CSGMDL};

fn get_version(mesh_data:&MeshData)->&str{
	match mesh_data{
		MeshData::CSGK(_)=>"CSGK",
		MeshData::CSGMDL(CSGMDL::V2(_))=>"CSGMDL2",
		MeshData::CSGMDL(CSGMDL::V4(_))=>"CSGMDL4",
		MeshData::CSGMDL(CSGMDL::V5(_))=>"CSGMDL5",
	}
}

fn read_mesh_data(data:&[u8])->Result<MeshData,Error>{
	let mut cursor=std::io::Cursor::new(data);
	let mesh_data=crate::read_mesh_data_versioned(&mut cursor)?;
	assert_eq!(cursor.position(),data.len() as u64);
	Ok(mesh_data)
}
fn dbg_mesh_data(mesh_data:MeshData,expected_version:&str){
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
		MeshData::CSGK(_)=>(),
		MeshData::CSGMDL(CSGMDL::V2(_))=>(),
		MeshData::CSGMDL(CSGMDL::V4(mesh_data4))=>{
			println!("==V4");
			println!("_unknown1={:?}",mesh_data4._unknown1_count);
			for (i,thing) in mesh_data4._unknown1_list.into_iter().enumerate(){
				println!("u6 row={i} list={thing:?}");
			}
		},
		MeshData::CSGMDL(CSGMDL::V5(mesh_data5))=>{
			println!("===V5===");
			println!("pos_count={}",mesh_data5.pos_count);
			println!("norm_count={}",mesh_data5.norm_count);
			println!("norm_len={}",mesh_data5.norm_len);
			for (i,thing) in mesh_data5.norm_list.into_iter().enumerate(){
				print!("u1 row={i:03} bin=");
				for byte in thing{
					print!("{byte:016b} ");
				}
				println!("list={thing:?}");
			}
			// println!("color_count={}",mesh_data5.color_count);
			// println!("normal_id_count={}",mesh_data5.normal_id_count);
			// println!("tex_count={}",mesh_data5.tex_count);
			// println!("_unknown4_count={}",mesh_data5._unknown4_count);
			// println!("_unknown4_len={}",mesh_data5._unknown4_len);
			// for (i,thing) in mesh_data5._unknown4_list.into_iter().enumerate(){
			// 	println!("u4 row={i} list={thing:?}");
			// }
			// println!("_unknown5_count1={} bytes={:?}",mesh_data5._unknown5_count1,mesh_data5._unknown5_count1.to_le_bytes());
			// println!("_unknown5_count2={} bytes={:?}",mesh_data5._unknown5_count2,mesh_data5._unknown5_count2.to_le_bytes());
			// let mut accumulate=0i32;
			// for (i,thing) in mesh_data5._unknown5_list.into_iter().enumerate(){//}.skip(mesh_data5._unknown5_count2 as usize-32){
			// 	accumulate=(accumulate+thing as i8 as i32).rem_euclid(mesh_data5.pos_count as i32);
			// 	println!("u5 row={i} accumulated={accumulate:?}");
			// }
			// println!("_unknown6={:?}",mesh_data5._unknown6_count);
			// for (i,thing) in mesh_data5._unknown6_list.into_iter().enumerate(){
			// 	println!("u6 row={i} list={thing:?}");
			// }
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
	dbg_mesh_data(mesh_data,"CSGMDL2");
	// unknown = [179, 166, 219, 60, 135, 12, 62, 153, 36, 94, 13, 28, 6, 183, 71, 222]
}
#[test]
fn meshdata_394453730_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/394453730.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGMDL2");
	// unknown = [44, 128, 126, 197, 153, 213, 233, 128, 178, 234, 201, 204, 83, 191, 103, 214]
}
#[test]
fn meshdata_5692112940_2(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/5692112940_2.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGMDL2");
}
#[test]
fn meshdata_4500696697_4(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/4500696697_4.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGMDL4");
}
#[test]
fn meshdata_15124417947_5(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/15124417947_5.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGMDL5");
}
#[test]
fn meshdata_14846974687_5(){
	let mesh_data=read_mesh_data(include_bytes!("../../meshes/14846974687_5.meshdata")).unwrap();
	dbg_mesh_data(mesh_data,"CSGMDL5");
}
