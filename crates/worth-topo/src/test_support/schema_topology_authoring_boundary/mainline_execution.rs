use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::RawTopologyIntent;
use schema::facade::topology_authoring::{
    commit_topology_intent as commit_seeded_topology_intent, TopologyIntentCommitError,
};

use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;

fn commit_topology_intent_through_schema_authority(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<TopologyCommitCertificationInput, TopologyIntentCommitError> {
    let seeded = commit_seeded_topology_intent(runtime, intent)?;
    Ok(TopologyCommitCertificationInput::from_seeded_commit(seeded))
}

pub(crate) fn commit_topology_intent_through_schema_execution(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<TopologyCommitCertificationInput, TopologyIntentCommitError> {
    commit_topology_intent_through_schema_authority(runtime, intent)
}
