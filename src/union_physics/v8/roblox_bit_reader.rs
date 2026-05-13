pub type Cache = u32;

/// Read bits from the slice in order. Bits are read as if
/// from each byte, starting from the least significant bit.
#[derive(Debug, Clone)]
pub struct BitReaderRoblox<'a> {
	chunks: core::slice::ChunksExact<'a, u8>,
	cache: Cache,
	cache_bits: usize,
}
impl<'a> BitReaderRoblox<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		Self {
			chunks: bytes.chunks_exact(size_of::<Cache>()),
			cache: 0,
			cache_bits: 0,
		}
	}
	pub fn read(&mut self, bits: usize) -> Cache {
		debug_assert!(bits <= Cache::BITS as usize);

		let mut value: Cache = 0;
		let mut value_bits = 0;

		// popluate cache with enough bits to fill value
		while self.cache_bits + value_bits < bits {
			value = value.unbounded_shl(self.cache_bits as u32) | self.cache;
			value_bits += self.cache_bits;

			match self.chunks.next() {
				Some(chunk) => {
					self.cache = Cache::from_le_bytes(chunk.try_into().unwrap());
					self.cache_bits = Cache::BITS as usize;
				}
				None => {
					let mut chunk = [0; _];
					let rem = self.chunks.remainder();
					chunk[size_of::<Cache>() - rem.len()..].copy_from_slice(rem);
					self.chunks = [].chunks_exact(size_of::<Cache>());
					self.cache = Cache::from_le_bytes(chunk);
					self.cache_bits = rem.len() * u8::BITS as usize;
				}
			};
		}

		// populate value with cached bits
		let draw_bits = bits - value_bits;
		let mask = (1 as Cache).unbounded_shl(draw_bits as u32).wrapping_sub(1);
		value = (value << draw_bits) | (self.cache >> (self.cache_bits - draw_bits));
		self.cache &= !mask.unbounded_shl((self.cache_bits - draw_bits) as u32);
		self.cache_bits -= draw_bits;
		value
	}
}
impl<'a> From<&'a [u8]> for BitReaderRoblox<'a> {
	fn from(value: &'a [u8]) -> Self {
		Self::new(value)
	}
}
