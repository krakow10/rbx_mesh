use std::io::BufRead;

use binrw::BinReaderExt;

pub fn read<R: BinReaderExt>(mut read: R) -> Result<Mesh1, super::Error> {
	read.read_le()
}

#[derive(Debug)]
pub enum Error1 {
	Header,
	UnexpectedEof,
	ParseIntError(std::num::ParseIntError),
	ParseFloatError(std::num::ParseFloatError),
	VertexCount,
}
impl std::fmt::Display for Error1 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for Error1 {}

enum InnerError {
	Io(std::io::Error),
	Other(Error1),
}
impl From<Error1> for InnerError {
	fn from(value: Error1) -> Self {
		Self::Other(value)
	}
}
impl From<InnerError> for binrw::Error {
	fn from(value: InnerError) -> Self {
		match value {
			InnerError::Io(error) => Self::Io(error),
			InnerError::Other(error1) => Self::Custom {
				pos: 0,
				err: Box::new(error1),
			},
		}
	}
}

struct LineMachine<R: BufRead> {
	lines: std::io::Lines<R>,
}
impl<R: BufRead> LineMachine<R> {
	fn new(read: R) -> Self {
		Self {
			lines: read.lines(),
		}
	}
	fn read_line(&mut self) -> Result<String, InnerError> {
		self.lines
			.next()
			.ok_or(InnerError::Other(Error1::UnexpectedEof))?
			.map_err(InnerError::Io)
	}
}

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub enum Revision1 {
	#[brw(magic = b"version 1.00")]
	Version100,
	#[brw(magic = b"version 1.01")]
	Version101,
}

#[derive(Debug, Clone)]
pub struct Vertex1 {
	pub pos: [f32; 3],
	pub norm: [f32; 3],
	pub tex: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Header1 {
	pub revision: Revision1,
	pub face_count: u32,
}

#[derive(Debug, Clone)]
pub struct Mesh1 {
	pub header: Header1,
	pub vertices: Vec<Vertex1>,
}

fn parse_triple_float(x: &str, y: &str, z: &str) -> Result<[f32; 3], std::num::ParseFloatError> {
	Ok([x.trim().parse()?, y.trim().parse()?, z.trim().parse()?])
}

macro_rules! lazy_regex {
	($r:literal) => {{
		use regex::Regex;
		use std::sync::LazyLock;
		static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new($r).unwrap());
		&RE
	}};
}

impl binrw::BinRead for Mesh1 {
	type Args<'a> = ();
	fn read_options<R: BinReaderExt>(
		reader: &mut R,
		endian: binrw::Endian,
		args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		let revision = Revision1::read_options(reader, endian, args)?;

		let mut lines = LineMachine::new(binrw::io::BufReader::new(reader));

		// the first line contains the revision, but we already parsed it.
		lines.read_line()?;

		let face_count = lines
			.read_line()?
			.trim()
			.parse()
			.map_err(|e| InnerError::Other(Error1::ParseIntError(e)))?;

		//final header
		let header = Header1 {
			revision,
			face_count,
		};

		let vertices_line = lines.read_line()?;

		//match three at a time, otherwise fail
		let vertex_pattern =
			lazy_regex!(r"\[(.*?),(.*?),(.*?)\]\[(.*?),(.*?),(.*?)\]\[(.*?),(.*?),(.*?)\]");

		let vertices = vertex_pattern
			.captures_iter(vertices_line.as_str())
			.map(|c| {
				let (_, [px, py, pz, nx, ny, nz, tx, ty, tz]) = c.extract();
				Ok(Vertex1 {
					pos: parse_triple_float(px, py, pz)?,
					norm: parse_triple_float(nx, ny, nz)?,
					tex: parse_triple_float(tx, ty, tz)?,
				})
			})
			.collect::<Result<Vec<Vertex1>, _>>()
			.map_err(|e| InnerError::Other(Error1::ParseFloatError(e)))?;

		let mut mesh = Mesh1 { header, vertices };

		// fix texture coordinates
		for vertex in &mut mesh.vertices {
			vertex.tex[1] = 1.0 - vertex.tex[1];
		}

		// mesh v1.00 is double size for some reason
		if let Revision1::Version100 = &mesh.header.revision {
			for vertex in &mut mesh.vertices {
				for p in &mut vertex.pos {
					*p *= 0.5;
				}
			}
		}

		// assert vertex count matches header
		if 3 * (mesh.header.face_count as usize) != mesh.vertices.len() {
			return Err(InnerError::Other(Error1::VertexCount).into());
		}

		Ok(mesh)
	}
}
