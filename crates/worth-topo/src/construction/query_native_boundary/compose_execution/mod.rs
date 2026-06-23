mod coverage;
mod error;
mod evidence;
mod execution;
mod family_programs;
mod obligation_registration;
mod program;
mod touched_basis;

pub use error::TopologyPrimitiveConstructionBirthComposeExecutionError;
pub use evidence::{
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
};
#[cfg(test)]
pub(crate) use execution::execute_primitive_construction_birth_compose;
pub use execution::{
    run_primitive_construction_birth_declared_touched_basis_compose,
    topology_primitive_construction_birth_graph_authority_proof,
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthGraphAuthorityProof,
};
pub(crate) use obligation_registration::topology_primitive_construction_birth_layout_violation_registration;
pub use obligation_registration::{
    topology_primitive_construction_birth_graph_obligation_registration,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
pub use program::TopologyPrimitiveConstructionBirthComposeProgram;
pub use touched_basis::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis;

#[cfg(test)]
#[path = "../compose_execution_tests/mod.rs"]
mod tests;
pub use coverage::{
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthTopologyKind,
};
