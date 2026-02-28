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

// --- Standard Euler Operator Aliases ---
// Operators that have moved to category directories are aliased from there.
pub type MEV = make_edge_vertex::MakeEdgeVertex;
pub type KEV = kill_edge_vertex::KillEdgeVertex;
pub type MVE = split_edge::SplitEdge;
pub type KVE = kill_vertex_edge::KillVertexEdge;
pub type MEF = make_edge_face::MakeEdgeFace;
pub type MEKR = make_edge_kill_loop::MakeEdgeKillLoop;
pub type KEMR = kill_edge_make_loop::KillEdgeMakeLoop;
pub type MFKRH = make_face_kill_ring_hole::MakeFaceKillRingHole;
pub type KFMRH = kill_face_make_ring_hole::KillFaceMakeRingHole;
pub type MFV = make_face_vertex::MakeFaceVertex;
pub type KFV = kill_face_vertex::KillFaceVertex;
