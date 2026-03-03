//! Public façade for shared kernel operations.
//!
//! DOMAIN: Cross-cutting atomic operations consumed by all feature domains
//! (primitives, booleans, Euler operators, NURBS tessellation).
//!
//! External components depend ONLY on this façade — internal modules
//! (`placement/vertex.rs`, `mesh_building/`, etc.) remain hidden.

// Vertex placement
pub use super::placement::vertex::{place_vertex, place_vertex_exact, PlacementRegistry};

// Mesh building
pub(crate) use super::mesh_building::cell_to_mesh::{insert_faces_and_loops, stitch_twins};
pub use super::mesh_building::containment::{make_solid_hierarchy, SolidHierarchy};
