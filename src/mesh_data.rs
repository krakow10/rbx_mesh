use std::io::{Read,Seek,Write};
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
impl<R:Write+Seek> Write for Obfuscator<R>{
	fn write(&mut self,buf:&[u8])->std::io::Result<usize>{
		// avoiding allocation in Read was fortunate, but not possible here
		let mut copy=buf.to_owned();
		let pos=self.inner.stream_position()?;
		reversible_obfuscate(pos,&mut copy);
		self.inner.write(&copy)
	}
	fn flush(&mut self)->std::io::Result<()>{
		self.inner.flush()
	}
}
impl<R:Seek> Seek for Obfuscator<R>{
	fn seek(&mut self,pos:std::io::SeekFrom)->std::io::Result<u64>{
		self.inner.seek(pos)
	}
}

pub type Error=binrw::Error;

#[inline]
pub fn read_versioned<R:BinReaderExt>(mut read:R)->Result<MeshData,Error>{
	read.read_le()
}
#[inline]
pub fn read_header<R:BinReaderExt>(mut read:R)->Result<Header,Error>{
	read.read_le()
}

#[binrw::binrw]
#[brw(little)]
// #[brw(magic=b"CSGMDL")]
#[brw(magic=b"\x15\x7d\x29\x15\x75\x6c")]
#[derive(Debug,Clone,Eq,PartialEq)]
pub enum HeaderVersion{
	// #[brw(magic=2u32)]
	#[brw(magic=b"\x32\x04\x34\x69")]
	CSGMDL2,
	// #[brw(magic=4u32)]
	#[brw(magic=b"\x34\x04\x34\x69")]
	CSGMDL4,
	// #[brw(magic=5u32)]
	#[brw(magic=b"\x35\x04\x34\x69")]
	CSGMDL5,
}
#[binrw::binrw]
#[brw(little)]
#[brw(magic=b"CSGK")]
#[derive(Debug,Clone)]
pub struct CSGK{
	pub uuid_ascii_hex:[u8;32],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Header{
	CSGK(CSGK),
	CSGMDL(HeaderVersion),
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
#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
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
	// This field does not exist in the final struct and
	// exists purely to de/serialize the magic number.
	#[br(temp)]
	#[bw(ignore)]
	#[brw(magic=0u128)]
	_magic:(),
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
	#[br(count=face_count/3)]
	pub faces:Vec<[VertexId;3]>,
}
#[binrw::binrw]
#[brw(little)]
#[brw(magic=b"CSGMDL")]
#[derive(Debug,Clone)]
pub struct CSGMDL2{
	#[brw(magic=2u32)]
	pub hash:Hash,
	pub mesh:Mesh2,
}
#[binrw::binrw]
#[brw(little)]
#[brw(magic=b"CSGMDL")]
#[derive(Debug,Clone)]
pub struct CSGMDL4{
	#[brw(magic=4u32)]
	pub hash:Hash,
	pub mesh:Mesh2,
	pub _unknown1_count:u32,
	#[br(count=_unknown1_count)]
	pub _unknown1_list:Vec<u32>,
}
// TODO use read_options to directly read MeshData
// instead of reading header and then seeking back
#[binrw::binrw]
#[brw(little,repr=u8)]
#[derive(Debug,Clone)]
// Why does this differ from Roblox's own standard?
pub enum NormalId5{
	Right=1,
	Top=2,
	Back=3,
	Left=4,
	Bottom=5,
	Front=6,
}
pub struct VertexIndicesIterator<'a>{
	offset:usize,
	bytes:&'a [u8],
}
impl<'a> VertexIndicesIterator<'a>{
	fn new(bytes:&'a [u8])->Self{
		Self{
			offset:0,
			bytes,
		}
	}
}
impl Iterator for VertexIndicesIterator<'_>{
	type Item=usize;
	fn next(&mut self)->Option<Self::Item>{
		match self.bytes{
			&[v0,ref rest@..]=>{
				if v0&0x80==0{
					self.offset+=v0 as usize;
					self.bytes=rest;
					Some(self.offset)
				}else{
					// there are 3 numbers
					match rest{
						&[v1,v2,ref rest@..]=>{
							let offset=u32::from_le_bytes([v2,v1,v0&0x7F,0]);
							println!("yoo {offset:b}");
							self.offset+=offset as usize;
							self.bytes=rest;
							Some(self.offset)
						},
						_=>panic!("missing 2"),
					}
				}
			},
			&[]=>return None,
		}
	}
}

