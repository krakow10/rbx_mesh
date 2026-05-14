use super::bit_buffer::{BitBuffer, Cache};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BitCounterError {
	NotEnoughBytes,
	InvalidBytesLen,
	NotEnoughBits,
}
impl core::fmt::Display for BitCounterError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl core::error::Error for BitCounterError {}

const CHUNK_SIZE: usize = size_of::<Cache>();

/// Read bits in the same inconsistent manner as Roblox.
#[derive(Debug, Clone)]
pub struct BitReaderRoblox<'a> {
	chunks: core::slice::Iter<'a, [u8; CHUNK_SIZE]>,
	cache: BitBuffer,
	bit_count: usize,
}
impl<'a> BitReaderRoblox<'a> {
	pub fn new(bytes: &'a [u8], bit_count_limit: usize) -> Result<Self, BitCounterError> {
		if (bytes.len() * u8::BITS as usize) < bit_count_limit {
			return Err(BitCounterError::NotEnoughBytes);
		}
		let (chunks, rem) = (bytes.len() / CHUNK_SIZE, bytes.len() % CHUNK_SIZE);
		if rem != 0 {
			return Err(BitCounterError::InvalidBytesLen);
		}

		let chunks_ptr = bytes.as_ptr().cast();
		// SAFETY: we checked that chunks * CHUNK_SIZE == bytes.len() above
		let chunks = unsafe { core::slice::from_raw_parts(chunks_ptr, chunks) };

		Ok(Self {
			chunks: chunks.iter(),
			cache: BitBuffer::empty(),
			bit_count: bit_count_limit,
		})
	}
	pub fn read(&mut self, bits: usize) -> Result<Cache, BitCounterError> {
		debug_assert!(bits <= Cache::BITS as usize);

		// replace cache if we need more bits
		let mut value = if self.cache.bits() < bits {
			let draw_bits = self.bit_count.min(BitBuffer::CAPACITY);
			self.bit_count -= draw_bits;
			core::mem::replace(
				&mut self.cache,
				BitBuffer::new(
					self.chunks.next().copied().map_or(0, Cache::from_le_bytes),
					// bits are lsb-aligned
					draw_bits,
				),
			)
		} else {
			BitBuffer::empty()
		};

		let draw_bits = bits - value.bits();
		if self.cache.bits() < draw_bits {
			return Err(BitCounterError::NotEnoughBits);
		}

		// populate value with cached bits
		value.push_lsb(draw_bits, self.cache.pop_msb(draw_bits));
		Ok(value.value())
	}
}
