//! Face, loop, edge, and vertex lifecycle operators + Euler primitives.
//!
//! DOMAIN: Creation, destruction, splitting, merging of individual
//! topological entities. Includes the classical Euler operators (MEF, MEV,
//! KEV, SplitEdge, etc.) that maintain the Euler formula invariant.

pub mod kill_edge_vertex;
pub mod kill_face_vertex;
pub mod kill_shell_face;
pub mod kill_vertex_edge;
pub mod kill_vertex_face;
pub mod make_edge_face;
pub mod make_edge_vertex;
pub mod make_face_vertex;
pub mod make_isolated_vertex;
pub mod make_shell_face;
pub mod make_vertex_face;
pub mod split_edge;
