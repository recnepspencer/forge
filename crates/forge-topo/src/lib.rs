#![forbid(unsafe_code)]

pub mod b_rep;
pub mod arena;
pub use arena::TopologyArena;
pub mod semantic_attributes;
pub mod provenance;
pub mod persistent_naming;
pub mod change_detection;
pub mod transactions;
pub mod prelude;
pub mod topology;

pub use topology::handles;
pub use topology::history::lineage;
pub use topology::history::lineage_store;
pub use topology::state;

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
pub use topology::operations::operator;
pub use topology::operations::lifecycle;
pub use topology::operations::entity_lifecycle;
pub use topology::operations::boundary_editing;
pub use topology::operations::non_manifold;
pub use topology::queries;
pub use topology::queries::{classification, continuity, hierarchy, ordering, polygon, traverse};

#[cfg(test)]
pub mod testing;
