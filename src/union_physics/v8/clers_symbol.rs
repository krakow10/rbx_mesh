use super::roblox_bit_reader::{BitCounterError, BitReaderRoblox};

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
	bit_reader: BitReaderRoblox<'a>,
}
impl<'a> SymbolReader<'a> {
	pub fn new(bytes: &'a [u8], bits: usize) -> Result<Self, BitCounterError> {
		let bit_reader = BitReaderRoblox::new(bytes, bits)?;
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
	// 2 C_C_R_C_S_L_E_C
	// 1 C_C_R_C_S_L_E_C
	// 4 C_C_R_C_S_L_E_C
	// 3     R_C_S_L_E_C
	// BYTES are read in the order 4,3,2,1, 8,7,6,5 msb-first
	const BYTES: [u8; 8] = [
		0b0_101_111_0,0b0_0_110_0_10,// [4, 3]
		0b0_101_111_0,0b0_0_110_0_10,// [2, 1]
		0b0_101_111_0,0b0_0_110_0_10,// [8, 7]
		0b0_101_111_0,    0b110_0_10,// [6, 5]
	];
	// truncate last word by 2 bits
	let mut r = SymbolReader::new(&BYTES, 62).unwrap();
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Right));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Split));
	assert_eq!(r.read(), Ok(Symbol::Left));
	assert_eq!(r.read(), Ok(Symbol::End));
	assert_eq!(r.read(), Ok(Symbol::Continue));

	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Right));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Split));
	assert_eq!(r.read(), Ok(Symbol::Left));
	assert_eq!(r.read(), Ok(Symbol::End));
	assert_eq!(r.read(), Ok(Symbol::Continue));

	// Truncation affects this section
	assert_eq!(r.read(), Ok(Symbol::Right));
	assert_eq!(r.read(), Ok(Symbol::Continue));
	assert_eq!(r.read(), Ok(Symbol::Split));
	assert_eq!(r.read(), Ok(Symbol::Left));
	assert_eq!(r.read(), Ok(Symbol::End));
	assert_eq!(r.read(), Ok(Symbol::Continue));

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
