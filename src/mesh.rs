//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs
use std::io::{Read,BufRead};

#[derive(Debug)]
pub enum Error{
	Io(std::io::Error),
	Header,
	UnexpectedEof,
	ParseIntError(std::num::ParseIntError),
	ParseFloatError(std::num::ParseFloatError),
	Regex,
	PositionDimensionNot3(usize),
	NormalDimensionNot3(usize),
	TextureCoordsDimensionNot2(usize),
	VertexTripletCount,
	VertexCount,
}

#[inline]
const fn ch(c:u8)->Option<u32>{
	match c{
		b'0'=>Some(0),
		b'1'=>Some(1),
		b'2'=>Some(2),
		b'3'=>Some(3),
		b'4'=>Some(4),
		b'5'=>Some(5),
		b'6'=>Some(6),
		b'7'=>Some(7),
		b'8'=>Some(8),
		b'9'=>Some(9),
		_=>None,
	}
}

pub fn convert<R:Read>(mut read:R)->Result<obj::ObjData,Error>{
	let mut buf=[0u8;12];
	read.read_exact(&mut buf).map_err(Error::Io)?;
	//b"version 6.02" -> 602u32
	let version=match (&buf[0..8],ch(buf[8+0]),buf[8+1],ch(buf[8+2]),ch(buf[8+3])){
		(b"version ",Some(huns),b'.',Some(tens),Some(ones))=>huns*100+tens*10+ones,
		_=>return Err(Error::Header),
	};

	match version{
		100=>read_ascii(read),
		_=>read_bin(read),
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
	fn read_usize(&mut self)->Result<usize,Error>{
		Ok(self.lines.next().ok_or(Error::UnexpectedEof)?.map_err(Error::Io)?.parse().map_err(Error::ParseIntError)?)
	}
	fn read_line(&mut self)->Result<String,Error>{
		Ok(self.lines.next().ok_or(Error::UnexpectedEof)?.map_err(Error::Io)?)
	}
}

fn read_ascii<R:Read>(read:R)->Result<obj::ObjData,Error>{
	let mut lines=LineMachine::new(read);
	//NumFaces
	let face_count=lines.read_usize()?;
	//vertices
	let vertices_line=lines.read_line()?;
	let captures=lazy_regex::regex!(r"\[(.*?)\]")
	.captures(vertices_line.as_str())
	.ok_or(Error::Regex)?;
	let mut captures_iter=captures.iter();
	//ignore the full capture thing at position 0
	captures_iter.next().ok_or(Error::Regex)?;
	let mut position=Vec::new();
	let mut texture=Vec::new();
	let mut normal=Vec::new();
	let index_tuples=std::iter::from_fn(||{
		//match three at a time, otherwise fail
		match (captures_iter.next(),captures_iter.next(),captures_iter.next()){
			(Some(Some(pos)),Some(Some(norm)),Some(Some(tex)))=>Some((||{//use a closure to make errors easier
				//pos
				let pos=pos.as_str().split(",").map(|f|
					f.parse().map_err(Error::ParseFloatError)
				).collect::<Result<Vec<f32>,Error>>()?;
				let pos_idx=position.len();
				match pos.as_slice(){
					&[x,y,z]=>position.push([x,y,z]),
					_=>return Err(Error::PositionDimensionNot3(pos.len())),
				}
				//norm
				let norm=norm.as_str().split(",").map(|f|
					f.parse().map_err(Error::ParseFloatError)
				).collect::<Result<Vec<f32>,Error>>()?;
				let norm_idx=normal.len();
				match norm.as_slice(){
					&[x,y,z]=>normal.push([x,y,z]),
					_=>return Err(Error::NormalDimensionNot3(norm.len())),
				}
				//tex
				let tex=tex.as_str().split(",").map(|f|
					f.parse().map_err(Error::ParseFloatError)
				).collect::<Result<Vec<f32>,Error>>()?;
				let tex_idx=texture.len();
				match tex.as_slice(){
					&[x,y]=>texture.push([x,1.0-y]),
					_=>return Err(Error::TextureCoordsDimensionNot2(tex.len())),
				}
				Ok(obj::IndexTuple(pos_idx,Some(tex_idx),Some(norm_idx)))
			})()),//closure called here
			(None,None,None)=>None,
			_=>Some(Err(Error::VertexTripletCount)),
		}
	}).collect::<Result<Vec<obj::IndexTuple>,Error>>()?;
	let mut tuples_chunks=index_tuples.chunks_exact(3);
	let polys:Vec<_>=(&mut tuples_chunks).map(|cnk|
		obj::SimplePolygon(cnk.to_vec())
	).collect();
	//some validation
	if polys.len()!=face_count||tuples_chunks.remainder().len()!=0{
		return Err(Error::VertexCount)
	}
	Ok(obj::ObjData{
		position,
		texture,
		normal,
		objects:vec![obj::Object{
			name:String::new(),
			groups:vec![
				obj::Group{
					name:String::new(),
					index:0,
					material:None,
					polys,// <- polys data is fed into here
				}
			],
		}],
		material_libs:Vec::new(),
	})
}

fn read_bin<R:Read>(read:R)->Result<obj::ObjData,Error>{
	Ok(obj::ObjData::default())
}