use super::bit_stream::{BitReader, BitReaderError};

#[derive(Debug, Eq, PartialEq)]
pub enum SymbolError {
	NotEnoughBits,
}

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
	pub fn new(bytes: &'a [u8], bits: usize) -> Result<Self, BitReaderError> {
		let bit_reader = BitReader::new(bytes, bits)?;
		Ok(Self { bit_reader })
	}
	pub fn read(&mut self) -> Result<Symbol, SymbolError> {
		if self.bit_reader.read(1).ok_or(SymbolError::NotEnoughBits)? == 0 {
			return Ok(Symbol::Continue);
		}
		let bits = self.bit_reader.read(2).ok_or(SymbolError::NotEnoughBits)?;
		Ok(match (bits & 0b10 != 0, bits & 0b01 != 0) {
			(false, false) => Symbol::Split,
			(false, true) => Symbol::Left,
			(true, false) => Symbol::Right,
			(true, true) => Symbol::End,
		})
	}
}

#[test]
fn read_symbols() {
	// C_C_R_C_S_L_E_C
	const BYTES: &[u8] = &0b0_0_101_0_001_011_111_0u16.to_le_bytes();
	let mut r = SymbolReader::new(BYTES, BYTES.len() * 8).unwrap();
	// reverse order
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::End));
	assert_eq!(r.read(), Ok(Symbol::Left));
	assert_eq!(r.read(), Ok(Symbol::Split));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Right));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Err(SymbolError::NotEnoughBits));
}
