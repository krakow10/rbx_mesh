use std::io::BufRead;

#[derive(Debug)]
pub enum Error1 {
	Header,
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
impl From<std::io::Error> for InnerError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
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
	fn read_line(&mut self) -> Result<String, std::io::Error> {
		self.lines.next().ok_or(std::io::ErrorKind::UnexpectedEof)?
	}
}

#[derive(Debug, Clone)]
pub enum Revision1 {
	Version100,
	Version101,
}
impl std::str::FromStr for Revision1 {
	type Err = Error1;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"version 1.00" => Self::Version100,
			"version 1.01" => Self::Version101,
			_ => return Err(Error1::Header.into()),
		})
	}
}

#[derive(Debug, Clone)]
pub struct Vertex1 {
	pub pos: [f32; 3],
	pub norm: [f32; 3],
	pub tex: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Mesh1 {
	pub revision: Revision1,
	pub vertices: Vec<Vertex1>,
}

impl binrw::BinRead for Mesh1 {
	type Args<'a> = ();
	fn read_options<R: binrw::BinReaderExt>(
		reader: &mut R,
		_endian: binrw::Endian,
		_args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		Ok(read(binrw::io::BufReader::new(reader))?)
	}
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

fn read<R: BufRead>(reader: R) -> Result<Mesh1, InnerError> {
	let mut lines = LineMachine::new(reader);

	// the first line contains the version number
	let revision = lines.read_line()?.trim().parse()?;

	let face_count: u32 = lines
		.read_line()?
		.trim()
		.parse()
		.map_err(Error1::ParseIntError)?;

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
		.map_err(Error1::ParseFloatError)?;

	// assert vertex count matches header
	if 3 * (face_count as usize) != vertices.len() {
		return Err(Error1::VertexCount.into());
	}

	let mut mesh = Mesh1 { revision, vertices };

	// fix texture coordinates
	for vertex in &mut mesh.vertices {
		vertex.tex[1] = 1.0 - vertex.tex[1];
	}

	// mesh v1.00 is double size for some reason
	if let Revision1::Version100 = &mesh.revision {
		for vertex in &mut mesh.vertices {
			for p in &mut vertex.pos {
				*p *= 0.5;
			}
		}
	}

	Ok(mesh)
}
