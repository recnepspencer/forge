//! # forge-topo
//!
//! Halfedge mesh data structure, Euler operators, and epoch-versioned
//! topology state for the Forge geometry kernel.
//!
//! ## Architecture
//!
//! This crate owns the topological representation of solids.
//! Key design principles:
//!
//! - **Typed handles** ([`handles`]): `FaceId`, `HalfEdgeId`, `VertexId` are
//!   distinct types — passing the wrong one is a compile error
//! - **Immutable state** ([`state`]): `TopologyState` is never mutated in place.
//!   All changes go through `MutableDraft`, which auto-rolls back on drop (D6)
//! - **Formalized operators** ([`operator`]): Every topology mutation implements
//!   `EulerOperator` and is executed via `apply_op()` — the single choke point
//! - **Provenance tracking** ([`lineage`]): Every entity knows where it came from
//! - **Auto-validation** ([`validate`]): `draft.commit()` checks invariants automatically

#![forbid(unsafe_code)]

pub mod handles;
pub mod lineage;
pub mod state;
pub mod arena;
pub mod operator;
pub mod validate;
pub mod classify;
pub mod ordering;
pub mod replay;
pub mod hashing;
pub mod attributes;

pub mod euler;
pub mod traverse;
pub mod diff;
pub mod brutality;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
