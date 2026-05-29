use forge_harness::facade::{replay_id, ReplayRecord, ReplayRequest};

use crate::facade::harness::RelationalHarnessError;
use crate::facade::runtime::RelationalRuntime;

use super::super::data::{RelationalFixture, RelationalHarnessAdapter};
use super::replay_summary_fields::replay_summary;

pub(super) fn capture_replay(
    _adapter: &RelationalHarnessAdapter,
    runtime: &RelationalRuntime,
    fixture: &forge_harness::facade::ScenarioFixture<RelationalFixture>,
    replay: &ReplayRequest<String>,
) -> Result<ReplayRecord<String>, RelationalHarnessError> {
    let publication_artifacts = runtime.publication().artifacts().snapshot();
    let latest_replay = publication_artifacts
        .latest_replay
        .ok_or_else(|| RelationalHarnessError("no replay artifact available".to_string()))?;
    Ok(ReplayRecord {
        schema_version: forge_harness::facade::RecordSchemaVersion::V1,
        replay_id: replay_id(&replay.source_run.run_id, &replay.name),
        source_run_id: replay.source_run.run_id.clone(),
        scenario_id: forge_harness::facade::scenario_id(&fixture.name),
        adapter_name: "forge-relational".to_string(),
        scenario_name: fixture.name.clone(),
        replay_name: replay.name.clone(),
        requested_targets: replay.request.targets.clone(),
        summary: replay_summary(latest_replay).into_harness_projection(),
        attachments: Vec::new(),
    })
}
