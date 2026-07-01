use super::contract_subject::{captured_retained_workload, retained_replay_parts};
use worth_spatial::facade::retained_replay_workload::{
    admit_retained_replay_capture, ReplayEvidenceKind, ReplayParityKind, ReplayWorkload,
};
use worth_spatial::facade::workload_vocabulary::SpatialWorkloadStage;

#[test]
fn retained_replay_workload_consumes_retained_artifacts() {
    let parts = retained_replay_parts("retained-replay-consumes");
    let transformed_identity = parts
        .transformed
        .receipts()
        .stage_identity()
        .receipt_identity();
    let transformed_stage_receipt_identity = parts
        .transformed
        .receipts()
        .stage_receipt()
        .identity()
        .receipt_identity();
    let projection_digest = parts
        .projection_consumed
        .projection_consumption_digest()
        .to_string();
    let projection_rows = parts
        .projection_consumed
        .counters()
        .projection_receipts_consumed();
    let retained_capture = captured_retained_workload("retained-replay-consumes", &parts);
    let retained_capture_identity = retained_capture
        .capture_receipt()
        .capture_identity()
        .to_string();
    let retained_artifact_identity = retained_capture
        .capture_receipt()
        .retained_artifact_identity()
        .to_string();
    let retained_basis_identity = retained_capture
        .capture_receipt()
        .retained_basis_identity()
        .to_string();
    let replay_checkpoint_identity = retained_capture
        .capture_receipt()
        .replay_checkpoint_identity()
        .to_string();
    let retained_historical = parts
        .retained_parts
        .retained
        .historical_replay(&parts.retained_parts.retained.replay_subject())
        .expect("retained historical replay should remain available");
    let historical_replay_identity = retained_historical.historical_digest().to_string();
    let replay_evidence_identity = format!(
        "replay-evidence:{transformed_identity}:{retained_artifact_identity}:{historical_replay_identity}:{projection_digest}"
    );

    let replayed = ReplayWorkload::for_transformed_workload(parts.transformed)
        .declared("replay retained planar artifacts for overlap certification")
        .with_admitted_retained_replay_capture(admit_retained_replay_capture(retained_capture))
        .replay()
        .expect("retained replay workload should admit");

    assert_eq!(
        replayed.receipts().stage_identity().stage(),
        SpatialWorkloadStage::RetainedReplay
    );
    assert_eq!(
        replayed
            .receipts()
            .stage_receipt()
            .identity()
            .upstream_receipt(),
        transformed_stage_receipt_identity
    );
    assert_eq!(
        replayed.receipts().transformed_workload_identity(),
        transformed_identity
    );
    assert_eq!(
        replayed.receipts().retained_artifact_identity(),
        retained_artifact_identity
    );
    assert_eq!(
        replayed.receipts().retained_artifact_capture_identity(),
        retained_capture_identity
    );
    assert_eq!(
        replayed.receipts().retained_basis_identity(),
        retained_basis_identity
    );
    assert_eq!(
        replayed.receipts().replay_checkpoint_identity(),
        replay_checkpoint_identity
    );
    assert_eq!(
        replayed.receipts().replay_evidence_identity(),
        replay_evidence_identity
    );
    assert_eq!(replayed.receipts().counters().retained_artifact_rows(), 2);
    assert_eq!(replayed.receipts().counters().replay_evidence_rows(), 3);
    assert_eq!(replayed.receipts().counters().replay_rows(), 1);
    assert_eq!(
        replayed.receipts().counters().projection_consumed_rows(),
        projection_rows
    );
    assert_eq!(
        replayed
            .evidence()
            .rows()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        vec![
            ReplayEvidenceKind::RetainedArtifactCapture,
            ReplayEvidenceKind::HistoricalReplay,
            ReplayEvidenceKind::ProjectionConsumptionParity,
        ]
    );
    assert_eq!(
        replayed.evidence().rows()[0].evidence_identity(),
        retained_artifact_identity
    );
    assert_eq!(
        replayed.evidence().rows()[1].evidence_identity(),
        historical_replay_identity
    );
    assert_eq!(
        replayed.evidence().rows()[2].evidence_identity(),
        projection_digest
    );
    assert_eq!(
        replayed.parity_report().rows()[0].kind(),
        ReplayParityKind::LiveRetainedReplayedProjectionMatch
    );
    assert!(replayed.can_enter_diagnostics_workload());
}
