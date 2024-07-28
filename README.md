Roblox Mesh Parser
==================

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
use rbx_mesh::{read_physics_data,PhysicsData,CSGPHS};

// this data is extracted from the "PhysicsData" property of UnionOperation
let file=std::fs::File::open("CSGPHS_3.data")?;
let input=std::io::BufReader::new(file);
let physics_data=read_physics_data(input)?;

match physics_data{
	// the most common format (99% of the 100000 unions in my testing)
	PhysicsData::CSGPHS(CSGPHS::Meshes3(meshes))
	// this format is identical but has a different magic number.
	|PhysicsData::CSGPHS(CSGPHS::Meshes5(meshes))=>println!("CSGPHS::Meshes"),
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
