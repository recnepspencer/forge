use forge_query::facade::ForgeQueryWorkspaceError;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, MutationIntent};
use schema::facade::topology_authoring::commit_topology_mutation_set;

pub(super) fn commit_topology_mutation_set_through_schema_runtime_boundary(
    runtime: &mut RelationalRuntime,
    transaction_label: &'static str,
    intents: Vec<MutationIntent>,
) -> Result<CommitResult, ForgeQueryWorkspaceError> {
    commit_topology_mutation_set(runtime, transaction_label, intents).map_err(|error| {
        ForgeQueryWorkspaceError::new(format!(
            "topology production runtime write commit failed: {error}"
        ))
    })
}
