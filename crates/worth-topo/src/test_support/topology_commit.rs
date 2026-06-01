use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::RawTopologyIntent;
pub(crate) use schema::facade::topology_authoring::TopologyIntentCommitError;
use schema::facade::topology_authoring::{
    commit_topology_intent as commit_seeded_topology_intent,
    commit_topology_intent_on_branch as commit_seeded_topology_intent_on_branch,
};

use crate::committed_artifact::TopologyCommittedArtifact;

pub(crate) fn commit_topology_intent(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<TopologyCommittedArtifact, TopologyIntentCommitError> {
    let seeded = commit_seeded_topology_intent(runtime, intent)?;
    Ok(TopologyCommittedArtifact::from_seeded_commit(seeded))
}

pub(crate) fn commit_topology_intent_on_branch(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
    branch_id: BranchId,
) -> Result<TopologyCommittedArtifact, TopologyIntentCommitError> {
    let seeded = commit_seeded_topology_intent_on_branch(runtime, intent, branch_id)?;
    Ok(TopologyCommittedArtifact::from_seeded_commit(seeded))
}
