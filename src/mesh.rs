//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs
use std::io::{BufRead,Read,Seek};

use binrw::BinReaderExt;

#[derive(Debug)]
pub enum Error{
	Io(std::io::Error),
	Header,
	UnknownVersion(Vec<u8>),
	//1.00
	UnexpectedEof,
	ParseIntError(std::num::ParseIntError),
	ParseFloatError(std::num::ParseFloatError),
	Regex,
	PositionDimensionNot3(usize),
	NormalDimensionNot3(usize),
	TextureCoordsDimensionNot3(usize),
	VertexTripletCount,
	VertexCount,
	//2.00
	BinRead(binrw::Error),
}
impl std::fmt::Display for Error{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		write!(f,"{self:?}")
	}
}
impl std::error::Error for Error{}

pub enum VersionedMesh{
	Version1(Mesh1),
	Version2(Mesh2),
	Version3(Mesh3),
	Version4(Mesh4),
	//Version5(Mesh5),
	//Version6(Mesh6),
	//Version7(Mesh7),
}

pub fn read<R:Read+Seek>(read:R)->Result<VersionedMesh,Error>{
	let mut buf_reader=binrw::io::BufReader::new(read);
	let buf=buf_reader.fill_buf().map_err(Error::Io)?;
	match &buf[0..12]{
		b"version 1.00"=>Ok(VersionedMesh::Version1(read_100(buf_reader)?)),
		b"version 1.01"=>Ok(VersionedMesh::Version1(read_101(buf_reader)?)),
		b"version 2.00"=>Ok(VersionedMesh::Version2(read_200(buf_reader)?)),
		b"version 3.00"
		|b"version 3.01"=>Ok(VersionedMesh::Version3(read_300(buf_reader)?)),
		b"version 4.00"
		|b"version 4.01"=>Ok(VersionedMesh::Version4(read_400(buf_reader)?)),
		//b"version 5.00"=>Ok(VersionedMesh::Version5(read_500(buf_reader)?)),
		//b"version 6.00"=>Ok(VersionedMesh::Version6(read_600(buf_reader)?)),
		//b"version 7.00"=>Ok(VersionedMesh::Version7(read_700(buf_reader)?)),
		other=>Err(Error::UnknownVersion(other.to_vec())),
	}
}

struct LineMachine<R:Read>{
	lines:std::io::Lines<std::io::BufReader<R>>,
}
impl<R:Read> LineMachine<R>{
	fn new(read:R)->Self{
		Self{
			lines:std::io::BufReader::new(read).lines(),
		}
	}
	fn read_u32(&mut self)->Result<u32,Error>{
		Ok(self.lines.next().ok_or(Error::UnexpectedEof)?.map_err(Error::Io)?.trim().parse().map_err(Error::ParseIntError)?)
	}
	fn read_line(&mut self)->Result<String,Error>{
		Ok(self.lines.next().ok_or(Error::UnexpectedEof)?.map_err(Error::Io)?)
	}
}

pub struct Vertex1{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub tex:[f32;3],
}
pub struct Header1{
	pub face_count:u32,
}
pub struct Mesh1{
	pub header:Header1,
	pub vertices:Vec<Vertex1>
}

#[inline]
pub fn fix_100(mesh:&mut Mesh1){
	for vertex in &mut mesh.vertices{
		for p in &mut vertex.pos{
			*p=*p*0.5;
		}
	}
}
#[inline]
pub fn fix1(mesh:&mut Mesh1){
	for vertex in &mut mesh.vertices{
		vertex.tex[1]=1.0-vertex.tex[1];
	}
}
#[inline]
pub fn check1(mesh:Mesh1)->Result<Mesh1,Error>{
	if 3*(mesh.header.face_count as usize)==mesh.vertices.len(){
		Ok(mesh)
	}else{
		Err(Error::VertexCount)
	}
}

#[inline]
pub fn read_100<R:Read>(read:R)->Result<Mesh1,Error>{
	let mut mesh=read1(read)?;
	//we'll fix it in post
	fix1(&mut mesh);
	fix_100(&mut mesh);
	check1(mesh)
}

#[inline]
pub fn read_101<R:Read>(read:R)->Result<Mesh1,Error>{
	let mut mesh=read1(read)?;
	fix1(&mut mesh);
	check1(mesh)
}

