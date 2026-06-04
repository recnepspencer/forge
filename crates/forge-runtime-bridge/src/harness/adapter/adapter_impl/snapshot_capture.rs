use super::*;

use forge_harness::facade::{
    snapshot_id, SnapshotObservation, SnapshotPayload as HarnessSnapshotCaptureValue,
};

use crate::harness::adapter::terminal_report_export::{
    empty_bridge_terminal_snapshot_capture_value, historical_terminal_snapshot_capture_value,
    route_terminal_snapshot_capture_value,
};
use crate::harness::adapter::BridgeHarnessTargetId;

pub(super) fn capture_bridge_snapshot_record(
    adapter_name: &str,
    runtime: &BridgeHarnessSession,
    fixture: &ScenarioFixture<BridgeHarnessFixture>,
    request: &ExecutionRequest<BridgeHarnessTargetId>,
    profile: &ExecutionProfile,
) -> Result<SnapshotRecord<BridgeHarnessTargetId>, BridgeHarnessError> {
    let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
    let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
    let runtime_bridge = runtime
        .runtime
        .as_ref()
        .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
    let (status, detail, value) = snapshot_observation_export(runtime_bridge);
    Ok(SnapshotRecord {
        schema_version: RecordSchemaVersion::V1,
        snapshot_id: snapshot_id(&run_id_value, "capture"),
        run_id: run_id_value,
        adapter_name: adapter_name.to_string(),
        scenario_name: fixture.name.clone(),
        profile_name: profile.name.clone(),
        time_marker: profile.time_marker.clone(),
        observations: vec![SnapshotObservation {
            target: request.targets.first().cloned().unwrap_or_else(|| {
                BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new(
                    "bridge",
                ))
            }),
            status,
            detail,
            value,
        }],
        attachments: Vec::new(),
        extensions: BTreeMap::new(),
    })
}

fn snapshot_observation_export(
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> (
    ObservationStatus,
    Option<String>,
    Option<HarnessSnapshotCaptureValue>,
) {
    if let Some(record) = runtime_bridge
        .diagnostics()
        .last_historical_evaluation_record()
    {
        return (
            ObservationStatus::Clean,
            Some("historical evaluation record captured".to_string()),
            Some(historical_terminal_snapshot_capture_value(&record)),
        );
    }

    match runtime_bridge.diagnostics().last_route_record() {
        Some(record) => (
            ObservationStatus::Clean,
            Some(format!(
                "{} snapshot records captured",
                record.counters().snapshot_read_count()
            )),
            Some(route_terminal_snapshot_capture_value(&record)),
        ),
        None => (
            ObservationStatus::Clean,
            Some("bridge delivery has not run yet".to_string()),
            Some(empty_bridge_terminal_snapshot_capture_value()),
        ),
    }
}
