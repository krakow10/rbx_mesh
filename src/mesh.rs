//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs
use std::io::{BufRead,Read,Seek};

use binrw::BinReaderExt;

#[derive(Debug)]
pub enum Error{
	Io(std::io::Error),
	Header,
	UnknownVersion([u8;12]),
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
	//Version4(Mesh4),
	//Version5(Mesh5),
	//Version6(Mesh6),
	//Version7(Mesh7),
}

pub fn read<R:Read+Seek>(mut read:R)->Result<VersionedMesh,Error>{
	let mut buf=[0u8;12];
	read.read_exact(&mut buf).map_err(Error::Io)?;
	match &buf{
		b"version 1.00"=>Ok(VersionedMesh::Version1(read_100(read)?)),
		b"version 1.01"=>Ok(VersionedMesh::Version1(read_101(read)?)),
		b"version 2.00"=>Ok(VersionedMesh::Version2(read_200(read)?)),
		b"version 3.00"
		|b"version 3.01"=>Ok(VersionedMesh::Version3(read_300(read)?)),
		//b"version 4.00"=>Ok(VersionedMesh::Version4(read_400(read)?)),
		//b"version 4.01"=>Ok(VersionedMesh::Version4(read_401(read)?)),
		//b"version 5.00"=>Ok(VersionedMesh::Version5(read_500(read)?)),
		//b"version 6.00"=>Ok(VersionedMesh::Version6(read_600(read)?)),
		//b"version 7.00"=>Ok(VersionedMesh::Version7(read_700(read)?)),
		_=>Err(Error::UnknownVersion(buf)),
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
	//drop empty line
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
	#[brw(magic=b"\x0C\0\x28\x0C")]//12u16,40u8,12u8
	//sizeof_header:u16,//size of this struct...
	//sizeof_vertex:u8,
	//sizeof_face:u8,
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
	#[brw(magic=b'\n')]//probably not the greatest idea but whatever
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
	#[brw(magic=b"\x0C\0\x24\x0C")]//12u16,36u8,12u8
	//sizeof_header:u16,//size of this struct...
	//sizeof_vertex:u8,
	//sizeof_face:u8,
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
	#[brw(magic=b'\n')]//probably not the greatest idea but whatever
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
		Err(binrw::Error::BadMagic{..})=>{
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
		Err(other)=>Err(Error::BinRead(other)),
	}
}

#[binrw::binrw]
#[brw(little)]
pub struct Header3{
	#[brw(magic=b"\x10\0\x28\x0C\x04\0")]//16u16,40u8,12u8,4u16
	//sizeof_header:u16,//size of this struct...
	//sizeof_vertex:u8,
	//sizeof_face:u8,
	//sizeof_lod:u16,
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
	#[brw(magic=b'\n')]//probably not the greatest idea but whatever
	pub header:Header3,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods:Vec<Lod3>,
}

#[inline]
pub fn read_300<R:Read+Seek>(read:R)->Result<Mesh3,Error>{
	read3(read)
}

pub fn read3<R:BinReaderExt>(mut read:R)->Result<Mesh3,Error>{
	read.read_le().map_err(Error::BinRead)
}