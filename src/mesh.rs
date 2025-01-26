use std::io::{BufRead,Read,Seek};

use binrw::BinReaderExt;

pub const DEFAULT_VERTEX_TANGENT:[i8;4]=[0,0,-128,127];

#[derive(Debug)]
pub enum Error{
	Io(std::io::Error),
	UnknownVersion(Vec<u8>),
	//1.00
	Header,
	UnexpectedEof,
	ParseIntError(std::num::ParseIntError),
	ParseFloatError(std::num::ParseFloatError),
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

#[derive(Debug,Clone)]
pub enum VersionedMesh{
	Version1(Mesh1),
	Version2(Mesh2),
	Version3(Mesh3),
	Version4(Mesh4),
	Version5(Mesh5),
	//Version6(Mesh6),
	//Version7(Mesh7),
}

pub fn read_versioned<R:Read+Seek>(read:R)->Result<VersionedMesh,Error>{
	let mut buf_reader=binrw::io::BufReader::new(read);
	let buf=buf_reader.fill_buf().map_err(Error::Io)?;
	if buf.len()<12{
		return Err(Error::Header);
	}
	match &buf[0..12]{
		b"version 1.00"=>Ok(VersionedMesh::Version1(read_100(buf_reader)?)),
		b"version 1.01"=>Ok(VersionedMesh::Version1(read_101(buf_reader)?)),
		b"version 2.00"=>Ok(VersionedMesh::Version2(read_200(buf_reader)?)),
		b"version 3.00"
		|b"version 3.01"=>Ok(VersionedMesh::Version3(read_300(buf_reader)?)),
		b"version 4.00"
		|b"version 4.01"=>Ok(VersionedMesh::Version4(read_400(buf_reader)?)),
		b"version 5.00"=>Ok(VersionedMesh::Version5(read_500(buf_reader)?)),
		//b"version 6.00"=>Ok(VersionedMesh::Version6(read_600(buf_reader)?)),
		//b"version 7.00"=>Ok(VersionedMesh::Version7(read_700(buf_reader)?)),
		other=>Err(Error::UnknownVersion(other.to_vec())),
	}
}

struct LineMachine<R:BufRead>{
	lines:std::io::Lines<R>,
}
impl<R:BufRead> LineMachine<R>{
	fn new(read:R)->Self{
		Self{
			lines:read.lines(),
		}
	}
	fn read_line(&mut self)->Result<String,Error>{
		Ok(self.lines.next().ok_or(Error::UnexpectedEof)?.map_err(Error::Io)?)
	}
}

#[derive(Debug,Clone)]
pub enum Revision1{
	Version100,
	Version101,
}
#[derive(Debug,Clone)]
pub struct Vertex1{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub tex:[f32;3],
}
#[derive(Debug,Clone)]
pub struct Header1{
	pub revision:Revision1,
	pub face_count:u32,
}
#[derive(Debug,Clone)]
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
pub fn read_100<R:BufRead>(read:R)->Result<Mesh1,Error>{
	let mut mesh=read1(read)?;
	//we'll fix it in post
	fix1(&mut mesh);
	fix_100(&mut mesh);
	check1(mesh)
}

#[inline]
pub fn read_101<R:BufRead>(read:R)->Result<Mesh1,Error>{
	let mut mesh=read1(read)?;
	fix1(&mut mesh);
	check1(mesh)
}

fn parse_triple_float(x:&str,y:&str,z:&str)->Result<[f32;3],std::num::ParseFloatError>{
	Ok([
		x.trim().parse()?,
		y.trim().parse()?,
		z.trim().parse()?,
	])
}

//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs LoadGeometry_Ascii function
pub fn read1<R:BufRead>(read:R)->Result<Mesh1,Error>{
	let mut lines=LineMachine::new(read);
	let revision=match lines.read_line()?.trim(){
		"version 1.00"=>Revision1::Version100,
		"version 1.01"=>Revision1::Version101,
		_=>return Err(Error::Header),
	};
	let face_count=lines.read_line()?.trim().parse().map_err(Error::ParseIntError)?;
	let vertices_line=lines.read_line()?;
	//match three at a time, otherwise fail
	let vertex_pattern=lazy_regex::regex!(r"\[(.*?),(.*?),(.*?)\]\[(.*?),(.*?),(.*?)\]\[(.*?),(.*?),(.*?)\]");
	Ok(Mesh1{
		header:Header1{
			revision,
			face_count,
		},
		vertices:vertex_pattern.captures_iter(vertices_line.as_str()).map(|c|
			Ok(Vertex1{
				pos:parse_triple_float(&c[1],&c[2],&c[3])?,
				norm:parse_triple_float(&c[4],&c[5],&c[6])?,
				tex:parse_triple_float(&c[7],&c[8],&c[9])?,
			})
		).collect::<Result<Vec<Vertex1>,_>>().map_err(Error::ParseFloatError)?
	})
}

//the rest is based on https://devforum.roblox.com/t/roblox-mesh-format/326114
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Revision2{
	#[brw(magic=b"2.00")]
	Version200,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum SizeOfVertex2{
	#[brw(magic=36u8)]
	Truncated,
	#[brw(magic=40u8)]
	Full,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Header2{
	#[brw(magic=b"version ")]
	pub revision:Revision2,
	#[brw(magic=b"\n\x0C\0")]//newline,sizeof_header
	//sizeof_header:u16,//12=0x000C
	pub sizeof_vertex:SizeOfVertex2,
	#[brw(magic=b"\x0C")]//sizeof_face
	//sizeof_face:u8,//12=0x0C
	pub vertex_count:u32,
	pub face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Vertex2{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub tex:[f32;2],
	pub tangent:[i8;4],// Tangent Vector & Bi-Normal Direction
	pub color:[u8;4],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Vertex2Truncated{
	pub pos:[f32;3],
	pub norm:[f32;3],
	pub tex:[f32;2],
	pub tangent:[i8;4],// Tangent Vector & Bi-Normal Direction
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone,Eq,Hash,PartialEq)]
pub struct VertexId2(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Face2(pub VertexId2,pub VertexId2,pub VertexId2);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
/// Only one of {vertices,vertices_truncated} is populated based on header.sizeof_vertex
pub struct Mesh2{
	pub header:Header2,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Full=>header.vertex_count,_=>0})]
	pub vertices:Vec<Vertex2>,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Truncated=>header.vertex_count,_=>0})]
	pub vertices_truncated:Vec<Vertex2Truncated>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
}

