//! Euler Operator primitives.
//!
//! Classical Euler topology operators for manifold B-Rep manipulation.
//! Each operator guarantees the (V − E + F = 2·S) Euler identity.
//!
//! # Lineage
//! All Euler operators accept an `OpSignature`, which tags the operation
//! in the geometric log.

pub mod join_faces_nmt;
pub mod kill_edge_make_loop;
pub mod kill_edge_vertex;
pub mod kill_face_make_ring_hole;
pub mod kill_face_vertex;
pub mod kill_vertex_edge;
pub mod make_edge_face;
pub mod make_edge_kill_loop;
pub mod make_edge_vertex;
pub mod make_face_in_shell_from_vertices;
pub mod make_face_kill_ring_hole;
pub mod make_face_vertex;
pub mod make_loop_in_face_from_vertices;
pub mod sew_edge;
pub mod split_edge;
pub mod unsew_edge;

#[cfg(test)]
pub mod tests;

// --- Re-exports from category directories ---
// These operators moved but are re-exported for backward compatibility.
pub use super::entity_lifecycle::make_vertex_face;
pub use super::entity_lifecycle::kill_vertex_face;
pub use super::entity_lifecycle::make_shell_face;
pub use super::entity_lifecycle::kill_shell_face;
pub use super::entity_lifecycle::make_isolated_vertex;
pub use super::boundary_editing::join_faces;
pub use super::boundary_editing::make_face_from_vertices;
pub use super::lifecycle::solid as make_solid;
pub use super::lifecycle::lump as make_lump_region;
pub use super::lifecycle::shell as make_empty_shell;

// --- Standard Euler Operator Aliases ---
pub type MVFS = make_vertex_face::MakeVertexFace;
pub type KVFS = kill_vertex_face::KillVertexFace;
pub type MEV = make_edge_vertex::MakeEdgeVertex;
pub type KEV = kill_edge_vertex::KillEdgeVertex;
pub type MVE = split_edge::SplitEdge;
pub type KVE = kill_vertex_edge::KillVertexEdge;
pub type MEF = make_edge_face::MakeEdgeFace;
pub type KEF = join_faces::JoinFaces;
pub type MEKR = make_edge_kill_loop::MakeEdgeKillLoop;
pub type KEMR = kill_edge_make_loop::KillEdgeMakeLoop;
pub type MFKRH = make_face_kill_ring_hole::MakeFaceKillRingHole;
pub type KFMRH = kill_face_make_ring_hole::KillFaceMakeRingHole;
pub type MFV = make_face_vertex::MakeFaceVertex;
pub type KFV = kill_face_vertex::KillFaceVertex;

