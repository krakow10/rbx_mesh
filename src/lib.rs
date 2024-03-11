mod mesh;
#[cfg(test)]
mod test;

#[inline]
pub fn read<R:std::io::Read+std::io::Seek>(read:R)->Result<mesh::VersionedMesh,mesh::Error>{
	mesh::read(read)
}