use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;
#[cfg(test)]
use crate::validation::reference_integrity::{
    milestone_one_runtime_builder, MilestoneOneRuntimeSetupError,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use crate::test_support::schema_topology_authoring_boundary::{
    seed_milestone_one_primitive_through_schema_execution,
    seed_minimal_topology_through_schema_execution, SchemaMinimalTopologySeedWitness,
    SchemaPrimitiveAuthoringError,
};

#[cfg(test)]
pub(crate) fn build_test_runtime() -> Result<RelationalRuntime, MilestoneOneRuntimeSetupError> {
    Ok(milestone_one_runtime_builder()?.build())
}

pub(crate) fn seeded_bootstrap(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<
    SchemaMinimalTopologySeedWitness,
    forge_relational::facade::transactions::TransactionCommitError,
> {
    seed_minimal_topology_through_schema_execution(runtime, stem)
}

pub(crate) fn committed_primitive_input(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
) -> Result<TopologyCommitCertificationInput, SchemaPrimitiveAuthoringError> {
    seed_milestone_one_primitive_through_schema_execution(runtime, stem, primitive)
}
