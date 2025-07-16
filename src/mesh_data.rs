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
#[derive(Debug)]
pub struct NormalIDError;
impl std::fmt::Display for NormalIDError{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		write!(f,"{self:?}")
	}
}
impl core::error::Error for NormalIDError{}
// Why does this differ from Roblox's own standard?
#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub enum NormalId{
	Right=1,
	Top=2,
	Back=3,
	Left=4,
	Bottom=5,
	Front=6,
}
#[binrw::binrw]
#[brw(little,repr=u32)]
#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub struct NormalId2(pub NormalId);
impl From<&NormalId2> for u32{
	#[inline]
	fn from(&NormalId2(value):&NormalId2)->u32{
		value as u32
	}
}
impl TryFrom<u32> for NormalId2{
	type Error=NormalIDError;
	#[inline]
	fn try_from(value:u32)->Result<NormalId2,NormalIDError>{
		Ok(NormalId2(match value{
			1=>NormalId::Right,
			2=>NormalId::Top,
			3=>NormalId::Back,
			4=>NormalId::Left,
			5=>NormalId::Bottom,
			6=>NormalId::Front,
			_=>return Err(NormalIDError),
		}))
	}
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
#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub struct NormalId5(pub NormalId);
impl From<&NormalId5> for u8{
	#[inline]
	fn from(&NormalId5(value):&NormalId5)->u8{
		value as u8
	}
}
impl TryFrom<u8> for NormalId5{
	type Error=NormalIDError;
	#[inline]
	fn try_from(value:u8)->Result<NormalId5,NormalIDError>{
		Ok(NormalId5(match value{
			1=>NormalId::Right,
			2=>NormalId::Top,
			3=>NormalId::Back,
			4=>NormalId::Left,
			5=>NormalId::Bottom,
			6=>NormalId::Front,
			_=>return Err(NormalIDError),
		}))
	}
}

#[derive(Debug)]
pub enum FacesStateMachineError{
	UnexpectedEOF,
	UnusedData,
}
impl std::fmt::Display for FacesStateMachineError{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		write!(f,"{self:?}")
	}
}
impl core::error::Error for FacesStateMachineError{}

#[derive(Debug,Clone)]
pub struct Faces5{
	pub faces:Vec<u32>,
	pub _unknown:Vec<u32>,
}
impl binrw::BinRead for Faces5{
	type Args<'a>=();
	fn read_options<R:Read+Seek>(
		reader:&mut R,
		_endian:binrw::Endian,
		_args:Self::Args<'_>,
	)->binrw::BinResult<Self>{
		#[binrw::binrw]
		#[brw(little)]
		enum FacesRangeMarkers{
			#[brw(magic=2u8)]
			Two{
				range_start:u32,
				range_end:u32,
			},
			#[brw(magic=3u8)]
			Three{
				range_start:u32,
				range_end:u32,
				range_extra:u32,
			},
		}
		// complete faces data
		#[binrw::binrw]
		#[brw(little)]
		struct Faces5Inner{
			vertex_count:u32,
			vertex_data_len:u32,
			#[br(count=vertex_data_len)]
			vertex_data:Vec<u8>,
			range_markers:FacesRangeMarkers,
		}

		fn read_state_machine(data:Vec<u8>,expected_output_count:usize)->Result<Vec<u32>,FacesStateMachineError>{
			let mut indices=Vec::with_capacity(expected_output_count);
			let mut it=data.into_iter();
			let mut index_out=0;
			for _ in 0..expected_output_count{
				let v0=it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
				if v0&(1<<7)==0{
					// TODO: test whether 64 goes to top or bottom case
					if v0&(1<<6)==0{
						index_out+=v0 as u32;
					}else{
						// 64..127 is mapped to -64..-1
						index_out-=-((v0|0x80) as i8) as u32;
					}
				}else{
					let v1=it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
					let v2=it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
					index_out+=u32::from_le_bytes([v2,v1,v0&0x7F,0]);
				}
				indices.push(index_out&0x7FFFFF);
			}

			// iterator should be fully depleted
			if it.next().is_some(){
				return Err(FacesStateMachineError::UnusedData);
			}

			Ok(indices)
		}

		// read complete data
		let faces_inner:Faces5Inner=reader.read_le()?;

		// accumulate vertex indices using state machine
		let mut faces=read_state_machine(faces_inner.vertex_data,faces_inner.vertex_count as usize)
		.map_err(|e|{
			Error::Custom{
				// TODO: inject position
				pos:0,
				err:Box::new(e),
			}
		})?;

		// split indices according to range marker count
		Ok(match faces_inner.range_markers{
			FacesRangeMarkers::Two{..}=>{
				// TODO: check range markers against observed counts
				Self{
					faces,
					_unknown:Vec::new(),
				}
			},
			FacesRangeMarkers::Three{range_end,..}=>{
				// TODO: check range markers against observed counts
				let _unknown=faces.split_off(range_end as usize);
				Self{
					faces,
					_unknown,
				}
			},
		})
	}
}