#[derive(Debug,Clone)]
pub struct Faces5{
	pub faces:Vec<u32>,
}
impl binrw::BinRead for Faces5{
	type Args<'a>=();
	fn read_options<R:Read+Seek>(
		reader:&mut R,
		_endian:binrw::Endian,
		_args:Self::Args<'_>,
	)->binrw::BinResult<Self>{
		let vertex_count=u32::read_le(reader)?;
		// ignore validation
		let _faces_data_len=u32::read_le(reader)?;
		let mut faces=Vec::with_capacity(vertex_count as usize);
		let mut index=0;
		for _ in 0..vertex_count{
			let v0=u8::read_le(reader)?;
			if v0&0x80==0{
				index+=v0 as u32;
			}else{
				let [v2,v1]=u16::read_le(reader)?.to_le_bytes();
				index+=u32::from_le_bytes([v2,v1,v0&0x7F,0]);
			}
			faces.push(index);
		}
		Ok(Self{
			faces,
		})
	}
}
#[derive(Debug,Clone)]
pub struct QuantizedF32x3{
	pub values:Vec<[f32;3]>,
}
impl binrw::BinRead for QuantizedF32x3{
	type Args<'a>=();
	fn read_options<R:Read+Seek>(
		reader:&mut R,
		_endian:binrw::Endian,
		_args:Self::Args<'_>,
	)->binrw::BinResult<Self>{
		const SCALE: f32 = 1.0 / 32_767.0; // ? ok
		let values_count=u16::read_le(reader)?;
		// ignore validation
		let _data_len=u32::read_le(reader)?;
		let mut values=Vec::with_capacity(values_count as usize);
		for _ in 0..values_count{
			let x=i16::read_le(reader)?;
			let y=i16::read_le(reader)?;
			let z=i16::read_le(reader)?;
			values.push([
				(x as f32)*SCALE,
				(y as f32)*SCALE,
				(z as f32)*SCALE,
			]);
		}
		Ok(Self{
			values,
		})
	}
}
#[binrw::binread]
#[br(little)]
#[derive(Debug,Clone)]
pub struct CSGMDL5{
	// #[brw(magic=b"CSGMDL\x05\0\0\0")] but obfuscated
	#[brw(magic=b"\x15\x7d\x29\x15\x75\x6c\x35\x04\x34\x69")]
	pub pos_count:u16,//208
	#[br(count=pos_count)]
	pub positions:Vec<[f32;3]>,

	pub normals:QuantizedF32x3,

	pub color_count:u16,//208
	#[br(count=color_count)]
	pub colors:Vec<[u8;4]>,

	pub normal_id_count:u16,//208
	#[br(count=normal_id_count)]
	pub normal_ids:Vec<NormalId5>,

	pub tex_count:u16,//208
	#[br(count=tex_count)]
	pub tex:Vec<[f32;2]>,

	pub tangents:QuantizedF32x3,

	// delta encoded vertex indices
	pub faces:Faces5,

	pub range_marker_count:u8,//2-3
	#[br(count=range_marker_count)]
	// the numbers in this list seem to match various list lengths
	pub range_markers:Vec<u32>,
}
#[binrw::binread]
#[br(little)]
#[derive(Debug,Clone)]
pub enum CSGMDL{
	V2(CSGMDL2),
	V4(CSGMDL4),
	V5(CSGMDL5),
}
#[derive(Debug,Clone)]
pub enum MeshData{
	CSGK(CSGK),
	CSGMDL(CSGMDL),
}
impl binrw::BinRead for MeshData{
	type Args<'a>=();
	fn read_options<R:Read+Seek>(
		reader:&mut R,
		endian:binrw::Endian,
		args:Self::Args<'_>,
	)->binrw::BinResult<Self>{
		let header=Header::read_options(reader,endian,args)?;
		Ok(match header{
			Header::CSGK(csgk)=>MeshData::CSGK(csgk),
			Header::CSGMDL(header_version)=>{
				reader.seek(std::io::SeekFrom::Start(0))?;
				match header_version{
					HeaderVersion::CSGMDL2=>MeshData::CSGMDL(CSGMDL::V2(CSGMDL2::read_options(&mut Obfuscator::new(reader),endian,args)?)),
					HeaderVersion::CSGMDL4=>MeshData::CSGMDL(CSGMDL::V4(CSGMDL4::read_options(&mut Obfuscator::new(reader),endian,args)?)),
					// in version 5 only the header is obfuscated.
					HeaderVersion::CSGMDL5=>MeshData::CSGMDL(CSGMDL::V5(CSGMDL5::read_options(reader,endian,args)?)),
				}
			}
		})
	}
}
impl binrw::BinWrite for MeshData{
	type Args<'a>=();
	fn write_options<W:Write+Seek>(
		&self,
		writer:&mut W,
		endian:binrw::Endian,
		args:Self::Args<'_>,
	)->binrw::BinResult<()>{
		match self{
			MeshData::CSGK(csgk)=>csgk.write_options(writer,endian,args),
			MeshData::CSGMDL(CSGMDL::V2(mesh_data2))=>mesh_data2.write_options(&mut Obfuscator::new(writer),endian,args),
			MeshData::CSGMDL(CSGMDL::V4(mesh_data4))=>mesh_data4.write_options(&mut Obfuscator::new(writer),endian,args),
			MeshData::CSGMDL(CSGMDL::V5(mesh_data5))=>panic!(),//mesh_data5.write_options(writer,endian,args),
		}
	}
}
