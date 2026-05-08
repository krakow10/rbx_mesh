#[cfg(feature = "mesh")]
pub mod mesh;
#[cfg(feature = "mesh")]
pub use mesh::read_versioned as read_mesh_versioned;

// shared code between union formats
#[cfg(any(feature = "union-graphics", feature = "union-physics"))]
mod union;

#[cfg(feature = "union-graphics")]
pub mod union_graphics;
#[cfg(feature = "union-graphics")]
pub use union_graphics::read_versioned as read_union_graphics_versioned;

#[cfg(feature = "union-physics")]
pub mod union_physics;
#[cfg(feature = "union-physics")]
pub use union_physics::read_versioned as read_union_physics_versioned;

#[cfg(test)]
mod test;

// test readme
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
