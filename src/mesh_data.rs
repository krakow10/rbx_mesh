use std::io::{Read,Seek};

pub const OBFUSCATION_NOISE_CYCLE_XOR:[u8;31]=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
fn reversible_obfuscate(offset:u64,buf:&mut [u8]){
	const LEN:u64=OBFUSCATION_NOISE_CYCLE_XOR.len() as u64;
	for (i,b) in buf.iter_mut().enumerate(){
		*b^=OBFUSCATION_NOISE_CYCLE_XOR[((offset+i as u64)%LEN) as usize];
	}
}

pub struct Obfuscator<R:Read+Seek>{
	inner:R,
}
impl<R:Read+Seek> Obfuscator<R>{
	pub fn new(read:R)->Self{
		Self{inner:read}
	}
}
impl<R:Read+Seek> Read for Obfuscator<R>{
	fn read(&mut self,buf:&mut [u8])->std::io::Result<usize>{
		let pos=self.inner.stream_position()?;
		let read_amount=self.inner.read(buf)?;
		reversible_obfuscate(pos,&mut buf[..read_amount]);
		Ok(read_amount)
	}
}
impl<R:Read+Seek> Seek for Obfuscator<R>{
	fn seek(&mut self,pos:std::io::SeekFrom)->std::io::Result<u64>{
		self.inner.seek(pos)
	}
}

pub type Error=binrw::Error;

/// Deobfuscated
pub struct CSGMDL<R>{
	inner:R,
}
impl<R:binrw::BinReaderExt> CSGMDL<R>{
	/// MeshData is obfuscated by a cyclic noise pattern
	pub fn new_obfuscated(read:R)->CSGMDL<Obfuscator<R>>{
		CSGMDL{inner:Obfuscator::new(read)}
	}
	pub fn new_already_deobfuscated(read:R)->Self{
		Self{inner:read}
	}
}

#[inline]
pub fn read_deobfuscated<R:binrw::BinReaderExt>(mut read:CSGMDL<R>)->Result<MeshData,Error>{
	read.inner.read_le()
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
	pub normal_id:u32,//for textures!
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
