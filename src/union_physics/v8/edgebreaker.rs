use std::fmt;

#[derive(Debug, Clone)]
pub struct Hull {
	pub vertices: Vec<[f32; 3]>,
	// 0 based indices into vertices
	pub triangles: Vec<[u32; 3]>,
}

#[derive(Debug)]
pub enum EdgebreakerError {
	BitstreamUnderflow,
	BitstreamTruncated,
	HullDecodeFailed { hull: u32 },
	VertexOutOfRange { hull: u32 },
}
impl fmt::Display for EdgebreakerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{self:?}")
	}
}
impl core::error::Error for EdgebreakerError {}

const SENTINEL_UNINIT: i32 = -3;
const SENTINEL_BOUNDARY: i32 = -1;
const SENTINEL_PROCESSING: i32 = -2;

struct BitReader<'a> {
	words: &'a [u32],
	bit_pos: u32,
	total_bits: u32,
}
impl<'a> BitReader<'a> {
	fn new(words: &'a [u32], total_bits: u32) -> Result<Self, EdgebreakerError> {
		if (total_bits.div_ceil(32) as usize) > words.len() {
			return Err(EdgebreakerError::BitstreamTruncated);
		}
		Ok(Self {
			words,
			bit_pos: 0,
			total_bits,
		})
	}

	#[inline]
	fn read_bit(&mut self) -> Result<u32, EdgebreakerError> {
		if self.bit_pos >= self.total_bits {
			return Err(EdgebreakerError::BitstreamUnderflow);
		}
		let word_idx = (self.bit_pos / 32) as usize;
		let bit_in_word = self.bit_pos % 32;
		// roblox quirk: msb-first across full words, lsb-aligned in the last partial word
		let bits_in_this_word = (self.total_bits - (word_idx as u32) * 32).min(32);
		let shift = bits_in_this_word - bit_in_word - 1;
		self.bit_pos += 1;
		Ok((self.words[word_idx] >> shift) & 1)
	}
}

// rotate within the triangle's 3-edge slot (mod-3): wrap step is -2
#[inline]
fn next_offset(edge: i32) -> i32 {
	if edge.rem_euclid(3) < 2 {
		1
	} else {
		-2
	}
}
#[inline]
fn prev_offset(edge: i32) -> i32 {
	if edge.rem_euclid(3) > 0 {
		1
	} else {
		-2
	}
}

struct HullState {
	// adjacency[edge] = twin edge index, or one of SENTINEL_*
	adjacency: Vec<i32>,
	// indices[edge] = vertex id at this triangle corner
	indices: Vec<u32>,
	current_triangle: u32,
	vertex_counter: u32,
}
impl HullState {
	fn new(capacity: usize) -> Self {
		let cap = capacity.max(3);
		let mut adjacency = vec![SENTINEL_UNINIT; cap];
		let mut indices = vec![0u32; cap];
		// implicit first triangle: indices [0,1,2], outer edges marked boundary, middle edge (1) left as -3 so the decoder starts walking from it
		indices[0] = 0;
		indices[1] = 1;
		indices[2] = 2;
		adjacency[0] = SENTINEL_BOUNDARY;
		adjacency[2] = SENTINEL_BOUNDARY;
		Self {
			adjacency,
			indices,
			current_triangle: 0,
			vertex_counter: 2,
		}
	}

	fn ensure_capacity(&mut self, idx: usize) {
		if idx >= self.adjacency.len() {
			let new_size = (idx + 1).next_power_of_two();
			self.adjacency.resize(new_size, SENTINEL_UNINIT);
			self.indices.resize(new_size, 0);
		}
	}
}

fn zip_boundary(state: &mut HullState, cursor_in: i32) -> i32 {
	let mut current_edge = cursor_in;

	// loop while a SENTINEL_PROCESSING edge still needs to be paired
	// inf loop if bad format
	while state.adjacency[current_edge as usize] == SENTINEL_PROCESSING {
		let next_off = next_offset(current_edge);
		let mut candidate_edge = current_edge + next_off;

		// walk the fan via twin+next until we reach a boundary edge
		// inf loop if bad format
		while state.adjacency[candidate_edge as usize] >= 0 {
			let opposite_edge = state.adjacency[candidate_edge as usize];
			let next_of_opposite = next_offset(opposite_edge);
			candidate_edge = opposite_edge + next_of_opposite;
		}

		if state.adjacency[candidate_edge as usize] != SENTINEL_BOUNDARY {
			break;
		}

		// link the two boundary edges as twins (zip them shut)
		state.adjacency[current_edge as usize] = candidate_edge;
		state.adjacency[candidate_edge as usize] = current_edge;

		let prev_off = prev_offset(current_edge);
		current_edge -= prev_off;

		let mut prev_edge = current_edge;
		let cand_prev_off = prev_offset(candidate_edge);
		let prev_cand_edge = candidate_edge - cand_prev_off;

		// rewrite the merged corner with the surviving (donor) vertex id
		let prev_of_current = prev_offset(current_edge);
		state.indices[(current_edge - prev_of_current) as usize] =
			state.indices[prev_cand_edge as usize];

		// propagate that vertex id around the rest of the merged fan
		let mut connected_edge = state.adjacency[current_edge as usize];
		// inf loop if bad format
		while connected_edge >= 0 && candidate_edge != prev_edge {
			let prev_of_connected = prev_offset(connected_edge);
			prev_edge = connected_edge - prev_of_connected;

			let prev_of_prev = prev_offset(prev_edge);
			state.indices[(prev_edge - prev_of_prev) as usize] =
				state.indices[prev_cand_edge as usize];

			connected_edge = state.adjacency[prev_edge as usize];
		}

		// hop along the connected fan to the next still-unzipped edge
		// inf loop if bad format
		while state.adjacency[current_edge as usize] >= 0 && current_edge != candidate_edge {
			let next_link = state.adjacency[current_edge as usize];
			let prev_of_next = prev_offset(next_link);
			current_edge = next_link - prev_of_next;
		}
	}

	current_edge
}

