type Cache = u64;

pub struct BitReader<'a> {
	chunks: core::slice::ChunksExact<'a, u8>,
	bits: usize,
	cache: Cache,
	cache_bits: usize,
}
impl<'a> BitReader<'a> {
	pub fn new(bytes: &'a [u8], bits: usize) -> Self {
		Self {
			chunks: bytes.chunks_exact(size_of::<Cache>()),
			bits,
			cache: 0,
			cache_bits: 0,
		}
	}

	pub fn read(&mut self, bits: usize) -> Option<Cache> {
		self.bits = self.bits.checked_sub(bits)?;

		let mut value = 0;
		let mut value_bits = 0;

		// popluate cache with enough bits to fill value
		while self.cache_bits + value_bits < bits {
			value += self.cache.unbounded_shl(value_bits as u32);
			value_bits += self.cache_bits;

			match self.chunks.next() {
				Some(chunk) => {
					self.cache = Cache::from_le_bytes(chunk.try_into().unwrap());
					self.cache_bits = Cache::BITS as usize;
				}
				None => {
					let mut cache = Cache::MIN;
					for (i, &byte) in self.chunks.remainder().iter().enumerate() {
						cache |= (byte as Cache) << (i * Cache::BITS as usize);
					}
					self.cache = cache;
					self.cache_bits = self.chunks.remainder().len() * Cache::BITS as usize;
				}
			};
		}

		// populate value with cached bits
		let draw_bits = bits - value_bits;
		let mask = (1 as Cache).unbounded_shl(draw_bits as u32) - 1;
		value += (self.cache & mask).unbounded_shl(value_bits as u32);
		self.cache = self.cache.unbounded_shr(draw_bits as u32);
		self.cache_bits -= draw_bits;
		Some(value)
	}
}

#[test]
fn test_read_bytes() {
	let mut r = BitReader::new(b"asdf", 32);
	assert_eq!(r.read(8), Some('a' as Cache));
	assert_eq!(r.read(8), Some('s' as Cache));
	assert_eq!(r.read(8), Some('d' as Cache));
	assert_eq!(r.read(8), Some('f' as Cache));
	// end of bytes
	assert_eq!(r.read(0), Some(0));
	assert_eq!(r.read(1), None);
}

#[test]
fn test_read_bits() {
	fn assert_s(shift: usize) {
		assert_eq!(
			BitReader::new(b"s", 32).read(shift),
			Some('s' as Cache & ((1 as Cache).unbounded_shl(shift as u32) - 1))
		);
	}
	assert_s(0);
	assert_s(1);
	assert_s(2);
	assert_s(3);
	assert_s(4);
	assert_s(5);
	assert_s(6);
	assert_s(7);
	assert_s(8);
}

#[test]
fn test_read_sequence() {
	let mut r = BitReader::new(b"asd", 24);
	// dsa 011_00100 011_1001_1 011_00_001
	assert_eq!(r.read(3), Some(0b001));
	assert_eq!(r.read(2), Some(0b00));
	assert_eq!(r.read(4), Some(0b1_011));
	assert_eq!(r.read(4), Some(0b1001));
	assert_eq!(r.read(8), Some(0b00100_011));
	assert_eq!(r.read(3), Some(0b011));
	// end of bytes
	assert_eq!(r.read(0), Some(0));
	assert_eq!(r.read(1), None);
}
