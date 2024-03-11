//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs
use std::io::{Read,BufRead};

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
}

pub enum VersionedMesh{
	Version1(Mesh1),
	//Version2(Mesh2),
	//Version3(Mesh3),
	//Version4(Mesh4),
	//Version5(Mesh5),
	//Version6(Mesh6),
	//Version7(Mesh7),
}

pub fn read<R:Read>(mut read:R)->Result<VersionedMesh,Error>{
	let mut buf=[0u8;12];
	read.read_exact(&mut buf).map_err(Error::Io)?;
	match &buf{
		b"version 1.00"=>Ok(VersionedMesh::Version1(read_100(read)?)),
		b"version 1.01"=>Ok(VersionedMesh::Version1(read_101(read)?)),
		//b"version 2.00"=>Ok(VersionedMesh::Version2(read_200(read)?)),
		//b"version 3.00"=>Ok(VersionedMesh::Version3(read_300(read)?)),
		//b"version 3.01"=>Ok(VersionedMesh::Version3(read_301(read)?)),
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
pub struct Mesh1{
	pub face_count:u32,
	pub vertices:Vec<Vertex1>
}

pub fn read_100<R:Read>(read:R)->Result<Mesh1,Error>{
	let mut mesh=read1(read)?;
	//we'll fix it in post
	for vertex in &mut mesh.vertices{
		for p in &mut vertex.pos{
			*p=*p*0.5;
		}
	}
	Ok(mesh)
}

#[inline]
pub fn read_101<R:Read>(read:R)->Result<Mesh1,Error>{
	read1(read)
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
		face_count,
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
	}).collect::<Result<Vec<Vertex1>,Error>>()?})
}

