#![forbid(unsafe_code)]

pub mod arena;
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
pub use topology::integrity::{diff, hashing, healing, validate};
pub use topology::operations::algorithms;
pub use topology::operations::euler;
pub use topology::operations::operator;
pub use topology::operations::operator::EulerOperator;
pub use topology::queries::{classification, continuity, hierarchy, ordering, polygon, traverse};

#[cfg(test)]
pub mod testing;
