//! Euler operators for topology construction and modification.
//!
//! DOMAIN: Atomic topological mutations following the Euler operator formalism.
//!
//! ## Current Operators
//! - `MakeVertexFace` (MVF): seed creation — V+1 F+1 E+1 L+1
//! - `MakeEdgeFace` (MEF): face splitting — E+1 F+1
//! - `MakeEdgeVertex` (MEV): vertex extension — V+1 E+1 (wire edge)
//! - `MakeEdgeKillLoop` (MEKL): loop merge — E+1 L-1
//! - `SplitEdge` (SE): edge subdivision — V+1 E+1
//! - `JoinFaces` (JF): face merging (inverse of MEF) — E-1 F-1
//! - `JoinFacesNmt` (JFN): NMT face merging (leaves a slit) — F-1
//! - `KillEdgeVertex` (KEV): edge/vertex collapse (inverse of MEV/SE) — V-1 E-1
//! - `KillEdgeMakeLoop` (KEML): loop split (inverse of MEKL) — E-1 L+1
//! - `SewEdge` (SEW): boundary edge gluing — E-1 χ+1
//! - `UnsewEdge` (USEW): boundary edge ungluing (inverse of SEW) — E+1
//! - `MakeShellFace` (MSF): disjoint shell creation within a solid — V+1 F+1 E+1 L+1 S+1
//! - `KillShellFace` (KSF): disjoint shell destruction (inverse of MSF) — V-1 F-1 E-1 L-1 S-1
//! - `KillVertexFace` (KVF): atomic body teardown (inverse of MVF) — V-1 F-1 E-1 L-1 S-1 So-1
//!
//! ## Missing Operators (Future Roadmap)
//!
//! INVARIANTS:
//! - Every operator is executed via `apply_op()` — never called directly
//! - Operators produce topologically valid meshes (validated on commit)
//! - Each operator returns new entity handles (never mutates in place semantically)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance), `handles` (typed IDs)

pub mod join_faces;
pub mod join_faces_nmt;
pub mod kill_edge_make_loop;
pub mod kill_edge_vertex;
pub mod kill_face_make_ring_hole;
pub mod kill_face_vertex;
pub mod kill_shell_face;
pub mod kill_vertex_edge;
pub mod kill_vertex_face;
pub mod make_edge_face;
pub mod make_edge_kill_loop;
pub mod make_edge_vertex;
pub mod make_empty_shell;
pub mod make_face_kill_ring_hole;
pub mod make_face_vertex;
pub mod make_isolated_vertex;
pub mod make_shell_face;
pub mod make_solid;
pub mod make_vertex_face;
pub mod sew_edge;
pub mod split_edge;
pub mod unsew_edge;

#[cfg(test)]
pub mod tests;

pub mod make_face_from_vertices;
pub mod make_face_in_shell_from_vertices;
pub mod make_loop_in_face_from_vertices;
pub mod make_lump_region;

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
