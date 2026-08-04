use super::hostile_certification_world::temporal_phase9_mixed_workload;
use crate::facade::{
    DiagnosticsLevel, TemporalPerformanceFailureMode, TemporalReconstructabilityArtifact,
};

#[test]
fn temporal_phase9_mixed_workload_preserves_parity_and_boundedness_across_branch_restore() {
    let outcome = temporal_phase9_mixed_workload();

    assert!(outcome.bundle.passed, "{:?}", outcome.bundle.failures);
    assert_eq!(outcome.bundle.summary.passed_family_count, 4);

    assert!(
        outcome.eligibility_parity.parity,
        "{:?}",
        outcome.eligibility_parity.mismatch_classes
    );
    assert_eq!(
        outcome.feature.reconstructability_before_restore.clock_checkpoint_digest,
        outcome.sibling.reconstructability_before_restore.clock_checkpoint_digest,
        "equivalent sibling branches must share the same checkpoint-honest clock basis before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .scheduled_wake_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .scheduled_wake_digest,
        "equivalent sibling branches must share the same scheduled wake frontier before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .ready_wake_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .ready_wake_digest,
        "equivalent sibling branches must share the same ready frontier before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .temporal_eligibility_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .temporal_eligibility_digest,
        "equivalent sibling branches must share the same temporal eligibility truth before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .clock_checkpoint_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .clock_checkpoint_digest,
        "equivalent restored sibling branches must converge to the same clock checkpoint digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .scheduled_wake_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .scheduled_wake_digest,
        "equivalent restored sibling branches must converge to the same scheduled wake digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .ready_wake_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .ready_wake_digest,
        "equivalent restored sibling branches must converge to the same ready queue digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .temporal_eligibility_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .temporal_eligibility_digest,
        "equivalent restored sibling branches must converge to the same eligibility digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .previous_value_reference_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .previous_value_reference_digest,
        "equivalent restored sibling branches must converge to the same previous-value basis"
    );
    assert_ne!(
        outcome
            .feature
            .reconstructability_before_restore
            .ready_wake_digest,
        outcome
            .feature
            .reconstructability_after_snapshot_drift
            .ready_wake_digest,
        "hostile branch-local churn should perturb ready-frontier truth before restore"
    );
    assert!(
        outcome.feature.replay_after_snapshot_drift.frames.len()
            >= outcome.feature.replay_before_restore.frames.len(),
        "hostile branch-local churn may append replay evidence before restore, but it must not erase prior feature replay history"
    );
    assert_eq!(
        outcome.feature.head_snapshot_before_restore,
        Some(outcome.feature.restored_snapshot_id),
        "capturing the hostile branch checkpoint must advance the branch head to that checkpoint"
    );
    assert_eq!(
        outcome.feature.head_snapshot_after_restore,
        Some(outcome.feature.restored_snapshot_id),
        "restoring a hostile branch checkpoint must reinstate the captured branch head"
    );
    assert_eq!(
        outcome.sibling.head_snapshot_before_restore,
        Some(outcome.sibling.restored_snapshot_id),
        "capturing the sibling hostile branch checkpoint must advance the branch head to that checkpoint"
    );
    assert_eq!(
        outcome.sibling.head_snapshot_after_restore,
        Some(outcome.sibling.restored_snapshot_id),
        "restoring the sibling hostile branch checkpoint must reinstate the captured branch head"
    );
    assert!(
        outcome.feature.restore_parity.parity,
        "{:?}",
        outcome.feature.restore_parity.mismatch_classes
    );
    assert!(
        outcome.sibling.restore_parity.parity,
        "{:?}",
        outcome.sibling.restore_parity.mismatch_classes
    );
    assert!(
        outcome.feature.replay_after_restore.frames.len()
            >= outcome.feature.replay_before_restore.frames.len(),
        "restore may append replay evidence but must not erase prior feature branch replay history"
    );
    assert!(
        outcome.sibling.replay_after_restore.frames.len()
            >= outcome.sibling.replay_before_restore.frames.len(),
        "restore may append replay evidence but must not erase prior sibling branch replay history"
    );
    assert!(
        outcome
            .feature
            .replay_after_restore
            .frames
            .iter()
            .all(|frame| frame.branch_id == outcome.feature.branch_id),
        "feature replay history must stay branch-local after restore"
    );
    assert!(
        outcome
            .sibling
            .replay_after_restore
            .frames
            .iter()
            .all(|frame| frame.branch_id == outcome.sibling.branch_id),
        "sibling replay history must stay branch-local after restore"
    );
    assert!(
        outcome.temporal_telemetry.temporal_broad_scan_denial_count >= 4,
        "mixed temporal torture should exercise ready-frontier promotion enough to charge broad temporal scan denial counters"
    );
    assert!(
        outcome
            .feature
            .temporal_telemetry_after_restore
            .branch_local_temporal_restore_count
            >= 1,
        "feature branch restore must charge branch-local temporal restore work"
    );
    assert!(
        outcome
            .sibling
            .temporal_telemetry_after_restore
            .branch_local_temporal_restore_count
            >= 1,
        "sibling branch restore must charge branch-local temporal restore work"
    );
    assert!(
        outcome
            .feature
            .temporal_telemetry_after_restore
            .branch_restore_temporal_rebuild_denial_count
            >= 1,
        "feature branch restore must consume retained frontier truth instead of rebuilding from node conditions"
    );
    assert!(
        outcome
            .sibling
            .temporal_telemetry_after_restore
            .branch_restore_temporal_rebuild_denial_count
            >= 1,
        "sibling branch restore must consume retained frontier truth instead of rebuilding from node conditions"
    );
    assert!(
        outcome.temporal_telemetry.missed_interval_count >= 399,
        "large interval jumps must charge missed policy outcomes rather than hiding elapsed timer work"
    );
    assert_eq!(outcome.boundedness_artifact.interval_regeneration_count, 3);
    assert_ne!(
        outcome.eligibility_artifact.temporal_eligibility_digest,
        TemporalReconstructabilityArtifact::default().temporal_eligibility_digest
    );
    assert_ne!(
        outcome
            .previous_value_artifact
            .previous_value_reference_digest,
        TemporalReconstructabilityArtifact::default().previous_value_reference_digest
    );
}

#[test]
fn milestone_a_closeout_bundle_covers_hostile_temporal_certification_paths() {
    let outcome = temporal_phase9_mixed_workload();

    assert!(outcome.bundle.passed, "{:?}", outcome.bundle.failures);
    assert_eq!(outcome.bundle.summary.passed_family_count, 4);
    assert_eq!(outcome.bundle.summary.failed_family_count, 0);
    assert_eq!(outcome.bundle.summary.missing_family_count, 0);

    assert_eq!(
        outcome
            .diagnostics_operational
            .with_profile(DiagnosticsLevel::Forensic),
        outcome.diagnostics_forensic
    );
    assert!(outcome
        .diagnostics_operational
        .cost_contracts
        .prohibited_failure_modes
        .contains(&TemporalPerformanceFailureMode::TemporalBroadScan));
    assert_eq!(
        outcome.temporal_telemetry.temporal_replay_parity_check_count,
        1,
        "returning to main must restore main-branch telemetry instead of smearing branch-local parity counters across branches"
    );
}
