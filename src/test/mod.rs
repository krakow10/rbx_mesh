#[cfg(feature = "mesh")]
mod mesh;
#[cfg(feature = "union-graphics")]
mod union_graphics;
#[cfg(feature = "union-physics")]
mod union_physics;

use binrw::{BinRead, BinReaderExt, BinWrite, BinWriterExt};
use std::io::Cursor;

fn binread<M>(bytes: Vec<u8>) -> binrw::BinResult<(M, Cursor<Vec<u8>>)>
where
	M: std::fmt::Debug,
	M: for<'a> BinRead<Args<'a> = ()>,
{
	let bytes_len = bytes.len() as u64;

	let mut rbuf = Cursor::new(bytes);
	let mesh: M = rbuf.read_le()?;
	assert_eq!(rbuf.position(), bytes_len, "Unread data in file");

	Ok((mesh, rbuf))
}
fn binwrite<M>(mesh: &M, rbuf: Cursor<Vec<u8>>) -> binrw::BinResult<()>
where
	M: std::fmt::Debug,
	M: for<'a> BinRead<Args<'a> = ()>,
	M: for<'a> BinWrite<Args<'a> = ()>,
{
	let mut wbuf = Cursor::new(Vec::new());
	wbuf.write_le(mesh)?;

	assert_eq!(rbuf, wbuf, "Round trip failed");
	Ok(())
}

pub fn readonly<M>(bytes: Vec<u8>) -> binrw::BinResult<M>
where
	M: std::fmt::Debug,
	M: for<'a> BinRead<Args<'a> = ()>,
{
	let (mesh, _rbuf) = binread(bytes)?;
	Ok(mesh)
}
pub fn roundtrip<M>(bytes: Vec<u8>) -> binrw::BinResult<M>
where
	M: std::fmt::Debug,
	M: for<'a> BinRead<Args<'a> = ()>,
	M: for<'a> BinWrite<Args<'a> = ()>,
{
	let (mesh, rbuf) = binread(bytes)?;
	binwrite(&mesh, rbuf)?;
	Ok(mesh)
}
