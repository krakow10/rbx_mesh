use binrw::{BinRead, BinReaderExt};

pub fn read_var_u32<R: BinReaderExt>(
	reader: &mut R,
	endian: binrw::Endian,
	args: (),
) -> binrw::BinResult<u32> {
	let mut result = 0;
	let mut shift = 0;
	loop {
		let byte = u8::read_options(reader, endian, args)?;
		result |= ((byte & 0b01111111) as u32) << shift;
		if byte & 0b10000000 == 0 {
			return Ok(result);
		}
		shift += 7;
	}
}
