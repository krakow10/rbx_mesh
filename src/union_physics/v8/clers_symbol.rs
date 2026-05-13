use bit_stream::{BitCounterError, BitRead, CountedBitReaderRoblox};

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
	// 1 bit
	Continue, // 0b0
	// 3 bits
	Split, // 0b1_00
	Left,  // 0b1_01
	Right, // 0b1_10
	End,   // 0b1_11
}

pub struct SymbolReader<'a> {
	bit_reader: CountedBitReaderRoblox<'a>,
}
impl<'a> SymbolReader<'a> {
	pub fn new(bytes: &'a [u8], bits: usize) -> Result<Self, BitCounterError> {
		let bit_reader = CountedBitReaderRoblox::new_reader(bytes, bits)?;
		Ok(Self { bit_reader })
	}
	pub fn read(&mut self) -> Result<Symbol, BitCounterError> {
		if self.bit_reader.read(1)? == 0 {
			return Ok(Symbol::Continue);
		}
		let bits = self.bit_reader.read(2)?;
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
	const BYTES: [u8; 2] = 0b0_0_110_0_100_101_111_0u16.to_be_bytes();
	let mut r = SymbolReader::new(&BYTES, BYTES.len() * u8::BITS as usize).unwrap();
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Right));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Split));
	assert_eq!(r.read(), Ok(Symbol::Left));
	assert_eq!(r.read(), Ok(Symbol::End));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Err(BitCounterError::NotEnoughBits));
}
