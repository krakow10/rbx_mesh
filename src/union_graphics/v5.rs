use binrw::BinReaderExt;

use std::io::{Read, Seek};

use super::{Error, NormalIDError, NormalId};

// TODO use read_options to directly read MeshData
// instead of reading header and then seeking back
#[binrw::binrw]
#[brw(little,repr=u8)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct NormalId5(pub NormalId);
impl From<&NormalId5> for u8 {
	#[inline]
	fn from(&NormalId5(value): &NormalId5) -> u8 {
		value as u8
	}
}
impl TryFrom<u8> for NormalId5 {
	type Error = NormalIDError;
	#[inline]
	fn try_from(value: u8) -> Result<NormalId5, NormalIDError> {
		Ok(NormalId5(match value {
			1 => NormalId::Right,
			2 => NormalId::Top,
			3 => NormalId::Back,
			4 => NormalId::Left,
			5 => NormalId::Bottom,
			6 => NormalId::Front,
			_ => return Err(NormalIDError),
		}))
	}
}

#[derive(Debug)]
pub enum FacesStateMachineError {
	UnexpectedEOF,
	UnusedData,
}
impl std::fmt::Display for FacesStateMachineError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl core::error::Error for FacesStateMachineError {}

#[derive(Debug, Clone)]
pub struct Faces5 {
	pub indices: Vec<u32>,
	/// Additional lists of unknown content.  Assumed to be more indices.  Possibly LODs or something.
	pub _unknown: Vec<Vec<u32>>,
}
impl binrw::BinRead for Faces5 {
	type Args<'a> = ();
	fn read_options<R: Read + Seek>(
		reader: &mut R,
		_endian: binrw::Endian,
		_args: Self::Args<'_>,
	) -> binrw::BinResult<Self> {
		// complete faces data
		#[binrw::binrw]
		#[brw(little)]
		struct Faces5Inner {
			vertex_count: u32,
			vertex_data_len: u32,
			#[br(count=vertex_data_len)]
			vertex_data: Vec<u8>,
			range_marker_count: u8,
			#[br(count=range_marker_count)]
			range_markers: Vec<u32>,
		}

		// use the stream position at the beginning of the Faces data
		let pos = reader.stream_position()?;

		fn read_state_machine(
			data: Vec<u8>,
			expected_output_count: usize,
		) -> Result<Vec<u32>, FacesStateMachineError> {
			let mut indices = Vec::with_capacity(expected_output_count);
			let mut it = data.into_iter();
			let mut index_out = 0;
			for _ in 0..expected_output_count {
				let v0 = it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
				if v0 & (1 << 7) == 0 {
					// TODO: test whether 64 goes to top or bottom case
					if v0 & (1 << 6) == 0 {
						index_out += v0 as u32;
					} else {
						// 64..127 is mapped to -64..-1
						index_out -= -((v0 | 0x80) as i8) as u32;
					}
				} else {
					let v1 = it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
					let v2 = it.next().ok_or(FacesStateMachineError::UnexpectedEOF)?;
					index_out += u32::from_le_bytes([v2, v1, v0 & 0x7F, 0]);
				}
				indices.push(index_out & 0x7FFFFF);
			}

			// iterator should be fully depleted
			if it.next().is_some() {
				return Err(FacesStateMachineError::UnusedData);
			}

			Ok(indices)
		}

		// read complete data
		let faces_inner: Faces5Inner = reader.read_le()?;

		// accumulate vertex indices using state machine
		let mut indices =
			read_state_machine(faces_inner.vertex_data, faces_inner.vertex_count as usize)
				.map_err(|e| Error::Custom {
					pos,
					err: Box::new(e),
				})?;

		// Validate markers
		{
			let mut it = faces_inner.range_markers.iter().copied().enumerate();
			if let Some((i, mut last_marker)) = it.next() {
				if indices.len() < (last_marker as usize) {
					return Err(Error::Custom {
						pos,
						err: Box::new(format!("Marker {i} (value {last_marker}) out of range")),
					});
				}
				for (i, marker) in it {
					if marker < last_marker {
						return Err(Error::Custom{
							pos,
							err:Box::new(format!("Marker {i} (value {marker}) is less than marker {} (value {last_marker})",i-1)),
						});
					}
					if indices.len() < (marker as usize) {
						return Err(Error::Custom {
							pos,
							err: Box::new(format!("Marker {i} (value {marker}) out of range")),
						});
					}
					last_marker = marker;
				}
			}
		}

		// split indices according to range marker count
		let mut it = faces_inner.range_markers.into_iter();
		let Some(marker0) = it.next() else {
			return Err(Error::Custom {
				pos,
				err: Box::new("Not enough range markers: 0"),
			});
		};
		let mut remaining_start_index = marker0;
		if marker0 != 0 {
			// drop indices at the start of the list
			indices.drain(..marker0 as usize);
		}
		let Some(marker1) = it.next() else {
			return Err(Error::Custom {
				pos,
				err: Box::new("Not enough range markers: 1"),
			});
		};
		let Some(mut marker2) = it.next() else {
			return Ok(Self {
				indices,
				_unknown: Vec::new(),
			});
		};

		// split indices according to marker points
		let mut _unknown = Vec::new();
		let mut remaining_indices = indices.split_off((marker1 - remaining_start_index) as usize);
		remaining_start_index = marker1;

		for marker in it {
			let next_remaining_indices =
				remaining_indices.split_off((marker2 - remaining_start_index) as usize);
			_unknown.push(remaining_indices);
			remaining_indices = next_remaining_indices;
			remaining_start_index = marker2;

			marker2 = marker;
		}

		// insert the final range
		if ((marker2 - remaining_start_index) as usize) < remaining_indices.len() {
			// drop indices at the end of the list
			remaining_indices.drain((marker2 - remaining_start_index) as usize..);
		}
		_unknown.push(remaining_indices);

		Ok(Self { indices, _unknown })
	}
}

