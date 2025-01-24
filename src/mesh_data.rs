pub type Error=binrw::Error;

// use std::io::{Read,Seek};
//
// struct Decoder<R:Read+Seek>{
// 	buf:Vec<u8>,
// 	pos:u64,
// 	inner:R,
// }
// impl<R:Read+Seek> Read for Decoder<R>{
// 	fn read(&mut self,buf:&mut [u8])->std::io::Result<usize>{
// 		self.inner.read(&mut self.buf);
// 		self.buf.iter_mut()
// 	}
// }

fn decode<R:binrw::BinReaderExt>(mut read:R)->Result<Vec<u8>,Error>{
	const NOISE:[u8;31]=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
	let mut buf=Vec::new();
	read.read_to_end(&mut buf).map_err(Error::Io)?;
	for (i,b) in buf.iter_mut().enumerate(){
		*b^=NOISE[i%NOISE.len()];
	}
	Ok(buf)
}

#[inline]
pub fn read<R:binrw::BinReaderExt>(mut read:R)->Result<MeshData,Error>{
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Header{
	pub version:u32,//2
	pub hash:[u8;16],//784f216c8b49e5f6
	pub _unknown:[u8;16],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Vertex{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub color:[u8;4],
	pub mystery_number_up_to_6:u32,
	pub tex:[f32;2],
	#[brw(magic=0u128)]
	pub tangent:[f32;3],
	#[brw(magic=0u128)]
	pub _nothing:(),
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct VertexId(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct MeshData{
	#[brw(magic=b"CSGMDL")]
	pub header:Header,
	pub vertex_count:u32,
	// vertex data length
	#[brw(magic=84u32)]
	#[br(count=vertex_count)]
	pub vertices:Vec<Vertex>,
	pub face_count:u32,
	#[br(count=face_count)]
	pub faces:Vec<VertexId>,
}

#[test]
fn do_it(){
	let data=include_bytes!("../meshes/394453730.meshdata");
	let decoded=decode(std::io::Cursor::new(data)).unwrap();
	let mut cursor=std::io::Cursor::new(decoded);
	let mesh_data=read(&mut cursor).unwrap();
	println!("header._unknown={:?}",mesh_data.header._unknown);
	for (i,mesh) in mesh_data.vertices.into_iter().enumerate(){
		println!("===VERTEX NUMBER {i}===");
		println!("pos={:?}",mesh.pos);
		println!("norm={:?}",mesh.norm);
		println!("count={}",mesh.mystery_number_up_to_6);
		println!("tex={:?}",mesh.tex);
		println!("tangent={:?}",mesh.tangent);
	}
	assert_eq!(cursor.position(),data.len() as u64);
}
