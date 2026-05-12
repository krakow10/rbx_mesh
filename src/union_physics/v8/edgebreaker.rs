use crate::union_physics::v8::bit_stream::{BitReader, BitReaderError};

pub enum EdgebreakerError {
	BitReader(BitReaderError),
}
impl From<BitReaderError> for EdgebreakerError {
	fn from(value: BitReaderError) -> Self {
		EdgebreakerError::BitReader(value)
	}
}

// non-zero edge id
struct EdgeId(u32);

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

pub struct Hulls;

pub fn decode_clers_buffer(
	bytes: &[u8],
	bits: usize,
	hull_count: usize,
	face_count: usize,
	position_count: usize,
) -> Result<Hulls, EdgebreakerError> {
	let mut bit_reader = BitReader::new(bytes, bits)?;
	// F + V = E + 2
	let cap = (face_count + position_count - 2).max(3);
	let mut hull_state = HullState::new(cap);

	for _ in 0..hull_count {
		hull_state.clear(cap);
	}

	Ok(Hulls)
}

struct HullState {
	// adjacency[edge] = twin edge index, or one of SENTINEL_*
	adjacency: Vec<Edge>,
	// indices[edge] = vertex id at this triangle corner
	indices: Vec<u32>,
	current_triangle: u32,
	vertex_counter: u32,
}

impl HullState {
	fn new(cap: usize) -> Self {
		Self {
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
	// recursive function that matches symbols S and E like parentheses
	fn decode(
		&mut self,
		bit_reader: &mut BitReader<'_>,
		mut cursor: EdgeId,
	) -> Result<(), EdgebreakerError> {
		loop {}
	}
}
