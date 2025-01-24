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
	pub version:u32,
	pub hash:[u8;16],
	pub _unknown:[u32;4],
	pub model_count:u32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Model{
	pub payload:[f32;6],
	#[brw(magic=b"\x5B\x5D\x69\xFF")]
	pub payload_count:u32,
	pub tex:[f32;2],
	#[brw(magic=0u128)]
	pub more_float:[f32;3],
	#[brw(magic=0u128)]
	_nothing:(),
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct ModelId(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Indices{
	pub count:u32,
	#[br(count=count)]
	pub indices:Vec<ModelId>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct MeshData{
	#[brw(magic=b"CSGMDL")]
	pub header:Header,
	// model data length
	#[brw(magic=84u32)]
	#[br(count=header.model_count)]
	pub models:Vec<Model>,
	pub indices:Indices,
}

#[test]
fn do_it(){
	let data=include_bytes!("../meshes/385416572.meshdata");
	let decoded=decode(std::io::Cursor::new(data)).unwrap();
	let mut cursor=std::io::Cursor::new(decoded);
	let mesh_data=read(&mut cursor).unwrap();
	for (i,mesh) in mesh_data.models.into_iter().enumerate(){
		println!("===MESH NUMBER {i}===");
		println!("payload={:?}",mesh.payload);
		println!("count={}",mesh.payload_count);
		println!("tex={:?}",mesh.tex);
		println!("more_float={:?}",mesh.more_float);
	}
	assert_eq!(cursor.position(),data.len() as u64);
}
