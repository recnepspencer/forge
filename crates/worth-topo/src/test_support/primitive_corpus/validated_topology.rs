use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_milestone_one_primitive_on_branch, seed_minimal_topology,
    MilestoneOnePrimitiveAuthoringError, MilestoneOnePrimitiveCase, MinimalTopologySeed,
};
use schema::facade::{MutationOrigin, VerifiedTopologyCommit};

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
) -> Result<VerifiedTopologyCommit, MilestoneOnePrimitiveAuthoringError> {
    seed_milestone_one_primitive(runtime, stem, primitive)
}

pub(crate) fn verified_primitive_on_branch(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &MilestoneOnePrimitiveCase,
    branch_id: BranchId,
    mutation_origin: MutationOrigin,
) -> Result<VerifiedTopologyCommit, MilestoneOnePrimitiveAuthoringError> {
    seed_milestone_one_primitive_on_branch(runtime, stem, primitive, branch_id, mutation_origin)
}
