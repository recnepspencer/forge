//! Euler operators for topology construction and modification.
//!
//! DOMAIN: Atomic topological mutations following the Euler operator formalism.
//!
//! INVARIANTS:
//! - Every operator is executed via `apply_op()` — never called directly
//! - Operators produce topologically valid meshes (validated on commit)
//! - Each operator returns new entity handles (never mutates in place semantically)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance), `handles` (typed IDs)

pub mod make_vertex_face;
pub mod make_edge_face;
pub mod split_edge;
pub mod join_faces;
pub mod kill_edge_vertex;
pub mod bridge_edge;

#[cfg(test)]
pub mod tests;
