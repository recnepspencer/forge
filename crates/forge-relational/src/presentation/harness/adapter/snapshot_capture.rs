use std::collections::BTreeMap;

use forge_harness::facade::{
    run_id, snapshot_id, ExecutionProfile, ExecutionRequest, ObservationStatus,
    SnapshotObservation, SnapshotRecord,
};

use crate::facade::harness::RelationalHarnessError;
use crate::facade::runtime::RelationalRuntime;
use crate::storage::data::RelationalReadView;
use crate::transactions::data::RecordRef;

use super::super::data::{RelationalFixture, RelationalHarnessAdapter};
use super::super::targets::{parse_target, resolve_targets};
use super::aspect_snapshot_payloads::{entity_snapshot_payload, relation_snapshot_payload};

pub(super) fn capture_snapshot(
    _adapter: &RelationalHarnessAdapter,
    runtime: &RelationalRuntime,
    fixture: &forge_harness::facade::ScenarioFixture<RelationalFixture>,
    request: &ExecutionRequest<String>,
    profile: &ExecutionProfile,
) -> Result<SnapshotRecord<String>, RelationalHarnessError> {
    let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
    let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
    let mut clone = runtime.fork();
    let snapshot = clone.visibility_authority().snapshot();
    let resolved_version_id = clone
        .read_truth()
        .query_plan_context(&snapshot)
        .map(|context| context.version_id)
        .unwrap_or(snapshot.version_id);
    let read_view = clone.read_truth().read_version(resolved_version_id);
    let observations = resolve_targets(request)
        .into_iter()
        .map(|target| capture_target_snapshot(&read_view, target))
        .collect::<Result<Vec<_>, RelationalHarnessError>>()?;
    Ok(SnapshotRecord {
        schema_version: forge_harness::facade::RecordSchemaVersion::V1,
        snapshot_id: snapshot_id(&run_id_value, "capture"),
        run_id: run_id_value,
        adapter_name: "forge-relational".to_string(),
        scenario_name: fixture.name.clone(),
        profile_name: profile.name.clone(),
        time_marker: profile.time_marker.clone(),
        observations,
        attachments: Vec::new(),
        extensions: BTreeMap::new(),
    })
}

fn capture_target_snapshot(
    read_view: &RelationalReadView,
    target: String,
) -> Result<SnapshotObservation<String>, RelationalHarnessError> {
    let snapshot_value = match parse_target(&target)? {
        RecordRef::Entity(entity_id) => read_view
            .get_entity(entity_id)
            .map(entity_snapshot_payload)
            .transpose()?,
        RecordRef::Relation(relation_id) => read_view
            .get_relation(relation_id)
            .map(relation_snapshot_payload)
            .transpose()?,
    };
    let (status, detail) = match snapshot_value {
        Some(_) => (ObservationStatus::Clean, None),
        None => (
            ObservationStatus::Unknown,
            Some("target not visible at captured snapshot".to_string()),
        ),
    };
    Ok(SnapshotObservation {
        target,
        status,
        detail,
        value: snapshot_value,
    })
}
