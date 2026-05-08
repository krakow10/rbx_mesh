use binrw::BinWriterExt;

use crate::union_physics::{Error, UnionPhysics};

fn round_trip(data: &[u8]) -> Result<UnionPhysics, Error> {
	let data_len = data.len();
	let mut rbuf = binrw::io::Cursor::new(data.to_owned());
	let mut wbuf = binrw::io::Cursor::new(Vec::new());
	//read and then write mesh
	let union_physics = crate::read_union_physics_versioned(&mut rbuf)?;
	assert_eq!(rbuf.position(), data_len as u64);
	wbuf.write_le(&union_physics).unwrap();
	assert_eq!(rbuf, wbuf);
	Ok(union_physics)
}

#[test]
fn csgphs_3() {
	round_trip(include_bytes!("../../meshes/CSGPHS_3.data")).unwrap();
}
#[test]
fn csgphs_5() {
	round_trip(include_bytes!("../../meshes/CSGPHS_5.data")).unwrap();
}
#[test]
fn csgphs_7() {
	round_trip(include_bytes!("../../meshes/CSGPHS_7.data")).unwrap();
}
#[test]
fn csgk() {
	round_trip(include_bytes!("../../meshes/CSGK.data")).unwrap();
}
