#![forbid(unsafe_code)]

pub mod arena;
pub mod topology;
pub mod prelude;

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
    TopologyState, 
    MutableDraft,
    FaceId, VertexId, HalfEdgeId, LoopId,
};

pub use topology::operations::operator::EulerOperator;
pub use topology::operations::operator;
pub use topology::operations::euler;
pub use topology::queries::{traverse, classify, ordering};
pub use topology::history::replay;
pub use topology::integrity::{diff, validate, hashing, healing};
pub use topology::attributes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
