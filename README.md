Roblox Mesh Parser
==================

## Example

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