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
//! - `KillEdgeVertex` (KEV): edge/vertex collapse (inverse of MEV/SE) — V-1 E-1
//! - `KillEdgeMakeLoop` (KEML): loop split (inverse of MEKL) — E-1 L+1
//! - `SewEdge` (SEW): boundary edge gluing — E-1 χ+1
//!
//! ## Missing Operators (Future Roadmap)
//! - **MakeShellFace / KillShellFace**: shell-level creation when
//!   the region/shell layer is added for 3D solid modeling.
//!
//! INVARIANTS:
//! - Every operator is executed via `apply_op()` — never called directly
//! - Operators produce topologically valid meshes (validated on commit)
//! - Each operator returns new entity handles (never mutates in place semantically)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance), `handles` (typed IDs)

pub mod make_vertex_face;
pub mod make_edge_face;
pub mod make_edge_vertex;
pub mod make_edge_kill_loop;
pub mod split_edge;
pub mod join_faces;
pub mod kill_edge_vertex;
pub mod kill_edge_make_loop;
pub mod sew_edge;

#[cfg(test)]
pub mod tests;

