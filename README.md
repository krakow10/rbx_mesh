Roblox Mesh Parser
==================

[![Latest version](https://img.shields.io/crates/v/rbx_mesh.svg)](https://crates.io/crates/rbx_mesh)
![License](https://img.shields.io/crates/l/rbx_mesh.svg)

`rbx_mesh` is a collection of deserializers for the different mesh versions in Roblox, and a function to detect the version and invoke the correct deserializer.  The meshes are decoded into a structure that reflects the on-disk format of the particular mesh version, rather than being translated into a catch-all structure.

## Mesh Example

Print the mesh vertices for any mesh version / vertex size

```rust
use rbx_mesh::read_mesh_versioned;
use rbx_mesh::mesh::{Mesh,Vertices2};

let file=std::fs::read("torso.mesh")?;
let versioned_mesh=read_mesh_versioned(std::io::Cursor::new(file))?;
match versioned_mesh{
	Mesh::V1(mesh)=>println!("{:?}",mesh.vertices),
	Mesh::V2(mesh)=>{
		match mesh.vertices{
			Vertices2::Full(vertices)=>println!("{:?}",vertices),
			Vertices2::Truncated(vertices_truncated)=>println!("{:?}",vertices_truncated),
		}
	},
	Mesh::V3(mesh)=>{
		match mesh.vertices{
			Vertices2::Full(vertices)=>println!("{:?}",vertices),
			Vertices2::Truncated(vertices_truncated)=>println!("{:?}",vertices_truncated),
		}
	},
	Mesh::V4(mesh)=>println!("{:?}",mesh.vertices),
	Mesh::V5(mesh)=>println!("{:?}",mesh.vertices),
}
```

## Union Example
```rust
// PART 1: union graphics
use rbx_mesh::read_union_graphics_versioned;
use rbx_mesh::union_graphics::{UnionGraphics,CSGMDL};

// this data is extracted from the "MeshData" property of UnionOperation
// the data is not usually contained in the roblox file itself
// but is sourced from the associated "AssetId" of the UnionOperation
let mesh_file=std::fs::read("4500696697_4.meshdata")?;
let mesh=read_union_graphics_versioned(std::io::Cursor::new(mesh_file))?;

// print mesh vertices
match mesh{
	UnionGraphics::CSGK(_)=>(),
	UnionGraphics::CSGMDL(CSGMDL::V2(mesh2))=>println!("{:?}",mesh2.mesh.vertices),
	UnionGraphics::CSGMDL(CSGMDL::V4(mesh4))=>println!("{:?}",mesh4.mesh.vertices),
	UnionGraphics::CSGMDL(CSGMDL::V5(mesh5))=>{
		// CSGMDL::V5
		let vertices:Vec<_>=mesh5
			.faces
			.indices
			.chunks_exact(3)
			.map(|face_vertex_indices|{
				// construct face triangle from indices
				[
					mesh5.positions[face_vertex_indices[0] as usize],
					mesh5.positions[face_vertex_indices[1] as usize],
					mesh5.positions[face_vertex_indices[2] as usize],
				]
			})
			.collect();
		println!("{:?}",vertices);
	},
}


// PART 2: union physics
use rbx_mesh::read_union_physics_versioned;
use rbx_mesh::union_physics::{UnionPhysics,CSGPHS};

// this data is extracted from the "PhysicsData" property of UnionOperation
let phys_file=std::fs::read("CSGPHS_3.data")?;
let mesh=read_union_physics_versioned(std::io::Cursor::new(phys_file))?;

match mesh{
	// v3 and v5 are the same format, and the most common format
	// (99% of the 100000 unions in my testing)
	UnionPhysics::CSGPHS(CSGPHS::V3(_mesh))=>println!("CSGPHS V3"),
	UnionPhysics::CSGPHS(CSGPHS::V5(_mesh))=>println!("CSGPHS V5"),
	// new mesh format (2025)
	UnionPhysics::CSGPHS(CSGPHS::V7(_mesh))=>println!("CSGPHS V7"),
	// Only one occurence in my data set.
	// Who writes a uuid as ascii hex in a binary format!?
	UnionPhysics::CSGK(_csgk)=>println!("CSGK"),
	// These formats have zero occurences in my dataset
	// But they are documented at
	// https://devforum.roblox.com/t/some-info-on-sharedstrings-for-custom-collision-data-meshparts-unions-etc/294588
	UnionPhysics::CSGPHS(CSGPHS::Block(_block))=>println!("CSGPHS Block"),
	UnionPhysics::CSGPHS(CSGPHS::V6(_mesh))=>println!("CSGPHS V6"),
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
