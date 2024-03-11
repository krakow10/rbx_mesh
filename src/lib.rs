mod mesh;

#[inline]
pub fn read<R:std::io::Read>(read:R)->Result<mesh::VersionedMesh,mesh::Error>{
	mesh::read(read)
}

#[cfg(test)]
mod test{
	use super::*;
	fn load_mesh(name:&str)->Result<mesh::VersionedMesh,mesh::Error>{
		read(std::fs::File::open(name).map_err(mesh::Error::Io)?)
	}
	#[test]
	fn mesh_100(){
		load_mesh("meshes/158071912").unwrap();
	}
}