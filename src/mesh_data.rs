use std::io::{Read,Seek,Write};
use binrw::{BinReaderExt, parser, BinResult};

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
#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CSGMDL5 {
	#[brw(magic = b"\x15\x7d\x29\x15\x75\x6c\x35\x04\x34\x69")]

	vertex_count: u16,
	#[br(count = vertex_count)]
	pub vertices: Vec<[f32; 3]>,

	#[br(parse_with = parse_normals)]
	pub normals: Vec<[f32; 3]>,

	vert_col_count: u16,
	#[br(count = vert_col_count)]
	pub vert_cols: Vec<(u8, u8, u8, u8)>, // pulling "colors" out of thin air but might be accurate

	normal_id_count: u16,
	#[br(count = normal_id_count)]
	pub normal_ids: Vec<NormalId5>,

	flat_xz_count: u16, // repeated vertex position data, but it's {x,-z} instead!
	#[br(count = flat_xz_count)]
	pub flat_xz: Vec<[f32; 2]>,

	#[br(parse_with = parse_normals)]
	pub tangents: Vec<[f32; 3]>,

	#[br(parse_with = parse_indices)] // flat list of vertex ids. ranges of this array are specified by the following range_markers
	pub indices: Vec<u32>, // in the portion of the array before 'n' (below), the vertex indices are normal. beyond this bit 23 is flipped

	range_marker_count: u8,
	#[br(count = range_marker_count)]
	pub range_markers: Vec<u32>, // always 3 vals? 0,n,len(indices)
}

#[parser(reader, endian)]
fn parse_normals() -> BinResult<Vec<[f32; 3]>> {
	
	const SCALE15: f32 = 1.0 / 32_767.0; // ? ok
	
	let count: u16 = reader.read_le()?;
	let blob_len: u32 = reader.read_le()?;
	let expected = count as u32 * 6;

	let mut out = Vec::with_capacity(count as usize);
	for _ in 0..count {
		let x: u16 = reader.read_le()?;
		let y: u16 = reader.read_le()?;
		let z: u16 = reader.read_le()?;
		out.push([
			(x.wrapping_sub(0x7FFF) as f32) * SCALE15,
			(y.wrapping_sub(0x7FFF) as f32) * SCALE15,
			(z.wrapping_sub(0x7FFF) as f32) * SCALE15,
		]);
	}
	if blob_len > expected {
		reader.seek_relative((blob_len - expected) as i64)?;
	}
	Ok(out)
}

#[parser(reader, endian)]
fn parse_indices() -> BinResult<Vec<u32>> { // delta encoding with variable-length integers
	let total: u32 = reader.read_le()?;
	let blob_size: u32 = reader.read_le()?;
	let mut blob = vec![0u8; blob_size as usize];
	reader.read_exact(&mut blob)?;

	let mut out = Vec::with_capacity(total as usize);
	let mut acc = 0u32;
	let mut off = 0usize;
	while out.len() < total as usize {
		let b0 = blob[off];
		off += 1;
		let delta = if (b0 & 0x80) == 0 {
			b0 as u32
		} else {
			let b1 = blob[off];
			let b2 = blob[off + 1];
			off += 2;
			(((b0 & 0x7F) as u32) << 16) | ((b1 as u32) << 8) | b2 as u32
		};
		acc = acc.wrapping_add(delta);
		out.push(acc);
	}
	Ok(out)
}

#[binrw::binrw]
#[brw(little)]
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
			MeshData::CSGMDL(CSGMDL::V5(mesh_data5))=>mesh_data5.write_options(writer,endian,args),
		}
	}
}
