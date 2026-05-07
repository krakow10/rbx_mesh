mod bit_stream;
mod clers_symbol;

#[binrw::binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct CSGPHS8;
