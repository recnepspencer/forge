#![forbid(unsafe_code)]
// Direct float equality is banned workspace-wide. Use forge_core comparison
// predicates: approximately_equal, positions_coincident, is_effectively_zero.
#![deny(clippy::float_cmp)]

// Component modules (vertical domain slices with data/logic + façade)
pub mod b_rep;
pub mod canonical;
pub mod projection;
pub mod change_detection;
pub mod persistent_naming;
pub mod provenance;
pub mod semantic_attributes;
pub mod transactions;

// Shared infrastructure
pub mod handles;
pub mod identity;
pub mod prelude;

// Operations, queries, validators (promoted from topology/)
pub mod operations;
pub mod queries;
pub mod validators;

// Integration tests
pub(crate) mod tests;

// Convenience re-exports
pub use handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
pub use identity::{DraftId, OperationId};
pub use projection::{ProjectedTopology, ProjectionBuilder};

pub use operations::algorithms;
pub use operations::boundary_editing;
pub use operations::entity_lifecycle;
pub use operations::lifecycle;
pub use operations::non_manifold;
pub use operations::operator;
pub use queries::{classification, hierarchy, ordering, polygon, traverse};
pub use validators::validate;

#[cfg(test)]
pub mod testing;
