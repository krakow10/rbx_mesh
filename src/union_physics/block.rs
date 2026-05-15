/// This mesh is a rectangular prism, also known as a block.
#[binrw::binrw]
#[brw(little)]
// concat_bytes!(b"CSGPHS",0u32,b"BLOCK")
#[brw(magic = b"CSGPHS\0\0\0\0BLOCK")]
#[derive(Debug, Clone)]
pub struct Block;