pub fn read1<R:Read>(read:R)->Result<Mesh1,Error>{
	let mut lines=LineMachine::new(read);
	//drop version line
	lines.read_line()?;
	//NumFaces
	let face_count=lines.read_u32()?;
	//vertices
	let vertices_line=lines.read_line()?;
	let mut captures_iter=lazy_regex::regex!(r"\[(.*?)\]")
	.captures_iter(vertices_line.as_str());
	Ok(Mesh1{
		header:Header1{face_count},
		vertices:std::iter::from_fn(||{
			//match three at a time, otherwise fail
			match (captures_iter.next(),captures_iter.next(),captures_iter.next()){
				(Some(pos_capture),Some(norm_capture),Some(tex_capture))=>Some((||{//use a closure to make errors easier
					let pos={
						let pos=pos_capture.get(1).ok_or(Error::Regex)?.as_str().split(",").map(|f|
							f.parse().map_err(Error::ParseFloatError)
						).collect::<Result<Vec<f32>,Error>>()?;
						match pos.as_slice(){
							&[x,y,z]=>[x,y,z],
							_=>return Err(Error::PositionDimensionNot3(pos.len())),
						}
					};
					let norm={
						let norm=norm_capture.get(1).ok_or(Error::Regex)?.as_str().split(",").map(|f|
							f.parse().map_err(Error::ParseFloatError)
						).collect::<Result<Vec<f32>,Error>>()?;
						match norm.as_slice(){
							&[x,y,z]=>[x,y,z],
							_=>return Err(Error::NormalDimensionNot3(norm.len())),
						}
					};
					let tex={
						let tex=tex_capture.get(1).ok_or(Error::Regex)?.as_str().split(",").map(|f|
							f.parse().map_err(Error::ParseFloatError)
						).collect::<Result<Vec<f32>,Error>>()?;
						match tex.as_slice(){
							&[x,y,w]=>[x,y,w],
							_=>return Err(Error::TextureCoordsDimensionNot3(tex.len())),
						}
					};
					Ok(Vertex1{
						pos,
						norm,
						tex,
					})
				})()),//closure called here
				(None,None,None)=>None,
				_=>Some(Err(Error::VertexTripletCount)),
			}
		}).collect::<Result<Vec<Vertex1>,Error>>()?
	})
}

#[binrw::binrw]
#[brw(little)]
pub struct Header2{
	#[brw(magic=b"version 2.00\n\x0C\0\x28\x0C")]
	//sizeof_header:u16,//12
	//sizeof_vertex:u8,//40
	//sizeof_face:u8,//12
	pub vertex_count:u32,
	pub face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
pub struct Vertex2{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub tex:[f32;2],
	pub tangent:[i8;4],// Tangent Vector & Bi-Normal Direction
	pub color:[u8;4],
}
#[binrw::binrw]
#[brw(little)]
pub struct VertexId2(pub u32);
#[binrw::binrw]
#[brw(little)]
pub struct Face2(pub VertexId2,pub VertexId2,pub VertexId2);
#[binrw::binrw]
#[brw(little)]
pub struct Mesh2{
	pub header:Header2,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
}

//alternate version with truncated vertex...
#[binrw::binrw]
#[brw(little)]
struct Header2_36{
	#[brw(magic=b"version 2.00\n\x0C\0\x24\x0C")]
	//sizeof_header:u16,//12
	//sizeof_vertex:u8,//36
	//sizeof_face:u8,//12
	vertex_count:u32,
	face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
struct Vertex2_36{
	pos:[f32;3],
	norm:[f32;3],
	tex:[f32;2],
	tangent:[i8;4],// Tangent Vector & Bi-Normal Direction
}
#[binrw::binrw]
#[brw(little)]
struct Mesh2_36{
	header:Header2_36,
	#[br(count=header.vertex_count)]
	vertices:Vec<Vertex2_36>,
	#[br(count=header.face_count)]
	faces:Vec<Face2>,
}

#[inline]
pub fn fix2(mesh:&mut Mesh2){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=[0,0,-128,127],
			_=>(),
		}
	}
}

#[inline]
pub fn read_200<R:Read+Seek>(read:R)->Result<Mesh2,Error>{
	let mut mesh=read2(read)?;
	fix2(&mut mesh);
	Ok(mesh)
}

pub fn read2<R:BinReaderExt>(mut read:R)->Result<Mesh2,Error>{
	match read.read_le(){
		//read normally
		Ok(mesh)=>Ok(mesh),
		Err(e)=>{
			//devious error matching
			match &e{
				binrw::Error::Backtrace(binrw::error::Backtrace{
					error,
					frames:_,
					..
				})=>match error.as_ref(){
					binrw::Error::BadMagic{..}=>(),
					_=>return Err(Error::BinRead(e)),
				},
				_=>return Err(Error::BinRead(e)),
			}
			//read truncated vertex mesh
			let mesh:Mesh2_36=read.read_le().map_err(Error::BinRead)?;
			//convert to normal
			Ok(Mesh2{
				header:Header2{
					vertex_count:mesh.header.vertex_count,
					face_count:mesh.header.face_count,
				},
				vertices:mesh.vertices.into_iter().map(|v|{
					Vertex2{
						pos:v.pos,
						norm:v.norm,
						tex:v.tex,
						tangent:v.tangent,
						color:[255u8;4],
					}
				}).collect(),
				faces:mesh.faces,
			})
		},
	}
}