#[inline]
pub fn fix2(mesh:&mut Mesh2){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=DEFAULT_VERTEX_TANGENT,
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
	read.read_le().map_err(Error::BinRead)
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Revision3{
	#[brw(magic=b"3.00")]
	Version300,
	#[brw(magic=b"3.01")]
	Version301,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Header3{
	#[brw(magic=b"version ")]
	pub revision:Revision3,
	#[brw(magic=b"\n\x10\0")]//newline,sizeof_header
	//sizeof_header:u16,//16=0x0010
	pub sizeof_vertex:SizeOfVertex2,
	#[brw(magic=b"\x0C\x04\0")]//sizeof_face,sizeof_lod
	//sizeof_face:u8,//12=0x0C
	//sizeof_lod:u16,//4=0x0004
	pub lod_count:u16,
	pub vertex_count:u32,
	pub face_count:u32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone,Eq,Hash,PartialEq)]
/// Lods are indices into faces, representing the start of the range of
/// faces to be drawn for a particular level of detail, with the end of
/// the range represented by the next id in the list.
pub struct Lod3(pub u32);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
/// Only one of {vertices,vertices_truncated} is populated based on header.sizeof_vertex
pub struct Mesh3{
	pub header:Header3,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Full=>header.vertex_count,_=>0})]
	pub vertices:Vec<Vertex2>,
	#[br(count=match header.sizeof_vertex{SizeOfVertex2::Truncated=>header.vertex_count,_=>0})]
	pub vertices_truncated:Vec<Vertex2Truncated>,
	#[br(count=header.face_count)]
	pub faces:Vec<Face2>,
	#[br(count=header.lod_count)]
	pub lods:Vec<Lod3>,
}

