## [Unreleased]

### Added

### Changed

### Removed

## [0.7.0] - May 8th 2026

### Added

- Disabling mesh-v1 feature allows regex dependency to be dropped
- mesh, union-graphics, union-physics features to disable parts of the library you don't need

### Changed

- `Mesh1` is implemented via binrw::BinRead trait to provide a consistent interface
- `Mesh2` & `Mesh3` vertices are changed to use an enum `Vertices2`
- `Mesh2` through `Mesh5` faces `Face2` use an array of `VertexId2` instead of a tuple
- `read_versioned` is renamed to `read_mesh_versioned`
- `read_mesh_data_versioned` is renamed to `read_union_graphics_versioned`
- `read_physics_data_versioned` is renamed to `read_union_physics_versioned`
- `MeshData` is renamed to `UnionGraphics`
- `PhysicsData` is renamed to `UnionPhysics`
- `CSGPHS3` mesh field `Mesh.vertices` is renamed to `Mesh.positions` for consistency with other mesh definitions
- CSGPHS & CSGMDL variants are flattened into each union enum to reduce match depth
- Magic numbers are moved onto the individual mesh structs, meaning each one acts as its own encoder/decoder and can be used independently of the unified enum.  For example, if you know ahead of time that a mesh is using the version 2.00 format, you can write `let mesh: Mesh2 = data.read_le()?;`
- count & len fields have been removed from the public structs, they are calculated based on the contained data.  For example, vertex_count is just vertices.len(), so it is calculated from the vertices Vec upon serialization.

### Removed

- Mesh v2-v5 "fix tangents" procedure that replaces Vertex2.tangents default value `[-128, -128, -128, -128]` with `[0, 0, -128, 127]`.  If you depended on this behaviour, you will need to handle this yourself.  None of the ~10000 meshes in my dataset have default tangents. If you are using old meshes and need tangents, realistically you should be generating tangents via some algorithmic process rather than filling in default values.

## [0.6.0] - March 3rd 2026

## [0.5.0] - July 23rd 2025
