use binrw::BinRead;
use std::io::Cursor;

use super::Hull;

#[binrw::binread]
#[br(little)]
struct IndexSection {
	#[br(temp)]
	hull_range_count: u32,
	#[br(count = hull_range_count)]
	hull_ranges: Vec<u32>,
	// the last hull_range value gives the total index count
	#[br(count = hull_ranges.last().copied().unwrap_or(0))]
	index_base: Vec<u32>,
}

#[binrw::binread]
#[br(little)]
struct ComponentSection {
	#[br(temp)]
	vertex_range_count: u32,
	#[br(count = vertex_range_count)]
	vertex_ranges: Vec<u32>,
	#[br(count = vertex_ranges.last().copied().unwrap_or(0))]
	component_data: Vec<f32>,
}

pub fn encode_raw_hulls(hulls: &[Hull]) -> Vec<u8> {
	let mut out = Vec::new();

	// index section
	out.extend_from_slice(&(hulls.len() as u32).to_le_bytes());
	let mut cum_idx: u32 = 0;
	for hull in hulls {
		cum_idx += (hull.triangles.len() * 3) as u32;
		out.extend_from_slice(&cum_idx.to_le_bytes());
	}
	for hull in hulls {
		for tri in &hull.triangles {
			out.extend_from_slice(&tri[0].to_le_bytes());
			out.extend_from_slice(&tri[1].to_le_bytes());
			out.extend_from_slice(&tri[2].to_le_bytes());
		}
	}

	// component section
	out.extend_from_slice(&(hulls.len() as u32).to_le_bytes());
	let mut cum_comp: u32 = 0;
	for hull in hulls {
		cum_comp += (hull.vertices.len() * 3) as u32;
		out.extend_from_slice(&cum_comp.to_le_bytes());
	}
	for hull in hulls {
		for v in &hull.vertices {
			out.extend_from_slice(&v[0].to_le_bytes());
			out.extend_from_slice(&v[1].to_le_bytes());
			out.extend_from_slice(&v[2].to_le_bytes());
		}
	}

	out
}

pub fn decode_raw_hulls(data: &[u8]) -> Result<Vec<Hull>, binrw::Error> {
	if data.is_empty() {
		return Ok(Vec::new());
	}
	let mut cursor = Cursor::new(data);
	let endian = binrw::Endian::Little;

	let index_section = IndexSection::read_options(&mut cursor, endian, ())?;

	let component_section = if cursor.position() < data.len() as u64 {
		Some(ComponentSection::read_options(&mut cursor, endian, ())?)
	} else {
		None
	};

	let IndexSection {
		hull_ranges,
		index_base,
		..
	} = index_section;
	let (vertex_ranges, component_data) = match component_section {
		Some(cs) => (cs.vertex_ranges, cs.component_data),
		None => (Vec::new(), Vec::new()),
	};

	let mut hulls = Vec::with_capacity(hull_ranges.len().min(vertex_ranges.len()));
	let mut idx_start: u32 = 0;
	let mut comp_start: u32 = 0;

	for (&idx_end, &comp_end) in hull_ranges.iter().zip(vertex_ranges.iter()) {
		let comps = &component_data[comp_start as usize..comp_end as usize];
		let idxs = &index_base[idx_start as usize..idx_end as usize];

		let vertices = comps.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
		let triangles = idxs.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();

		hulls.push(Hull {
			vertices,
			triangles,
		});

		idx_start = idx_end;
		comp_start = comp_end;
	}

	Ok(hulls)
}
