//https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs
use std::io::Read;

pub enum Error{
	Io(std::io::Error),
	Header,
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

fn read_ascii<R:Read>(read:R)->Result<obj::ObjData,Error>{
	Ok(obj::ObjData::default())
}

fn read_bin<R:Read>(read:R)->Result<obj::ObjData,Error>{
	Ok(obj::ObjData::default())
}