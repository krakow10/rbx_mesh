use super::bit_stream::BitReader;

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
	// 1 bit
	Continue, // 0b_0
	// 3 bits
	Split, // 0b00_1
	Left,  // 0b01_1
	Right, // 0b10_1
	End,   // 0b11_1
}

pub struct SymbolReader<'a> {
	bit_reader: BitReader<'a>,
}
impl<'a> SymbolReader<'a> {
	pub fn new(bytes: &'a [u8], bits: usize) -> Self {
		let bit_reader = BitReader::new(bytes, bits);
		Self { bit_reader }
	}
}
impl<'a> Iterator for SymbolReader<'a> {
	type Item = Symbol;
	fn next(&mut self) -> Option<Self::Item> {
		if self.bit_reader.read(1)? == 0 {
			return Some(Symbol::Continue);
		}
		let bits = self.bit_reader.read(2).expect("Unexpected EOF");
		Some(match (bits & 0b10 != 0, bits & 0b01 != 0) {
			(false, false) => Symbol::Split,
			(false, true) => Symbol::Left,
			(true, false) => Symbol::Right,
			(true, true) => Symbol::End,
		})
	}
}

#[test]
fn read_symbols() {
	// R_C_S_L_E_C
	const BYTES: &[u8] = &0b101_0_001_011_111_0u16.to_le_bytes();
	let mut r = SymbolReader::new(BYTES, BYTES.len() * 8);
	// reverse order
	assert_eq!(r.next(), Some(Symbol::Continue));
	assert_eq!(r.next(), Some(Symbol::End));
	assert_eq!(r.next(), Some(Symbol::Left));
	assert_eq!(r.next(), Some(Symbol::Split));
	assert_eq!(r.next(), Some(Symbol::Continue));
	assert_eq!(r.next(), Some(Symbol::Right));
}
