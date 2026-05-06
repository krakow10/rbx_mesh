use std::io::BufRead;

#[derive(Debug)]
pub enum Error1 {
	Io(std::io::Error),
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

struct LineMachine<R: BufRead> {
	lines: std::io::Lines<R>,
}
impl<R: BufRead> LineMachine<R> {
	fn new(read: R) -> Self {
		Self {
			lines: read.lines(),
		}
	}
	fn read_line(&mut self) -> Result<String, Error1> {
		Ok(self
			.lines
			.next()
			.ok_or(Error1::UnexpectedEof)?
			.map_err(Error1::Io)?)
	}
}

#[derive(Debug, Clone)]
pub enum Revision1 {
	Version100,
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

#[inline]
pub fn fix_100(mesh: &mut Mesh1) {
	for vertex in &mut mesh.vertices {
		for p in &mut vertex.pos {
			*p = *p * 0.5;
		}
	}
}
#[inline]
pub fn fix1(mesh: &mut Mesh1) {
	for vertex in &mut mesh.vertices {
		vertex.tex[1] = 1.0 - vertex.tex[1];
	}
}
#[inline]
pub fn check1(mesh: Mesh1) -> Result<Mesh1, Error1> {
	if 3 * (mesh.header.face_count as usize) == mesh.vertices.len() {
		Ok(mesh)
	} else {
		Err(Error1::VertexCount)
	}
}

#[inline]
pub fn read_100<R: BufRead>(read: R) -> Result<Mesh1, Error1> {
	let mut mesh = read1(read)?;
	//we'll fix it in post
	fix1(&mut mesh);
	fix_100(&mut mesh);
	check1(mesh)
}

#[inline]
pub fn read_101<R: BufRead>(read: R) -> Result<Mesh1, Error1> {
	let mut mesh = read1(read)?;
	fix1(&mut mesh);
	check1(mesh)
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

//based on https://github.com/MaximumADHD/Rbx2Source/blob/main/Geometry/Mesh.cs LoadGeometry_Ascii function
pub fn read1<R: BufRead>(read: R) -> Result<Mesh1, Error1> {
	let mut lines = LineMachine::new(read);
	let revision = match lines.read_line()?.trim() {
		"version 1.00" => Revision1::Version100,
		"version 1.01" => Revision1::Version101,
		_ => return Err(Error1::Header),
	};
	let face_count = lines
		.read_line()?
		.trim()
		.parse()
		.map_err(Error1::ParseIntError)?;
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
			Ok(Vertex1 {
				pos: parse_triple_float(&c[1], &c[2], &c[3])?,
				norm: parse_triple_float(&c[4], &c[5], &c[6])?,
				tex: parse_triple_float(&c[7], &c[8], &c[9])?,
			})
		})
		.collect::<Result<Vec<Vertex1>, _>>()
		.map_err(Error1::ParseFloatError)?;

	Ok(Mesh1 { header, vertices })
}
