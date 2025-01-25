use std::io::{Read,Seek};
use binrw::BinReaderExt;

pub const OBFUSCATION_NOISE_CYCLE_XOR:[u8;31]=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
fn reversible_obfuscate(offset:u64,buf:&mut [u8]){
	const LEN:u64=OBFUSCATION_NOISE_CYCLE_XOR.len() as u64;
	for (i,b) in buf.iter_mut().enumerate(){
		*b^=OBFUSCATION_NOISE_CYCLE_XOR[((offset+i as u64)%LEN) as usize];
	}
}

pub struct Obfuscator<R>{
	inner:R,
}
impl<R> Obfuscator<R>{
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
impl<R:Seek> Seek for Obfuscator<R>{
	fn seek(&mut self,pos:std::io::SeekFrom)->std::io::Result<u64>{
		self.inner.seek(pos)
	}
}

pub type Error=binrw::Error;

#[inline]
pub fn read_versioned<R:BinReaderExt>(mut read:R)->Result<CSGPHS,Error>{
	let header:Header=read.read_le()?;
	Ok(match header{
		Header::CSGK(csgk)=>CSGPHS::CSGK(csgk),
		Header::CSGPHS(header_version)=>{
			read.seek(std::io::SeekFrom::Start(0))?;
			match header_version{
				HeaderVersion::CSGPHS2=>CSGPHS::CSGPHS2(Obfuscator::new(read).read_le()?),
				HeaderVersion::CSGPHS4=>CSGPHS::CSGPHS4(Obfuscator::new(read).read_le()?),
			}
		}
	})
}
#[inline]
pub fn read_header<R:BinReaderExt>(mut read:R)->Result<Header,Error>{
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone,Eq,PartialEq)]
pub enum HeaderVersion{
	// #[brw(magic=b"CSGMDL")] #[brw(magic=2u32)]
	#[brw(magic=b"\x15\x7d\x29\x15\x75\x6c\x32\x04\x34\x69")]
	CSGPHS2,
	// #[brw(magic=b"CSGMDL")] #[brw(magic=4u32)]
	#[brw(magic=b"\x15\x7d\x29\x15\x75\x6c\x34\x04\x34\x69")]
	CSGPHS4,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct CSGK{
	#[brw(magic=b"CSGK")]
	pub uuid_ascii_hex:[u8;32],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Header{
	CSGK(CSGK),
	CSGPHS(HeaderVersion),
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Hash{
	pub hash:[u8;16],//784f216c8b49e5f6
	pub _unknown:[u8;16],
}
#[binrw::binrw]
#[brw(little,repr=u32)]
#[derive(Debug,Clone)]
// Why does this differ from Roblox's own standard?
pub enum NormalId2{
	Right=1,
	Top=2,
	Back=3,
	Left=4,
	Bottom=5,
	Front=6,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Vertex{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub color:[u8;4],
	// NormalId is redundant and can simply be computed
	// from the normal axis with the largest magnitude.
	// Primarily used for textures.
	pub normal_id:NormalId2,
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
pub struct Mesh2{
	pub vertex_count:u32,
	// vertex data length
	#[brw(magic=84u32)]
	#[br(count=vertex_count)]
	pub vertices:Vec<Vertex>,
	pub face_count:u32,
	#[br(count=face_count)]
	pub faces:Vec<VertexId>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct MeshData2{
	#[brw(magic=b"CSGMDL\x02\0\0\0")]
	pub hash:Hash,
	pub mesh:Mesh2,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct MeshData4{
	#[brw(magic=b"CSGMDL\x04\0\0\0")]
	pub hash:Hash,
	pub mesh:Mesh2,
	pub _unknown1_count:u32,
	#[br(count=_unknown1_count)]
	pub _unknown1_list:Vec<u32>,
}
#[derive(Debug,Clone)]
pub enum CSGPHS{
	CSGK(CSGK),
	CSGPHS2(MeshData2),
	CSGPHS4(MeshData4),
}
