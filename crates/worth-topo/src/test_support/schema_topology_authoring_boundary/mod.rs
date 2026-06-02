#[cfg(test)]
use forge_relational::facade::transactions::{CommitResult, MutationIntent};
#[cfg(test)]
use schema::facade::topology_authoring::commit_topology_mutation_set as commit_seeded_topology_mutation_set;
pub(crate) use schema::facade::topology_authoring::TopologyIntentCommitError;
#[cfg(test)]
pub(crate) use schema::facade::topology_authoring::TopologyMutationSetCommitError;

mod branch_execution;
mod mainline_execution;
mod primitive_seeding;

#[cfg(test)]
pub(crate) use self::branch_execution::empty_branch_local_commit_input_through_schema_execution;
pub(crate) use self::branch_execution::{
    commit_topology_intent_on_branch_through_schema_execution,
    open_schema_topology_authoring_branch_execution,
    witness_rejected_branch_local_intent_through_schema_execution,
    SchemaTopologyAuthoringBranchExecutionLedger,
};
pub(crate) use self::mainline_execution::commit_topology_intent_through_schema_execution;
pub(crate) use self::primitive_seeding::{
    seed_milestone_one_primitive_in_new_branch_through_schema_execution,
    seed_milestone_one_primitive_through_schema_execution,
    seed_minimal_topology_through_schema_execution, SchemaBranchPrimitiveAuthoringError,
    SchemaMinimalTopologySeedWitness, SchemaPrimitiveAuthoringError,
};

#[cfg(test)]
pub(crate) fn commit_topology_mutation_set_through_schema_execution(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    transaction_label: &'static str,
    intents: impl IntoIterator<Item = MutationIntent>,
) -> Result<CommitResult, TopologyMutationSetCommitError> {
    commit_seeded_topology_mutation_set(runtime, transaction_label, intents)
}
