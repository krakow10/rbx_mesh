mod mesh;

#[inline]
pub fn convert<R:std::io::Read>(read:R)->Result<obj::ObjData,mesh::Error>{
	mesh::convert(read)
}

#[cfg(test)]
mod test{
	use super::*;
	fn load_mesh(name:&str)->Result<obj::ObjData,mesh::Error>{
		convert(std::fs::File::open(name).map_err(mesh::Error::Io)?)
	}
	#[test]
	fn mesh_100(){
		load_mesh("meshes/158071912").unwrap();
	}
}