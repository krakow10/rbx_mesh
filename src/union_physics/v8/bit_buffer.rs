/// Buffer size for read and write
pub type Cache = u32;

#[derive(Debug, Clone)]
pub struct BitBuffer {
	buffer: Cache,
	bits: u32,
}

impl BitBuffer {
	pub const CAPACITY: u32 = Cache::BITS;
	pub const fn new(buffer: Cache, bits: u32) -> Self {
		Self { buffer, bits }
	}
	pub const fn empty() -> Self {
		Self::new(0, 0)
	}
	pub const fn value(&self) -> Cache {
		self.buffer
	}
	pub const fn bits(&self) -> u32 {
		self.bits
	}
	/// Push `bits` bits into the lsb of buffer.
	/// Assumes non-active bits in value are zeroed
	pub const fn push_lsb(&mut self, bits: u32, value: Cache) {
		// enough room for bits
		debug_assert!(self.bits + bits <= Self::CAPACITY);

		// no nasty high bits
		debug_assert!(value & !(1 as Cache).unbounded_shl(bits as u32).wrapping_sub(1) == 0);

		self.buffer = self.buffer.unbounded_shl(bits as u32) | value;
		self.bits += bits;
	}
	/// Pop `bits` bits from the msb of buffer
	pub const fn pop_msb(&mut self, bits: u32) -> Cache {
		// enough available bits
		debug_assert!(bits <= self.bits);

		let shift = self.bits - bits;
		let value = self.buffer.unbounded_shr(shift as u32);
		let mask = (1 as Cache).unbounded_shl(shift as u32).wrapping_sub(1);
		self.buffer &= mask;
		self.bits -= bits;
		value
	}
}

#[test]
fn test_fifo_lsb() {
	let mut b = BitBuffer::empty();
	b.push_lsb(8, 'a' as Cache);
	b.push_lsb(8, 's' as Cache);
	b.push_lsb(8, 'd' as Cache);
	b.push_lsb(8, 'f' as Cache);
	assert_eq!(b.pop_msb(8), 'a' as Cache);
	assert_eq!(b.pop_msb(8), 's' as Cache);
	assert_eq!(b.pop_msb(8), 'd' as Cache);
	assert_eq!(b.pop_msb(8), 'f' as Cache);
}
