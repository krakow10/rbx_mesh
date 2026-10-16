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

#[derive(Clone, Debug, Eq, PartialEq)]
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
			_ => return Err(Error1::Header),
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
		let mut buf = String::new();
		reader.read_to_string(&mut buf)?;
		Ok(from_str(&buf)?)
	}
}

fn io_error<T>(s: Option<T>) -> Result<T, std::io::Error> {
	Ok(s.ok_or(std::io::ErrorKind::UnexpectedEof)?)
}

fn parse_triple_float(x: &str, y: &str, z: &str) -> Result<[f32; 3], std::num::ParseFloatError> {
	Ok([x.trim().parse()?, y.trim().parse()?, z.trim().parse()?])
}

fn parse_vertex(str: &mut &str) -> Result<Vertex1, InnerError> {
	let (_, rest) = io_error(str.split_once('['))?;
	let (px, rest) = io_error(rest.split_once(','))?;
	let (py, rest) = io_error(rest.split_once(','))?;
	let (pz, rest) = io_error(rest.split_once("]["))?;
	let pos = parse_triple_float(px, py, pz).map_err(Error1::ParseFloatError)?;

	let (nx, rest) = io_error(rest.split_once(','))?;
	let (ny, rest) = io_error(rest.split_once(','))?;
	let (nz, rest) = io_error(rest.split_once("]["))?;
	let norm = parse_triple_float(nx, ny, nz).map_err(Error1::ParseFloatError)?;

	let (tx, rest) = io_error(rest.split_once(','))?;
	let (ty, rest) = io_error(rest.split_once(','))?;
	let (tz, rest) = io_error(rest.split_once(']'))?;
	let tex = parse_triple_float(tx, ty, tz).map_err(Error1::ParseFloatError)?;

	// update cursor
	*str = rest;

	Ok(Vertex1 { pos, norm, tex })
}

fn from_str(str: &str) -> Result<Mesh1, InnerError> {
	let mut lines = str.lines();

	// the first line contains the version number
	let revision = io_error(lines.next())?.trim().parse()?;

	let face_count: u32 = io_error(lines.next())?
		.trim()
		.parse()
		.map_err(Error1::ParseIntError)?;

	let mut vertices_line = io_error(lines.next())?;
	let mut vertices = Vec::with_capacity(face_count as usize * 3);
	for _ in 0..3 * face_count {
		vertices.push(parse_vertex(&mut vertices_line)?);
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
