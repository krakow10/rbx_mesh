use crate::mesh::*;
fn load_mesh(name:&str)->Result<VersionedMesh,Error>{
	read(std::fs::File::open(name).map_err(Error::Io)?)
}
#[test]
fn mesh_100(){
	load_mesh("meshes/158071912").unwrap();
}
#[test]
fn mesh_200(){
	load_mesh("meshes/torso.mesh").unwrap();
}
#[test]
fn mesh_401(){
	load_mesh("meshes/sphere.mesh").unwrap();
}