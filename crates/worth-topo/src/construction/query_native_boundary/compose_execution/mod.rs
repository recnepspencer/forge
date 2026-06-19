mod coverage;
mod error;
mod evidence;
mod execution;
mod family_programs;
mod obligation_registration;
mod program;

pub use error::TopologyPrimitiveConstructionBirthComposeExecutionError;
pub use evidence::{
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
};
pub use execution::{
    execute_primitive_construction_birth_compose,
    TopologyPrimitiveConstructionBirthComposeExecution,
};
pub(crate) use obligation_registration::topology_primitive_construction_birth_layout_violation_registration;
pub use obligation_registration::{
    topology_primitive_construction_birth_graph_obligation_registration,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
pub use program::TopologyPrimitiveConstructionBirthComposeProgram;

#[cfg(test)]
#[path = "../compose_execution_tests/mod.rs"]
mod tests;
pub use coverage::{
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthTopologyKind,
};
