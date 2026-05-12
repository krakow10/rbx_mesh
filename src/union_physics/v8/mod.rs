mod bit_stream;
mod clers_symbol;
mod edgebreaker;

#[binrw::binrw]
#[brw(little)]
#[brw(magic = b"CSGPHS\x08\0\0\0")]
#[derive(Debug, Clone)]
pub struct CSGPHS8;
