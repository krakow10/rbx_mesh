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
	pub fn new(bytes: &'a [u8]) -> Self {
		let bit_reader = BitReader::new(bytes);
		Self { bit_reader }
	}
}
impl<'a> Iterator for SymbolReader<'a> {
	type Item = Symbol;
	fn next(&mut self) -> Option<Self::Item> {
		if self.bit_reader.read(1)? == 0 {
			return Some(Symbol::Continue);
		}
		Some(match self.bit_reader.read(2)? {
			0b00 => Symbol::Split,
			0b01 => Symbol::Left,
			0b10 => Symbol::Right,
			0b11 => Symbol::End,
			_ => unreachable!(),
		})
	}
}

#[test]
fn read_symbols() {
	// R_C_S_L_E_C
	const BYTES: &[u8] = &0b101_0_001_011_111_0u16.to_le_bytes();
	let mut r = SymbolReader::new(BYTES);
	// reverse order
	assert_eq!(r.next(), Some(Symbol::Continue));
	assert_eq!(r.next(), Some(Symbol::End));
	assert_eq!(r.next(), Some(Symbol::Left));
	assert_eq!(r.next(), Some(Symbol::Split));
	assert_eq!(r.next(), Some(Symbol::Continue));
	assert_eq!(r.next(), Some(Symbol::Right));
}
