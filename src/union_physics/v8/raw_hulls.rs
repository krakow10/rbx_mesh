use binrw::BinReaderExt;
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

pub fn decode_raw_hulls(data: &[u8]) -> Result<Vec<Hull>, binrw::Error> {
	let mut cursor = Cursor::new(data);

	let IndexSection {
		hull_ranges,
		index_base,
	} = cursor.read_le()?;

	let ComponentSection {
		vertex_ranges,
		component_data,
	} = cursor.read_le()?;

	let mut idx_start: u32 = 0;
	let mut comp_start: u32 = 0;

	let hulls = hull_ranges
		.into_iter()
		.zip(vertex_ranges)
		.map(|(idx_end, comp_end)| {
			let comps = &component_data[comp_start as usize..comp_end as usize];
			let idxs = &index_base[idx_start as usize..idx_end as usize];

			let vertices = comps
				.chunks_exact(3)
				.map(|c| c.try_into().unwrap())
				.collect();
			let triangles = idxs
				.chunks_exact(3)
				.map(|c| c.try_into().unwrap())
				.collect();

			idx_start = idx_end;
			comp_start = comp_end;

			Hull {
				vertices,
				triangles,
			}
		})
		.collect();

	Ok(hulls)
}
