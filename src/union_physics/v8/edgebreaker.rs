use super::clers_symbol::{Symbol, SymbolReader};
use super::roblox_bit_reader::BitCounterError;

#[derive(Debug, Clone)]
pub struct Hull<'f> {
	/// 0 based indices into vertices
	pub faces: &'f [[u32; 3]],
}

// non-negative edge id
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EdgeId(u32);
impl EdgeId {
	const fn idx(self) -> usize {
		let EdgeId(id) = self;
		id as usize
	}
	// rotate within the triangle's 3-edge slot (mod-3)
	const fn next(self) -> Self {
		let EdgeId(id) = self;
		// floor group and rotate id +1
		Self(id / 3 * 3 + (id + 1) % 3)
	}
	const fn prev(self) -> Self {
		let EdgeId(id) = self;
		// floor group and rotate id -1
		// +2 is used here to avoid underflow when id = 0
		Self(id / 3 * 3 + (id + 2) % 3)
	}
}
#[test]
fn test_edge_id() {
	assert_eq!(EdgeId(3).next(), EdgeId(4));
	assert_eq!(EdgeId(4).next(), EdgeId(5));
	assert_eq!(EdgeId(5).next(), EdgeId(3));

	assert_eq!(EdgeId(5).prev(), EdgeId(4));
	assert_eq!(EdgeId(4).prev(), EdgeId(3));
	assert_eq!(EdgeId(3).prev(), EdgeId(5));
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
	fn meaning(self) -> EdgeMeaning {
		match self {
			Edge(id) if 0 <= id => EdgeMeaning::Adjacency(EdgeId(id as u32)),
			Edge(-3) => EdgeMeaning::Sentinel(EdgeSentinel::Uninit),
			Edge(-1) => EdgeMeaning::Sentinel(EdgeSentinel::Boundary),
			Edge(-2) => EdgeMeaning::Sentinel(EdgeSentinel::Processing),
			_ => EdgeMeaning::Invalid,
		}
	}
}
impl From<EdgeId> for Edge {
	fn from(EdgeId(id): EdgeId) -> Self {
		Self(id as i32)
	}
}

pub struct HullDecoder<'a> {
	symbol_reader: SymbolReader<'a>,
	// adjacency[edge] = twin edge index, or one of SENTINEL_*
	adjacency: Box<[Edge]>,
	// indices[edge] = vertex id at this triangle corner
	indices: Box<[u32]>,
	current_triangle: u32,
	vertex_count: u32,
}

impl<'a> HullDecoder<'a> {
	pub fn new(symbol_reader: SymbolReader<'a>, cap: usize) -> Self {
		Self {
			symbol_reader,
			adjacency: vec![Edge::UNINIT; cap].into_boxed_slice(),
			indices: vec![0; cap].into_boxed_slice(),
			current_triangle: 0,
			vertex_count: 0,
		}
	}
	fn zip_boundary(&mut self, mut current_edge: EdgeId) -> EdgeId {
		// loop while a SENTINEL_PROCESSING edge still needs to be paired
		// inf loop if bad format
		while let EdgeMeaning::Sentinel(EdgeSentinel::Processing) =
			self.adjacency[current_edge.idx()].meaning()
		{
			let mut candidate_edge = current_edge.next();

			// walk the fan via twin+next until we reach a boundary edge
			// inf loop if bad format
			while let EdgeMeaning::Adjacency(opposite_edge) =
				self.adjacency[candidate_edge.idx()].meaning()
			{
				candidate_edge = opposite_edge.next();
			}

			if !matches!(
				self.adjacency[candidate_edge.idx()].meaning(),
				EdgeMeaning::Sentinel(EdgeSentinel::Boundary)
			) {
				break;
			}

			// link the two boundary edges as twins (zip them shut)
			self.adjacency[current_edge.idx()] = candidate_edge.into();
			self.adjacency[candidate_edge.idx()] = current_edge.into();

			current_edge = current_edge.prev();

			let mut prev_edge = current_edge;
			let candidate_prev_edge = candidate_edge.prev();

			// rewrite the merged corner with the surviving (donor) vertex id
			self.indices[current_edge.prev().idx()] = self.indices[candidate_prev_edge.idx()];

			// propagate that vertex id around the rest of the merged fan
			let mut connected_edge = self.adjacency[current_edge.idx()];
			// inf loop if bad format
			while let EdgeMeaning::Adjacency(connected_edge_id) = connected_edge.meaning()
				&& candidate_edge != prev_edge
			{
				prev_edge = connected_edge_id.prev();
				self.indices[prev_edge.prev().idx()] = self.indices[candidate_prev_edge.idx()];
				connected_edge = self.adjacency[prev_edge.idx()];
			}

			// hop along the connected fan to the next still-unzipped edge
			// inf loop if bad format
			while let EdgeMeaning::Adjacency(linked_edge) =
				self.adjacency[current_edge.idx()].meaning()
				&& current_edge != candidate_edge
			{
				current_edge = linked_edge.prev();
			}
		}

		current_edge
	}
	// recursive function that matches symbols S and E like parentheses
	fn decode_recursive(&mut self, mut cursor: EdgeId) -> Result<(), BitCounterError> {
		loop {
			// emit a new triangle and glue its edge 0 to cursor_edge as twins;
			// edges 1 and 2 inherit the corner vertices from the gate edge
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

			cursor = current_edge_1;

			let symbol = self.symbol_reader.read()?;

			match symbol {
				Symbol::Continue => {
					self.indices[current_edge_0.idx()] = self.vertex_count;
					self.adjacency[cursor.next().idx()] = Edge::BOUNDARY;
					self.vertex_count += 1;
				}
				Symbol::Split => {
					self.decode_recursive(cursor)?;
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
	pub fn decode_hull(&mut self) -> Result<Hull<'_>, BitCounterError> {
		let start = self.current_triangle as usize;
		let edge = 3 * start as usize;
		self.adjacency[edge..edge + 3].copy_from_slice(&[
			Edge::BOUNDARY,
			Edge::UNINIT,
			Edge::BOUNDARY,
		]);
		let vertex_id = self.vertex_count;
		self.indices[edge..edge + 3].copy_from_slice(&[
			vertex_id + 0,
			vertex_id + 1,
			vertex_id + 2,
		]);
		self.vertex_count += 3;

		self.decode_recursive(EdgeId(edge as u32 + 1))?;

		// `self.vertex_count -= 1` feels wrong.  Are we really sure this is generating meshes correctly?
		self.vertex_count -= 1;

		let end = self.current_triangle as usize + 1;

		let (chunks, _) = self.indices.as_chunks();
		let faces = &chunks[start..end];

		Ok(Hull { faces })
	}
}
