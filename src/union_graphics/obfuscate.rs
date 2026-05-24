use std::io::{Read, Seek, Write};

const OBFUSCATION_NOISE_CYCLE_XOR: [u8; 31] = [
	86, 46, 110, 88, 49, 32, 48, 4, 52, 105, 12, 119, 12, 1, 94, 0, 26, 96, 55, 105, 29, 82, 43, 7,
	79, 36, 89, 101, 83, 4, 122,
];
fn reversible_obfuscate(offset: u64, buf: &mut [u8]) {
	const LEN: u64 = OBFUSCATION_NOISE_CYCLE_XOR.len() as u64;
	for (i, b) in buf.iter_mut().enumerate() {
		*b ^= OBFUSCATION_NOISE_CYCLE_XOR[((offset + i as u64) % LEN) as usize];
	}
}

pub struct Obfuscator<R> {
	inner: R,
}
impl<R> Obfuscator<R> {
	pub fn new(read: R) -> Self {
		Self { inner: read }
	}
}
impl<R: Read + Seek> Read for Obfuscator<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let pos = self.inner.stream_position()?;
		let read_amount = self.inner.read(buf)?;
		reversible_obfuscate(pos, &mut buf[..read_amount]);
		Ok(read_amount)
	}
}
impl<R: Write + Seek> Write for Obfuscator<R> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		// avoiding allocation in Read was fortunate, but not possible here
		let mut copy = buf.to_owned();
		let pos = self.inner.stream_position()?;
		reversible_obfuscate(pos, &mut copy);
		self.inner.write(&copy)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}
impl<R: Seek> Seek for Obfuscator<R> {
	fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
		self.inner.seek(pos)
	}
}