fn decode_recursive(
	state: &mut HullState,
	bits: &mut BitReader,
	cursor_in: i32,
) -> Result<bool, EdgebreakerError> {
	let mut cursor_edge = cursor_in;

	loop {
		// inf loop / stack overflow if bad format
		// emit a new triangle and glue its edge 0 to cursor_edge as twins;
		// edges 1 and 2 inherit the corner vertices from the gate edge
		state.current_triangle += 1;
		let tri_idx = state.current_triangle;
		let tri_base_edge = (3 * tri_idx) as i32;
		state.ensure_capacity((tri_base_edge + 2) as usize);
		state.adjacency[tri_base_edge as usize] = SENTINEL_UNINIT;
		state.adjacency[(tri_base_edge + 1) as usize] = SENTINEL_UNINIT;
		state.adjacency[(tri_base_edge + 2) as usize] = SENTINEL_UNINIT;

		state.adjacency[cursor_edge as usize] = tri_base_edge;
		state.adjacency[tri_base_edge as usize] = cursor_edge;

		let prev_off = prev_offset(cursor_edge);
		let next_off = next_offset(cursor_edge);
		state.indices[(tri_base_edge + 1) as usize] =
			state.indices[(cursor_edge - prev_off) as usize];
		state.indices[(tri_base_edge + 2) as usize] =
			state.indices[(cursor_edge + next_off) as usize];

		cursor_edge = tri_base_edge + 1;

		// CLERS decoding
		let bit = bits.read_bit()?;
		if bit == 0 {
			// C: introduce new vertex
			state.vertex_counter += 1;
			state.indices[tri_base_edge as usize] = state.vertex_counter;
			let no = next_offset(cursor_edge);
			state.adjacency[(cursor_edge + no) as usize] = SENTINEL_BOUNDARY;
		} else {
			let b2 = bits.read_bit()? != 0;
			let b3 = bits.read_bit()? != 0;
			match (b2, b3) {
				(false, false) => {
					// S: split
					if !decode_recursive(state, bits, cursor_edge)? {
						return Ok(false);
					}
					cursor_edge += next_offset(cursor_edge);
				}
				(false, true) => {
					// L: turn left
					state.adjacency[cursor_edge as usize] = SENTINEL_PROCESSING;
					cursor_edge += next_offset(cursor_edge);
				}
				(true, false) => {
					// R: turn right
					let no = next_offset(cursor_edge);
					let next_edge = cursor_edge + no;
					state.adjacency[next_edge as usize] = SENTINEL_PROCESSING;
					zip_boundary(state, next_edge);
				}
				(true, true) => {
					// E: end
					state.adjacency[cursor_edge as usize] = SENTINEL_PROCESSING;
					let no = next_offset(cursor_edge);
					let next_edge = cursor_edge + no;
					state.adjacency[next_edge as usize] = SENTINEL_PROCESSING;
					zip_boundary(state, next_edge);
					return Ok(true);
				}
			}
		}
	}
}

pub fn decode_edgebreaker_hulls(
	clers: &[u32],
	clers_bit_count: u32,
	hull_count: u32,
	vertices: &[[f32; 3]],
	total_faces: u32,
) -> Result<Vec<Hull>, EdgebreakerError> {
	let mut bits = BitReader::new(clers, clers_bit_count)?;
	let mut hulls = Vec::with_capacity(hull_count as usize);
	let mut global_vert_offset: usize = 0;
	let est_capacity = (total_faces.saturating_mul(3) as usize).max(1024);

	for h in 0..hull_count {
		let mut state = HullState::new(est_capacity);
		let success = decode_recursive(&mut state, &mut bits, 1)?;
		if !success {
			return Err(EdgebreakerError::HullDecodeFailed { hull: h });
		}

		let triangle_capacity = (state.current_triangle as usize) + 1;
		let mut triangles = Vec::with_capacity(triangle_capacity);
		let mut max_local_idx: u32 = 0;
		for t in 0..=state.current_triangle {
			let base = (3 * t) as usize;
			let i0 = state.indices[base];
			let i1 = state.indices[base + 1];
			let i2 = state.indices[base + 2];
			if i0 != i1 && i0 != i2 && i1 != i2 {
				triangles.push([i0, i1, i2]);
				max_local_idx = max_local_idx.max(i0).max(i1).max(i2);
			}
		}

		let verts_consumed = (max_local_idx as usize) + 1;
		let end = global_vert_offset + verts_consumed;
		if end > vertices.len() {
			return Err(EdgebreakerError::VertexOutOfRange { hull: h });
		}
		let hull_vertices = vertices[global_vert_offset..end].to_vec();
		global_vert_offset = end;

		hulls.push(Hull {
			vertices: hull_vertices,
			triangles,
		});
	}

	Ok(hulls)
}
