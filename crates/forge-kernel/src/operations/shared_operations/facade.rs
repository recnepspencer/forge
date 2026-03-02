//! Public façade for shared kernel operations.
//!
//! DOMAIN: Cross-cutting atomic operations consumed by all feature domains
//! (primitives, booleans, Euler operators, NURBS tessellation).
//!
//! External components depend ONLY on this façade — internal modules
//! (`placement/vertex.rs`, etc.) remain hidden.

pub use super::placement::vertex::{place_vertex, place_vertex_exact, PlacementRegistry};