#[binrw::binrw]
#[brw(little)]
pub enum Revision3{
	#[brw(magic=b"3.00")]
	Version300,
	#[brw(magic=b"3.01")]
	Version301,
}

#[binrw::binrw]
#[brw(little)]
pub struct Header3{
	#[brw(magic=b"version ")]
	pub revision:Revision3,
	#[brw(magic=b"\n\x10\0\x28\x0C\x04\0")]
	//sizeof_header:u16,//16
	//sizeof_vertex:u8,//40
	//sizeof_face:u8,//12
	//sizeof_lod:u16,//4
	pub lod_count:u16,
	pub vertex_count:u32,
	pub face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
pub struct Lod3(pub u32);
#[binrw::binrw]
#[brw(little)]
pub struct Mesh3{
	pub header:Header3,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods:Vec<Lod3>,
}

#[binrw::binrw]
#[brw(little)]
struct Header3_36{
	#[brw(magic=b"version ")]
	revision:Revision3,
	#[brw(magic=b"\n\x10\0\x24\x0C\x04\0")]
	//sizeof_header:u16,//16
	//sizeof_vertex:u8,//36
	//sizeof_face:u8,//12
	//sizeof_lod:u16,//4
	lod_count:u16,
	vertex_count:u32,
	face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
struct Mesh3_36{
	header:Header3_36,
	#[br(count=header.vertex_count)]
	vertices:Vec<Vertex2_36>,
	#[br(count=header.face_count)]
	faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	lods:Vec<Lod3>,
}

#[inline]
pub fn fix3(mesh:&mut Mesh3){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=[0,0,-128,127],
			_=>(),
		}
	}
}

#[inline]
pub fn read_300<R:Read+Seek>(read:R)->Result<Mesh3,Error>{
	let mut mesh=read3(read)?;
	fix3(&mut mesh);
	Ok(mesh)
}

pub fn read3<R:BinReaderExt>(mut read:R)->Result<Mesh3,Error>{
	match read.read_le(){
		//read normally
		Ok(mesh)=>Ok(mesh),
		Err(e)=>{
			//devious error matching
			match &e{
				binrw::Error::Backtrace(binrw::error::Backtrace{
					error,
					frames:_,
					..
				})=>match error.as_ref(){
					binrw::Error::BadMagic{..}=>(),
					_=>return Err(Error::BinRead(e)),
				},
				_=>return Err(Error::BinRead(e)),
			}
			//read truncated vertex mesh
			let mesh:Mesh3_36=read.read_le().map_err(Error::BinRead)?;
			//convert to normal
			Ok(Mesh3{
				header:Header3{
					revision:mesh.header.revision,
					vertex_count:mesh.header.vertex_count,
					face_count:mesh.header.face_count,
					lod_count:mesh.header.lod_count,
				},
				vertices:mesh.vertices.into_iter().map(|v|{
					Vertex2{
						pos:v.pos,
						norm:v.norm,
						tex:v.tex,
						tangent:v.tangent,
						color:[255u8;4],
					}
				}).collect(),
				faces:mesh.faces,
				lods:mesh.lods,
			})
		},
	}
}

