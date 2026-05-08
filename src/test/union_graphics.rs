use crate::union_graphics::{Error, UnionGraphics, CSGMDL};

fn get_version(union_graphics: &UnionGraphics) -> &str {
	match union_graphics {
		UnionGraphics::CSGK(_) => "CSGK",
		UnionGraphics::CSGMDL(CSGMDL::V2(_)) => "CSGMDL2",
		UnionGraphics::CSGMDL(CSGMDL::V4(_)) => "CSGMDL4",
		UnionGraphics::CSGMDL(CSGMDL::V5(_)) => "CSGMDL5",
	}
}

fn read_union_graphics(data: &[u8]) -> Result<UnionGraphics, Error> {
	let mut cursor = std::io::Cursor::new(data);
	let union_graphics = crate::read_union_graphics_versioned(&mut cursor)?;
	assert_eq!(cursor.position(), data.len() as u64);
	Ok(union_graphics)
}
fn dbg_union_graphics(union_graphics: UnionGraphics, expected_version: &str) {
	assert_eq!(get_version(&union_graphics), expected_version);
	//println!("header.version={:?}",union_graphics.header.version);
	// println!("header.hash={:?}",union_graphics.header.hash);
	// println!("header._unknown={:?}",union_graphics.header._unknown);
	// for (i,mesh) in union_graphics.vertices.into_iter().enumerate(){
	// 	println!("===VERTEX NUMBER {i}===");
	// 	println!("pos={:?}",mesh.pos);
	// 	println!("norm={:?}",mesh.norm);
	// 	println!("normal_id={:?}",mesh.normal_id);
	// 	println!("tex={:?}",mesh.tex);
	// 	println!("tangent={:?}",mesh.tangent);
	// }
	match union_graphics {
		UnionGraphics::CSGK(_) => (),
		UnionGraphics::CSGMDL(CSGMDL::V2(_)) => (),
		UnionGraphics::CSGMDL(CSGMDL::V4(csgmdl4)) => {
			println!("==V4");
			println!("_unknown1={:?}", csgmdl4._unknown1_list.len());
			for (i, thing) in csgmdl4._unknown1_list.into_iter().enumerate() {
				println!("u6 row={i} list={thing:?}");
			}
		}
		UnionGraphics::CSGMDL(CSGMDL::V5(csgmdl5)) => {
			println!("===V5===");
			println!("{:?}", csgmdl5);
			println!("pos_count={}", csgmdl5.positions.len());
			println!("faces={:?}", csgmdl5.faces.indices);
			println!("extra={:?}", csgmdl5.faces._unknown);
		}
	}
}
#[test]
fn meshdata_385416572_2() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/385416572.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL2");
	// unknown = [179, 166, 219, 60, 135, 12, 62, 153, 36, 94, 13, 28, 6, 183, 71, 222]
}
#[test]
fn meshdata_394453730_2() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/394453730.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL2");
	// unknown = [44, 128, 126, 197, 153, 213, 233, 128, 178, 234, 201, 204, 83, 191, 103, 214]
}
#[test]
fn meshdata_5692112940_2() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/5692112940_2.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL2");
}
#[test]
fn meshdata_4500696697_4() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/4500696697_4.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL4");
}
#[test]
fn meshdata_15124417947_5() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/15124417947_5.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL5");
}
#[test]
fn meshdata_14846974687_5() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/14846974687_5.meshdata")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL5");
}
#[test]
fn meshdata_13626979828_5() {
	let union_graphics =
		read_union_graphics(include_bytes!("../../meshes/13626979828.meshdata5")).unwrap();
	dbg_union_graphics(union_graphics, "CSGMDL5");
}
