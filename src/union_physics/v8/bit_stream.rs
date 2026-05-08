type Cache = u8;

pub struct BitReader<'a> {
	bytes: &'a [u8],
	cache: Cache,
	cache_bits: usize,
}
impl<'a> BitReader<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		Self {
			bytes,
			cache: 0,
			cache_bits: 0,
		}
	}

	pub fn read(&mut self, bits: usize) -> Option<Cache> {
		let mut value = 0;
		let mut value_bits = 0;

		// popluate cache with enough bits to fill value
		while self.cache_bits + value_bits < bits {
			value += self.cache.unbounded_shl(value_bits as u32);
			value_bits += self.cache_bits;

			let (before, after) = self.bytes.split_at_checked(size_of::<Cache>())?;
			self.bytes = after;
			let array: &[u8; size_of::<Cache>()] = before.try_into().unwrap();
			self.cache = Cache::from_le_bytes(*array);
			self.cache_bits = Cache::BITS as usize;
		}

		// populate value with cached bits
		let draw_bits = bits - value_bits;
		let mask = 1u8.unbounded_shl(draw_bits as u32) - 1;
		value += (self.cache & mask).unbounded_shl(value_bits as u32);
		self.cache = self.cache.unbounded_shr(draw_bits as u32);
		self.cache_bits -= draw_bits;
		Some(value)
	}
}

#[test]
fn test_read_bytes() {
	let mut r = BitReader::new(b"asdf");
	assert_eq!(r.read(8), Some('a' as u8));
	assert_eq!(r.read(8), Some('s' as u8));
	assert_eq!(r.read(8), Some('d' as u8));
	assert_eq!(r.read(8), Some('f' as u8));
	// end of bytes
	assert_eq!(r.read(0), Some(0));
	assert_eq!(r.read(1), None);
}

#[test]
fn test_read_bits() {
	fn assert_s(shift: usize) {
		assert_eq!(
			BitReader::new(b"s").read(shift),
			Some('s' as u8 & (1u8.unbounded_shl(shift as u32) - 1))
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
	let mut r = BitReader::new(b"asd");
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
