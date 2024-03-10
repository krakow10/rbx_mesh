mod mesh;

#[inline]
pub fn convert<R:std::io::Read>(read:R)->Result<obj::ObjData,mesh::Error>{
    mesh::convert(read)
}