#[binrw::binread]
#[br(little)]
#[br(map=Self::read)]
#[repr(transparent)]
struct QuantizedF32x3([f32;3]);
impl QuantizedF32x3{
	fn read([x,y,z]:[i16;3])->Self{
		const SCALE:f32=1.0/32_767.0; // ? ok
		Self([
			(x.wrapping_sub(0x7FFF) as f32)*SCALE,
			(y.wrapping_sub(0x7FFF) as f32)*SCALE,
			(z.wrapping_sub(0x7FFF) as f32)*SCALE,
		])
	}
}
fn parse_quantized_f32x3_array<R:binrw::io::Read+binrw::io::Seek>(
	reader:&mut R,
	_endian:binrw::Endian,
	args:binrw::VecArgs<()>,
)->binrw::BinResult<Vec<[f32;3]>>{
	// read quantized i16 values directly into Vec, converting to f32 on the fly
	let quantized:Vec<QuantizedF32x3>=reader.read_le_args(args)?;
	// transmute into expected type
	// SAFETY: QuantizedF32x3 is #[repr(transparent)]
	let transmuted:Vec<[f32;3]>=unsafe{core::mem::transmute(quantized)};
	// Equivalent safe code
	// let transmuted=quantized.into_iter().map(|QuantizedF32x3(value)|value).collect();
	Ok(transmuted)
}

#[binrw::binread]
#[br(little)]
#[derive(Debug,Clone)]
pub struct CSGMDL5{
	// #[brw(magic=b"CSGMDL\x05\0\0\0")] but obfuscated
	#[brw(magic=b"\x15\x7d\x29\x15\x75\x6c\x35\x04\x34\x69")]
	pub pos_count:u16,
	#[br(count=pos_count)]
	pub positions:Vec<[f32;3]>,

	pub normals_count:u16,
	pub normals_len:u32,
	#[br(parse_with=parse_quantized_f32x3_array,args_raw=binrw::VecArgs{count:normals_count as usize,inner:()})]
	pub normals:Vec<[f32;3]>,

	pub color_count:u16,
	#[br(count=color_count)]
	pub colors:Vec<[u8;4]>,

	pub normal_id_count:u16,
	#[br(count=normal_id_count)]
	pub normal_ids:Vec<NormalId5>,

	pub tex_count:u16,
	#[br(count=tex_count)]
	pub tex:Vec<[f32;2]>,

	pub tangents_count:u16,
	pub tangents_len:u32,
	#[br(parse_with=parse_quantized_f32x3_array,args_raw=binrw::VecArgs{count:tangents_count as usize,inner:()})]
	pub tangents:Vec<[f32;3]>,

	// delta encoded vertex indices
	pub faces:Faces5,
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

#[derive(Debug)]
pub enum WriteMeshDataError{
	CSGMDL5,
}
impl std::fmt::Display for WriteMeshDataError{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		match self{
			Self::CSGMDL5=>write!(f,"Writing CSGMDL5 is not supported")
		}
	}
}
impl core::error::Error for WriteMeshDataError{}
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
			MeshData::CSGMDL(CSGMDL::V5(_mesh_data5))=>{
				//mesh_data5.write_options(writer,endian,args),
				Err(Error::Custom{
					pos:0,
					err:Box::new(WriteMeshDataError::CSGMDL5),
				})
			},
		}
	}
}
