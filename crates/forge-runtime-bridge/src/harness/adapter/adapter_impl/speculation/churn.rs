use super::{shared, BridgeHarnessError, SpeculationHarnessExecution};
use crate::harness::fixtures::BridgeHarnessFixture;
use serde_json::json;

pub(super) fn execute_churn_certification(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<SpeculationHarnessExecution, BridgeHarnessError> {
    let mut lifecycle_digests = Vec::new();
    let mut branch_rows = Vec::new();
    let mut max_preview_artifacts = 0usize;
    let mut max_replay_width = 0usize;
    let baseline_authoritative_route_digest =
        shared::first_commit_routing_digest(runtime_bridge, fixture)?;
    let mut authoritative_route_digests_after_discard = Vec::new();

    for index in 0..3 {
        let branch = format!("branch-{index}");
        let session_id = format!("harness:speculation-churn:{index}");
        let signal_branch = format!("signal:{index}");
        let admitted = runtime_bridge
            .admit_preview_session(
                crate::facade::BridgePreviewSessionIdentity::new(session_id.clone()),
                shared::preview_declaration(&session_id, &branch, &signal_branch),
            )
            .map_err(|error| {
                BridgeHarnessError::new(format!("speculation churn admission failed: {error}"))
            })?;
        let (active, execution_record) =
            runtime_bridge.activate_preview_session(admitted, index + 1, 1, 1);
        let (_discarded, discard_record) = runtime_bridge
            .discard_preview_session(
                active,
                &execution_record,
                vec![
                    crate::facade::BridgePreviewResidueClass::PreviewDiagnosticsRetained,
                    crate::facade::BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
                ],
            )
            .map_err(|error| {
                BridgeHarnessError::new(format!("speculation churn discard failed: {error}"))
            })?;
        let replay_bundle = runtime_bridge
            .replay_preview_bundle(&session_id)
            .map_err(|error| {
                BridgeHarnessError::new(format!("speculation churn replay failed: {error}"))
            })?;
        lifecycle_digests.push(replay_bundle.digest().to_string());
        max_preview_artifacts =
            max_preview_artifacts.max(execution_record.counters().preview_artifact_count());
        max_replay_width = max_replay_width.max(replay_bundle.counters().replay_bundle_width());
        let authoritative_route_digest_after_discard =
            shared::first_commit_routing_digest(runtime_bridge, fixture)?;
        authoritative_route_digests_after_discard
            .push(authoritative_route_digest_after_discard.clone());
        branch_rows.push(json!({
            "preview_session_identity": session_id,
            "truth_branch_identity": branch,
            "execution_record_identity": execution_record.record_identity().as_str(),
            "discard_record_identity": discard_record.record_identity().as_str(),
            "lifecycle_outcome": format!("{:?}", replay_bundle.lifecycle_outcome()),
            "authoritative_route_digest_after_discard": authoritative_route_digest_after_discard,
        }));
    }

    let final_authoritative_route_digest =
        shared::first_commit_routing_digest(runtime_bridge, fixture)?;
    let preview_session_count_touched = lifecycle_digests.len();
    let authoritative_route_observation_count = authoritative_route_digests_after_discard.len() + 2;

    Ok(SpeculationHarnessExecution::Churn {
        lifecycle_digests,
        branch_isolation_matrix: json!({
            "rows": branch_rows,
            "baseline_authoritative_route_digest": baseline_authoritative_route_digest,
            "final_authoritative_route_digest": final_authoritative_route_digest,
        }),
        resource_bound_report: json!({
            "retained_preview_execution_record_count": runtime_bridge.diagnostics().preview_execution_records().len(),
            "retained_preview_discard_record_count": runtime_bridge.diagnostics().preview_discard_records().len(),
            "retained_preview_promotion_record_count": runtime_bridge.diagnostics().preview_promotion_records().len(),
            "max_preview_artifact_count": max_preview_artifacts,
            "max_replay_bundle_width": max_replay_width,
            "authoritative_route_observation_count": authoritative_route_observation_count,
        }),
        counter_snapshot: json!({
            "preview_session_count_touched": preview_session_count_touched,
            "max_preview_artifact_count": max_preview_artifacts,
            "max_replay_bundle_width": max_replay_width,
            "retained_preview_execution_record_count": runtime_bridge.diagnostics().preview_execution_records().len(),
            "retained_preview_discard_record_count": runtime_bridge.diagnostics().preview_discard_records().len(),
            "retained_preview_promotion_record_count": runtime_bridge.diagnostics().preview_promotion_records().len(),
            "authoritative_route_observation_count": authoritative_route_observation_count,
        }),
    })
}
