use super::bit_stream::BitReaderError;
use super::clers_symbol::{Symbol, SymbolReader};

pub enum EdgebreakerError {
	BitReader(BitReaderError),
	NotEnoughBits,
}
impl From<BitReaderError> for EdgebreakerError {
	fn from(value: BitReaderError) -> Self {
		EdgebreakerError::BitReader(value)
	}
}

pub struct PosId(pub u32);
pub struct Hull {
	pub triangles: Vec<[PosId; 3]>,
}

pub fn decode_clers_buffer(
	bytes: &[u8],
	bits: usize,
	hull_count: usize,
	face_count: usize,
	position_count: usize,
) -> Result<Vec<Hull>, EdgebreakerError> {
	let symbol_reader = SymbolReader::new(bytes, bits)?;
	// F + V = E + 2
	let cap = (face_count + position_count - 2).max(3);
	let mut hull_state = HullState::new(symbol_reader, cap);

	let hulls = (0..hull_count)
		.map(|h| {
			hull_state.clear(cap);
			hull_state.decode(EdgeId(1))?;
			let triangles = Vec::new();
			Ok(Hull { triangles })
		})
		.collect::<Result<_, EdgebreakerError>>()?;

	Ok(hulls)
}

// non-zero edge id
#[derive(Clone, Copy)]
struct EdgeId(u32);
impl EdgeId {
	fn idx(self) -> usize {
		let EdgeId(id) = self;
		id as usize
	}
	fn next(self) -> Self {
		let EdgeId(id) = self;
		Self(id / 3 * 3 + id.wrapping_add(1) % 3)
	}
	fn prev(self) -> Self {
		let EdgeId(id) = self;
		Self(id / 3 * 3 + id.wrapping_sub(1) % 3)
	}
}

enum EdgeSentinel {
	Uninit,
	Boundary,
	Processing,
}

enum EdgeMeaning {
	Sentinel(EdgeSentinel),
	Adjacency(EdgeId),
	Invalid,
}

#[derive(Clone, Copy)]
struct Edge(i32);
impl Edge {
	const UNINIT: Self = Edge(-3);
	const BOUNDARY: Self = Edge(-1);
	const PROCESSING: Self = Edge(-2);
	fn meaning(&self) -> EdgeMeaning {
		match self {
			UNINIT => EdgeMeaning::Sentinel(EdgeSentinel::Uninit),
			BOUNDARY => EdgeMeaning::Sentinel(EdgeSentinel::Boundary),
			PROCESSING => EdgeMeaning::Sentinel(EdgeSentinel::Processing),
			&Edge(id) if id.is_positive() => EdgeMeaning::Adjacency(EdgeId(id as u32)),
			_ => EdgeMeaning::Invalid,
		}
	}
}
impl From<EdgeId> for Edge {
	fn from(EdgeId(id): EdgeId) -> Self {
		Self(id as i32)
	}
}

struct HullState<'a> {
	symbol_reader: SymbolReader<'a>,
	// adjacency[edge] = twin edge index, or one of SENTINEL_*
	adjacency: Vec<Edge>,
	// indices[edge] = vertex id at this triangle corner
	indices: Vec<u32>,
	current_triangle: u32,
	vertex_counter: u32,
}

impl<'a> HullState<'a> {
	fn new(symbol_reader: SymbolReader<'a>, cap: usize) -> Self {
		Self {
			symbol_reader,
			adjacency: Vec::with_capacity(cap),
			indices: Vec::with_capacity(cap),
			current_triangle: 0,
			vertex_counter: 2,
		}
	}
	fn clear(&mut self, cap: usize) {
		self.adjacency.clear();
		self.adjacency
			.extend_from_slice(&[Edge::BOUNDARY, Edge::UNINIT, Edge::BOUNDARY]);
		self.adjacency.resize(cap, Edge::UNINIT);
		self.indices.clear();
		self.indices.extend_from_slice(&[0, 1, 2]);
		self.indices.resize(cap, 0);
		self.current_triangle = 0;
		self.vertex_counter = 2;
	}
	fn zip_boundary(&mut self, mut cursor: EdgeId) {}
	// recursive function that matches symbols S and E like parentheses
	fn decode(&mut self, mut cursor: EdgeId) -> Result<(), EdgebreakerError> {
		loop {
			self.current_triangle += 1;
			let current_triangle = self.current_triangle;

			let current_edge_0 = EdgeId(3 * current_triangle);
			let current_edge_1 = EdgeId(3 * current_triangle + 1);
			let current_edge_2 = EdgeId(3 * current_triangle + 2);
			self.adjacency[current_edge_0.idx()] = cursor.into();
			self.adjacency[current_edge_1.idx()] = Edge::UNINIT;
			self.adjacency[current_edge_2.idx()] = Edge::UNINIT;

			self.adjacency[cursor.idx()] = current_edge_0.into();

			self.indices[current_edge_1.idx()] = self.indices[cursor.prev().idx()];
			self.indices[current_edge_2.idx()] = self.indices[cursor.next().idx()];

			cursor = current_edge_0.next();

			let symbol = self
				.symbol_reader
				.next()
				.ok_or(EdgebreakerError::NotEnoughBits)?;

			match symbol {
				Symbol::Continue => {
					self.vertex_counter += 1;
					self.indices[current_edge_0.idx()] = self.vertex_counter;
					self.adjacency[cursor.next().idx()] = Edge::BOUNDARY;
				}
				Symbol::Split => {
					self.decode(cursor)?;
					cursor = cursor.next();
				}
				Symbol::Left => {
					self.adjacency[cursor.idx()] = Edge::PROCESSING;
					cursor = cursor.next();
				}
				Symbol::Right => {
					let next_edge = cursor.next();
					self.adjacency[next_edge.idx()] = Edge::PROCESSING;
					self.zip_boundary(next_edge);
				}
				Symbol::End => {
					self.adjacency[cursor.idx()] = Edge::PROCESSING;
					let next_edge = cursor.next();
					self.adjacency[next_edge.idx()] = Edge::PROCESSING;
					self.zip_boundary(next_edge);
					return Ok(());
				}
			}
		}
	}
}
