Roblox Mesh Parser
==================

[![Latest version](https://img.shields.io/crates/v/rbx_mesh.svg)](https://crates.io/crates/rbx_mesh)
![License](https://img.shields.io/crates/l/rbx_mesh.svg)

## Mesh Example

Print the mesh vertices for any mesh version / vertex size

```rust
use rbx_mesh::{read_versioned,mesh::{VersionedMesh,SizeOfVertex2}};

let file=std::fs::File::open("torso.mesh")?;
let input=std::io::BufReader::new(file);
let versioned_mesh=read_versioned(input)?;
match versioned_mesh{
	VersionedMesh::Version1(mesh)=>println!("{:?}",mesh.vertices),
	VersionedMesh::Version2(mesh)=>{
		match mesh.header.sizeof_vertex{
			SizeOfVertex2::Full=>println!("{:?}",mesh.vertices),
			SizeOfVertex2::Truncated=>println!("{:?}",mesh.vertices_truncated),
		}
	},
	VersionedMesh::Version3(mesh)=>{
		match mesh.header.sizeof_vertex{
			SizeOfVertex2::Full=>println!("{:?}",mesh.vertices),
			SizeOfVertex2::Truncated=>println!("{:?}",mesh.vertices_truncated),
		}
	},
	VersionedMesh::Version4(mesh)=>println!("{:?}",mesh.vertices),
	VersionedMesh::Version5(mesh)=>println!("{:?}",mesh.vertices),
}
```

## Union Example
```rust
// PART 1: MeshData
use rbx_mesh::read_mesh_data_versioned;
use rbx_mesh::mesh_data::{MeshData,CSGMDL};

// this data is extracted from the "MeshData" property of UnionOperation
// the data is not usually contained in the roblox file itself
// but is sourced from the associated "AssetId" of the UnionOperation
let mesh_file=std::fs::File::open("4500696697_4.meshdata")?;
let input=std::io::BufReader::new(mesh_file);
let mesh_data=read_mesh_data_versioned(input)?;

// print mesh vertices
match mesh_data{
	MeshData::CSGK(_)=>(),
	MeshData::CSGMDL(CSGMDL::CSGMDL2(mesh_data2))=>println!("{:?}",mesh_data2.mesh.vertices),
	MeshData::CSGMDL(CSGMDL::CSGMDL4(mesh_data4))=>println!("{:?}",mesh_data4.mesh.vertices),
	// There is a branch with code for csgmdl5, but the data is not fully labeled yet
}


// PART 2: PhysicsData
use rbx_mesh::read_physics_data_versioned;
use rbx_mesh::physics_data::{PhysicsData,CSGPHS};

// this data is extracted from the "PhysicsData" property of UnionOperation
let phys_file=std::fs::File::open("CSGPHS_3.data")?;
let input=std::io::BufReader::new(phys_file);
let physics_data=read_physics_data_versioned(input)?;

match physics_data{
	// the most common format (99% of the 100000 unions in my testing)
	PhysicsData::CSGPHS(CSGPHS::Meshes(meshes))=>println!("CSGPHS::Meshes"),
	// Only one occurence in my data set.
	// Who writes a uuid as ascii hex in a binary format!?
	PhysicsData::CSGK(csgk)=>println!("CSGK"),
	// These formats have zero occurences in my dataset
	// But they are documented at
	// https://devforum.roblox.com/t/some-info-on-sharedstrings-for-custom-collision-data-meshparts-unions-etc/294588
	PhysicsData::CSGPHS(CSGPHS::Block)=>println!("CSGPHS::Block"),
	PhysicsData::CSGPHS(CSGPHS::PhysicsInfoMesh(physics_info_mesh))=>println!("CSGPHS::PhysicsInfoMesh"),
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
