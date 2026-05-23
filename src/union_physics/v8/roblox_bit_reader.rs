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
pub struct RobloxBitReader<'a> {
	chunks: core::slice::Iter<'a, [u8; CHUNK_SIZE]>,
	cache: BitBuffer,
	bit_count: u32,
}
impl<'a> RobloxBitReader<'a> {
	pub fn new(bytes: &'a [u8], bit_count_limit: u32) -> Result<Self, BitCounterError> {
		if (bytes.len() as u32 * u8::BITS) < bit_count_limit {
			return Err(BitCounterError::NotEnoughBytes);
		}

		let (chunks, rem) = bytes.as_chunks();
		if !rem.is_empty() {
			return Err(BitCounterError::InvalidBytesLen);
		}

		Ok(Self {
			chunks: chunks.iter(),
			cache: BitBuffer::empty(),
			bit_count: bit_count_limit,
		})
	}
	pub const fn remaining_bits(&self) -> u32 {
		self.bit_count + self.cache.bits()
	}
	pub fn read(&mut self, bits: u32) -> Result<Cache, BitCounterError> {
		debug_assert!(bits <= Cache::BITS);
		let mut value = BitBuffer::empty();

		// replace cache if we need more bits
		if self.cache.bits() < bits {
			
			// populate value with remaining bits of cache
			let pre_read_bits = self.cache_bits();
			value.push_lsb(pre_read_bits, self.cache.pop_msb(pre_read_bits));
			
			// bits are lsb-aligned
			let draw_bits = BitBuffer::CAPACITY.min(self.bit_count);
			self.bit_count -= draw_bits;
			
			let new_cache = BitBuffer::new(
				self.chunks.next().copied().map_or(0, Cache::from_le_bytes),
				draw_bits,
			);
			core::mem::replace(
				&mut self.cache,
				new_cache,
			);
		}

		let draw_bits = bits - value.bits();
		if self.cache.bits() < draw_bits {
			return Err(BitCounterError::NotEnoughBits);
		}

		// populate value with cached bits
		value.push_lsb(draw_bits, self.cache.pop_msb(draw_bits));
		Ok(value.value())
	}
}