#[binrw::binrw]
#[brw(little)]
pub enum Revision4{
	#[brw(magic=b"4.00")]
	Version400,
	#[brw(magic=b"4.01")]
	Version401,
}
#[binrw::binrw]
#[brw(little,repr=u16)]
pub enum LodType4
{
	None=0,
	Unknown=1,
	RbxSimplifier=2,
	ZeuxMeshOptimizer=3,
	Type4=4,//shows up in sphere.mesh, don't know what it is
}
#[binrw::binrw]
#[brw(little)]
pub struct Header4{
	#[brw(magic=b"version ")]
	pub revision:Revision4,
	#[brw(magic=b"\n\x18\0")]
	//sizeof_header:u16,//24
	pub lod_type:LodType4,
	pub vertex_count:u32,
	pub face_count:u32,
	pub lod_count:u16,
	pub bone_count:u16,
	pub bone_names_len:u32,
	pub subset_count:u16,
	pub lod_hq_count:u8,
	_padding:u8,
}
#[binrw::binrw]
#[brw(little)]
pub struct Envelope4{
	pub bones:[u8;4],
	pub weights:[u8;4],
}
#[binrw::binrw]
#[brw(little)]
pub struct BoneId4(u16);
impl BoneId4{
	pub fn new(value:Option<u16>)->Self{
		Self(match value{
			None=>0xFFFF,
			//|Some(0xFFFF)//whatever
			Some(other)=>other,
		})
	}
	pub fn get(&self)->Option<u16>{
		match self.0{
			0xFFFF=>None,
			other=>Some(other),
		}
	}
}
#[binrw::binrw]
#[brw(little)]
pub struct CFrame4{
	pub r00:f32,pub r01:f32,pub r02:f32,
	pub r10:f32,pub r11:f32,pub r12:f32,
	pub r20:f32,pub r21:f32,pub r22:f32,
	pub x:f32,pub y:f32,pub z:f32,
}
#[binrw::binrw]
#[brw(little)]
pub struct Bone4{
	pub bone_name_pos:u32,
	pub parent:BoneId4,
	pub lod_parent:BoneId4,
	pub cull_distance:f32,
	pub cframe:CFrame4,
}
#[binrw::binrw]
#[brw(little)]
pub struct Subset4{
	pub faces_offset:u32,
	pub faces_len:u32,
	pub vertices_offset:u32,
	pub vertices_len:u32,
	pub bone_count:u32,
	pub bones:[BoneId4;26],
}
#[binrw::binrw]
#[brw(little)]
pub struct Mesh4{
	pub header:Header4,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=header.vertex_count)]
	pub envelopes:Vec<Envelope4>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods:Vec<Lod3>,
	#[br(count=header.bone_count)]
	pub bones:Vec<Bone4>,
	#[br(count=header.bone_names_len)]
	pub bone_names:Vec<u8>,
	#[br(count=header.subset_count)]
	pub subsets:Vec<Subset4>,
}

#[binrw::binrw]
#[brw(little)]
struct Header4Boneless{
	#[brw(magic=b"version ")]
	revision:Revision4,
	#[brw(magic=b"\n\x18\0")]
	//sizeof_header:u16,//24
	lod_type:LodType4,
	vertex_count:u32,
	face_count:u32,
	lod_count:u16,
	#[brw(magic=0u16)]
	//bone_count:u16,
	bone_names_len:u32,
	subset_count:u16,
	lod_hq_count:u8,
	_padding:u8,
}
#[binrw::binrw]
#[brw(little)]
struct Mesh4Boneless{
	header:Header4Boneless,
	#[br(count=header.vertex_count)]
	vertices:Vec<Vertex2>,
	#[br(count=header.face_count)]
	faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	lods:Vec<Lod3>,
	#[br(count=header.bone_names_len)]
	bone_names:Vec<u8>,
	#[br(count=header.subset_count)]
	subsets:Vec<Subset4>,
}

#[inline]
pub fn fix4(mesh:&mut Mesh4){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=[0,0,-128,127],
			_=>(),
		}
	}
}

#[inline]
pub fn read_400<R:Read+Seek>(read:R)->Result<Mesh4,Error>{
	let mut mesh=read4(read)?;
	fix4(&mut mesh);
	Ok(mesh)
}

pub fn read4<R:BinReaderExt>(mut read:R)->Result<Mesh4,Error>{
	match read.read_le::<Mesh4Boneless>(){
		Err(e)=>{
			//devious error matching
			match &e{
				binrw::Error::Backtrace(binrw::error::Backtrace{
					error,
					frames:_,
					..
				})=>match error.as_ref(){
					binrw::Error::BadMagic{..}=>(),
					_=>return Err(Error::BinRead(e)),
				},
				_=>return Err(Error::BinRead(e)),
			}
			//read normally
			read.read_le().map_err(Error::BinRead)
		},
		//boneless mesh
		Ok(mesh)=>{
			//convert to normal
			Ok(Mesh4{
				header:Header4{
					revision:mesh.header.revision,
					vertex_count:mesh.header.vertex_count,
					face_count:mesh.header.face_count,
					lod_count:mesh.header.lod_count,
					lod_type:mesh.header.lod_type,
					bone_count:0,
					bone_names_len:mesh.header.bone_names_len,
					subset_count:mesh.header.subset_count,
					lod_hq_count:mesh.header.lod_hq_count,
					_padding:mesh.header._padding,
				},
				vertices:mesh.vertices,
				envelopes:Vec::new(),
				faces:mesh.faces,
				lods:mesh.lods,
				bones:Vec::new(),
				bone_names:mesh.bone_names,
				subsets:mesh.subsets,
			})
		},
	}
}