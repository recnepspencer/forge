#![forbid(unsafe_code)]

pub mod arena;
pub use arena::TopologyArena;
pub mod prelude;
pub mod topology;

// Re-exports to maintain public API compatibility or provide shortcuts
pub use topology::handles;
pub use topology::history::lineage;
pub use topology::history::lineage_store;
pub use topology::state;
// operator was at root, now at topology::operations::operator
// but users might expect `forge_topo::operator`.
// Let's re-export `operator` from `topology::operations::operator` if possible?
// No, let's align with the new structure.

// Public re-exports from topology
pub use topology::{
    EdgeId, FaceId, HalfEdgeId, LoopId, MutableDraft, ShellId, TopologyState, VertexId,
};

pub use topology::attributes;
pub use topology::bitset;
pub use topology::history::replay;
pub use topology::validators::validate;
pub use topology::utils::{diff, hashing};
pub use topology::operations::healing::orientation::*;
pub use topology::operations::algorithms;
pub use topology::operations::euler;
pub use topology::operations::operator;
pub use topology::operations::operator::EulerOperator;
pub use topology::operations::lifecycle;
pub use topology::operations::entity_lifecycle;
pub use topology::operations::boundary_editing;
pub use topology::operations::non_manifold;
pub use topology::queries;
pub use topology::queries::{classification, continuity, hierarchy, ordering, polygon, traverse};

#[cfg(test)]
pub mod testing;
