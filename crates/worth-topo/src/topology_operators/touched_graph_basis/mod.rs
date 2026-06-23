mod basis;
mod declared_basis;
mod digest;
mod entity_relation;
mod lowering;
#[cfg(test)]
mod tests;
mod vocabulary;

pub use basis::{TopologyTouchedGraphBasis, TopologyTouchedGraphCounters};
pub use declared_basis::{
    TopologyDeclaredTouchedGraphBasis, TopologyDeclaredTouchedGraphBasisProof,
};
pub use entity_relation::{TopologyTouchedEntity, TopologyTouchedRelation};
pub use lowering::topology_operator_touch_descriptor_from_touched_graph_basis;
pub(crate) use lowering::topology_touched_graph_basis_from_mutation_sequence;
#[cfg(test)]
pub(crate) use lowering::{
    topology_rewire_loop_endpoint_touched_graph_basis,
    topology_splice_radial_adjacency_touched_graph_basis,
};
pub(crate) use vocabulary::topology_touched_aspect_from_schema_aspect;
pub(crate) use vocabulary::TopologyTouchedOperatingWorldIdentityDigest;
pub(crate) use vocabulary::{
    topology_lifecycle_posture_from_mutation_family, topology_touched_scope_from_changed_scope,
};
pub use vocabulary::{
    TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedOperatingWorld,
    TopologyTouchedOperatingWorldPosture, TopologyTouchedScope,
};

trait BasisDigestPart {
    fn digest_part(&self) -> String;
}