#[inline]
pub fn fix3(mesh:&mut Mesh3){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=DEFAULT_VERTEX_TANGENT,
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
	read.read_le().map_err(Error::BinRead)
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Revision4{
	#[brw(magic=b"4.00")]
	Version400,
	#[brw(magic=b"4.01")]
	Version401,
}
#[binrw::binrw]
#[brw(little,repr=u16)]
#[derive(Debug,Clone)]
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
#[derive(Debug,Clone)]
pub struct Header4{
	#[brw(magic=b"version ")]
	pub revision:Revision4,
	#[brw(magic=b"\n\x18\0")]//newline,sizeof_header
	//sizeof_header:u16,//24
	pub lod_type:LodType4,
	pub vertex_count:u32,
	pub face_count:u32,
	pub lod_count:u16,
	pub bone_count:u16,
	pub bone_names_len:u32,
	pub subset_count:u16,
	pub lod_hq_count:u8,
	pub _padding:u8,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Envelope4{
	pub bones:[u8;4],
	pub weights:[u8;4],
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone,Eq,Hash,PartialEq)]
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
#[derive(Debug,Clone)]
pub struct CFrame4{
	pub r00:f32,pub r01:f32,pub r02:f32,
	pub r10:f32,pub r11:f32,pub r12:f32,
	pub r20:f32,pub r21:f32,pub r22:f32,
	pub x:f32,pub y:f32,pub z:f32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Bone4{
	pub bone_name_pos:u32,
	pub parent:BoneId4,
	pub lod_parent:BoneId4,
	pub cull_distance:f32,
	pub cframe:CFrame4,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
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
#[derive(Debug,Clone)]
/// envelopes has the same length as vertices when header.bone_count!=0
pub struct Mesh4{
	pub header:Header4,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=if header.bone_count==0{0}else{header.vertex_count})]
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

#[inline]
pub fn fix4(mesh:&mut Mesh4){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=DEFAULT_VERTEX_TANGENT,
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
	read.read_le().map_err(Error::BinRead)
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub enum Revision5{
	#[brw(magic=b"5.00")]
	Version500,
}
#[binrw::binrw]
#[brw(little,repr=u32)]
#[derive(Debug,Clone)]
pub enum FacsFormat5{
	Format1=1,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Header5{
	#[brw(magic=b"version ")]
	pub revision:Revision5,
	#[brw(magic=b"\n\x20\0")]//newline,sizeof_header
	//sizeof_header:u16,//32=0x0020
	pub lod_type:LodType4,
	pub vertex_count:u32,
	pub face_count:u32,
	pub lod_count:u16,
	pub bone_count:u16,
	pub bone_names_len:u32,
	pub subset_count:u16,
	pub lod_hq_count:u8,
	pub _padding:u8,
	pub facs_format:FacsFormat5,
	pub sizeof_facs:u32,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
/// Quantized means interpolated from lerp0 to lerp1 based on [0-65535]
pub enum QuantizedMatrix5{
	#[brw(magic=1u16)]
	Raw{
		x:u32,
		y:u32,
		#[br(count=x*y)]
		matrix:Vec<f32>,
	},
	#[brw(magic=2u16)]
	Quantized{
		x:u32,
		y:u32,
		lerp0:f32,
		lerp1:f32,
		#[br(count=x*y)]
		matrix:Vec<u16>,
	},
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct QuantizedTransforms5{
	pub px:QuantizedMatrix5,
	pub py:QuantizedMatrix5,
	pub pz:QuantizedMatrix5,
	pub rx:QuantizedMatrix5,
	pub ry:QuantizedMatrix5,
	pub rz:QuantizedMatrix5,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone,Eq,Hash,PartialEq)]
pub struct ControlId5(pub u16);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct TwoPoseCorrective5(pub ControlId5,pub ControlId5);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct ThreePoseCorrective5(pub ControlId5,pub ControlId5,pub ControlId5);
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
pub struct Facs5{
	pub face_bone_names_len:u32,
	pub face_control_names_len:u32,
	pub quantized_transforms_len:u64,
	pub two_pose_correctives_len:u32,
	pub three_pose_correctives_len:u32,
	#[br(count=face_bone_names_len)]
	pub face_bone_names:Vec<u8>,
	#[br(count=face_control_names_len)]
	pub face_control_names:Vec<u8>,
	//is this not a list?
	pub quantized_transforms:QuantizedTransforms5,
	#[br(count=two_pose_correctives_len as usize/std::mem::size_of::<TwoPoseCorrective5>())]
	pub two_pose_correctives:Vec<TwoPoseCorrective5>,
	#[br(count=three_pose_correctives_len as usize/std::mem::size_of::<ThreePoseCorrective5>())]
	pub three_pose_correctives:Vec<ThreePoseCorrective5>,
}
#[binrw::binrw]
#[brw(little)]
#[derive(Debug,Clone)]
/// envelopes has the same length as vertices when header.bone_count!=0
pub struct Mesh5{
	pub header:Header5,
	#[br(count=header.vertex_count)]
	pub vertices:Vec<Vertex2>,
	#[br(count=if header.bone_count==0{0}else{header.vertex_count})]
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
	pub facs:Facs5,
}

#[inline]
pub fn fix5(mesh:&mut Mesh5){
	for vertex in &mut mesh.vertices{
		match vertex.tangent{
			[-128,-128,-128,-128]=>vertex.tangent=DEFAULT_VERTEX_TANGENT,
			_=>(),
		}
	}
}

#[inline]
pub fn read_500<R:Read+Seek>(read:R)->Result<Mesh5,Error>{
	let mut mesh=read5(read)?;
	fix5(&mut mesh);
	Ok(mesh)
}

pub fn read5<R:BinReaderExt>(mut read:R)->Result<Mesh5,Error>{
	read.read_le().map_err(Error::BinRead)
}