#[binrw::binread]
#[br(little)]
#[br(map=Self::read)]
#[repr(transparent)]
struct QuantizedF32x3([f32; 3]);
impl QuantizedF32x3 {
	fn read([x, y, z]: [i16; 3]) -> Self {
		const SCALE: f32 = 1.0 / 32_767.0; // ? ok
		Self([
			(x.wrapping_sub(0x7FFF) as f32) * SCALE,
			(y.wrapping_sub(0x7FFF) as f32) * SCALE,
			(z.wrapping_sub(0x7FFF) as f32) * SCALE,
		])
	}
}
fn parse_quantized_f32x3_array<R: binrw::io::Read + binrw::io::Seek>(
	reader: &mut R,
	_endian: binrw::Endian,
	args: binrw::VecArgs<()>,
) -> binrw::BinResult<Vec<[f32; 3]>> {
	// read quantized i16 values directly into Vec, converting to f32 on the fly
	let quantized: Vec<QuantizedF32x3> = reader.read_le_args(args)?;
	// transmute into expected type
	// SAFETY: QuantizedF32x3 is #[repr(transparent)]
	let transmuted: Vec<[f32; 3]> = unsafe { core::mem::transmute(quantized) };
	// Equivalent safe code
	// let transmuted=quantized.into_iter().map(|QuantizedF32x3(value)|value).collect();
	Ok(transmuted)
}

#[binrw::binread]
#[br(little)]
// reversible_obfuscate(0, concat_bytes!(b"CSGMDL", 5u32))
#[br(magic = b"\x15\x7d\x29\x15\x75\x6c\x35\x04\x34\x69")]
#[derive(Debug, Clone)]
pub struct CSGMDL5 {
	pub pos_count: u16,
	#[br(count=pos_count)]
	pub positions: Vec<[f32; 3]>,

	pub normals_count: u16,
	pub normals_len: u32,
	#[br(parse_with=parse_quantized_f32x3_array,args_raw=binrw::VecArgs{count:normals_count as usize,inner:()})]
	pub normals: Vec<[f32; 3]>,

	pub color_count: u16,
	#[br(count=color_count)]
	pub colors: Vec<[u8; 4]>,

	pub normal_id_count: u16,
	#[br(count=normal_id_count)]
	pub normal_ids: Vec<NormalId5>,

	pub tex_count: u16,
	#[br(count=tex_count)]
	pub tex: Vec<[f32; 2]>,

	pub tangents_count: u16,
	pub tangents_len: u32,
	#[br(parse_with=parse_quantized_f32x3_array,args_raw=binrw::VecArgs{count:tangents_count as usize,inner:()})]
	pub tangents: Vec<[f32; 3]>,

	// delta encoded vertex indices
	pub faces: Faces5,
}
