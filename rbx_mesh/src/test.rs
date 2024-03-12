use crate::mesh::*;
fn load_mesh(name:&str)->Result<VersionedMesh,Error>{
	read(std::fs::File::open(name).map_err(Error::Io)?)
}
#[test]
fn mesh_100(){
	match load_mesh("../meshes/158071912").unwrap(){
		VersionedMesh::Version1(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_200(){
	match load_mesh("../meshes/torso.mesh").unwrap(){
		VersionedMesh::Version2(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_300(){
	match load_mesh("../meshes/5115672913").unwrap(){
		VersionedMesh::Version3(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_301(){
	match load_mesh("../meshes/5648093777").unwrap(){
		VersionedMesh::Version3(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_401(){
	match load_mesh("../meshes/sphere.mesh").unwrap(){
		VersionedMesh::Version4(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_401_random_padding(){
	match load_mesh("../meshes/7665777615").unwrap(){
		VersionedMesh::Version4(_)=>(),
		_=>panic!(),
	}
}
//the only three v5.00 meshes I could find in bhop and surf
#[test]
fn mesh_500(){
	match load_mesh("../meshes/13674780763").unwrap(){
		VersionedMesh::Version5(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_500_alt1(){
	match load_mesh("../meshes/14818281896").unwrap(){
		VersionedMesh::Version5(_)=>(),
		_=>panic!(),
	}
}
#[test]
fn mesh_500_alt2(){
	match load_mesh("../meshes/15256456161").unwrap(){
		VersionedMesh::Version5(_)=>(),
		_=>panic!(),
	}
}
//also tested against ~2500 meshes from bhop and surf maps
