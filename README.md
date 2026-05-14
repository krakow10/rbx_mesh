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

let file=std::fs::read("meshes/torso.mesh")?;
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
# binrw::BinResult::Ok(())
```

## Union Graphics Example
```rust
use rbx_mesh::read_union_graphics_versioned;
use rbx_mesh::union_graphics::UnionGraphics;

// this data is extracted from the "MeshData" property of UnionOperation
// the data is not usually contained in the roblox file itself
// but is sourced from the associated "AssetId" of the UnionOperation
let mesh_file=std::fs::read("meshes/4500696697_4.meshdata")?;
let mesh=read_union_graphics_versioned(std::io::Cursor::new(mesh_file))?;

// print mesh vertices or vertex positions
match mesh{
	UnionGraphics::CSGK(_)=>(),
	UnionGraphics::V2(mesh2)=>println!("{:?}",mesh2.mesh.vertices),
	UnionGraphics::V4(mesh4)=>println!("{:?}",mesh4.mesh.vertices),
	UnionGraphics::V5(mesh5)=>println!("{:?}",mesh5.positions),
}
# binrw::BinResult::Ok(())
```

## Union Physics Example
```rust
use rbx_mesh::read_union_physics_versioned;
use rbx_mesh::union_physics::UnionPhysics;

// this data is extracted from the "PhysicsData" property of UnionOperation
let phys_file=std::fs::read("meshes/CSGPHS_3.data")?;
let mesh=read_union_physics_versioned(std::io::Cursor::new(phys_file))?;

// Meshes contain multiple convex hulls.
// print vertex positions of the first mesh
match mesh{
	// v3 and v5 are the same format, and the most common format
	// (99% of the 100000 unions in my testing)
	UnionPhysics::V3(mesh3)=>println!("{:?}",mesh3.meshes[0].positions),
	UnionPhysics::V5(mesh5)=>println!("{:?}",mesh5.meshes[0].positions),
	// new mesh format (2025)
	UnionPhysics::V7(mesh7)=>println!("{:?}",mesh7.meshes[0].positions),
	// Only one occurence in my data set.
	// Who writes a uuid as ascii hex in a binary format!?
	UnionPhysics::CSGK(_csgk)=>println!("CSGK"),
	// These formats have zero occurences in my dataset
	// But they are documented at
	// https://devforum.roblox.com/t/some-info-on-sharedstrings-for-custom-collision-data-meshparts-unions-etc/294588
	UnionPhysics::Block(_block)=>println!("CSGPHS Block"),
	UnionPhysics::V6(mesh6)=>println!("{:?}",mesh6.mesh.positions),
	UnionPhysics::V8(mesh8)=>println!("{:?}",mesh8.body.positions),
}
# binrw::BinResult::Ok(())
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
