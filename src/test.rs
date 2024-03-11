use crate::mesh::*;
fn load_mesh(name:&str)->Result<VersionedMesh,Error>{
	read(std::fs::File::open(name).map_err(Error::Io)?)
}
#[test]
fn mesh_100(){
	match load_mesh("meshes/158071912").unwrap(){
		VersionedMesh::Version1(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_200(){
	match load_mesh("meshes/torso.mesh").unwrap(){
		VersionedMesh::Version2(_)=>(),
		_=>panic!(),
	}
}
}
#[test]
fn mesh_401(){
	match load_mesh("meshes/sphere.mesh").unwrap(){
		VersionedMesh::Version4(_)=>(),
		_=>panic!(),
	}
}