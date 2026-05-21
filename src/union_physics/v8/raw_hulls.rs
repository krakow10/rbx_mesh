use super::Hull;

#[binrw::binread]
#[br(little)]
#[derive(Clone, Debug, Default)]
pub struct RawHulls {
	#[br(temp)]
	face_range_count: u32,
	#[br(count = face_range_count)]
	pub face_ranges: Vec<u32>,
	// the last hull_range value gives the total index count
	#[br(count = face_ranges.last().copied().unwrap_or(0))]
	pub faces: Vec<u32>,
	#[br(temp)]
	pos_range_count: u32,
	#[br(count = pos_range_count)]
	pub pos_ranges: Vec<u32>,
	#[br(count = pos_ranges.last().copied().unwrap_or(0))]
	pub positions: Vec<f32>,
}
impl RawHulls {
	pub fn iter_hulls(&self) -> impl ExactSizeIterator<Item = Hull<'_>> {
		self.face_ranges
			.array_windows()
			.zip(self.pos_ranges.array_windows())
			.map(move |(&[face_start, face_end], &[pos_start, pos_end])| {
				let positions = &self.positions[pos_start as usize..pos_end as usize];
				let faces = &self.faces[face_start as usize..face_end as usize];

				let (positions, _) = positions.as_chunks();
				let (faces, _) = faces.as_chunks();

				Hull { positions, faces }
			})
	}
}
