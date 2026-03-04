//! Topology operations — all mutation primitives and composite algorithms.
//!
//! DOMAIN: Topology mutation through operators organized by category.
//!
//! Core infrastructure:
//! - `operator`: `TopoOperator` trait and `MutableDraft::execute()` runner
//! - `algorithms`: Compound algorithms built from operator primitives
//!
//! Category subdirectories:
//! - `lifecycle`: Body/component/lump/shell lifecycle
//! - `entity_lifecycle`: Face/loop/edge/vertex lifecycle + Euler primitives
//! - `boundary_editing`: Loop wiring, face merging, ring/hole operators
//! - `non_manifold`: Radial-edge sewing/unsewing

pub mod algorithms;
pub mod operator;

pub mod lifecycle;
pub mod entity_lifecycle;
pub mod boundary_editing;
pub mod non_manifold;

pub mod regions;
pub mod sheets_wires;
pub mod brep_coupling;
pub mod degeneracy;
pub mod boolean;
pub mod construction;
pub mod global_editing;
pub mod transform;

#[cfg(test)]
pub(crate) mod tests;

/// Standard Euler Operator type aliases.
pub type MVFS = entity_lifecycle::make_vertex_face::MakeVertexFace;
pub type KVFS = entity_lifecycle::kill_vertex_face::KillVertexFace;
pub type MEV = entity_lifecycle::make_edge_vertex::MakeEdgeVertex;
pub type KEV = entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
pub type MVE = entity_lifecycle::split_edge::SplitEdge;
pub type KVE = entity_lifecycle::kill_vertex_edge::KillVertexEdge;
pub type MEF = entity_lifecycle::make_edge_face::MakeEdgeFace;
pub type KEF = boundary_editing::join_faces::JoinFaces;
pub type MEKR = boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;
pub type KEMR = boundary_editing::kill_edge_make_loop::KillEdgeMakeLoop;
pub type MFKRH = boundary_editing::make_face_kill_ring_hole::MakeFaceKillRingHole;
pub type KFMRH = boundary_editing::kill_face_make_ring_hole::KillFaceMakeRingHole;
pub type MFV = entity_lifecycle::make_face_vertex::MakeFaceVertex;
pub type KFV = entity_lifecycle::kill_face_vertex::KillFaceVertex;
