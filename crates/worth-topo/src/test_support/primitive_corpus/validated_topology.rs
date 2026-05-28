use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch, seed_minimal_topology,
    MilestoneOnePrimitiveAuthoringError, MilestoneOnePrimitiveCase, MinimalTopologySeed,
};
use schema::facade::platform::authority::MutationOrigin;
use crate::committed_artifact::TopologyCommittedArtifact;
#[cfg(test)]
use crate::validation::reference_integrity::{
    milestone_one_runtime_builder, MilestoneOneRuntimeSetupError,
};

#[cfg(test)]
pub(crate) fn build_test_runtime() -> Result<RelationalRuntime, MilestoneOneRuntimeSetupError> {
    Ok(milestone_one_runtime_builder()?.build())
}

pub(crate) fn seeded_bootstrap(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<MinimalTopologySeed, forge_relational::facade::transactions::TransactionCommitError> {
    seed_minimal_topology(runtime, stem)
}

pub(crate) fn verified_primitive(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
) -> Result<TopologyCommittedArtifact, MilestoneOnePrimitiveAuthoringError> {
    let verified = seed_milestone_one_primitive(runtime, stem, primitive)?;
    Ok(TopologyCommittedArtifact::from_parts(
        verified.canonical_batch,
        verified.branch_id,
        verified.commits,
        verified.persisted_truth,
        verified.read_basis,
    ))
}

pub(crate) fn verified_primitive_on_branch(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
    branch_id: BranchId,
    mutation_origin: MutationOrigin,
) -> Result<TopologyCommittedArtifact, MilestoneOnePrimitiveAuthoringError> {
    let verified = seed_milestone_one_primitive_on_branch(
        runtime,
        stem,
        primitive,
        branch_id,
        mutation_origin,
    )?;
    Ok(TopologyCommittedArtifact::from_parts(
        verified.canonical_batch,
        verified.branch_id,
        verified.commits,
        verified.persisted_truth,
        verified.read_basis,
    ))
}




