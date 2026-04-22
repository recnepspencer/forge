use super::support::*;

#[test]
fn certification_stale_checkpoint_report_is_not_replay_mismatch() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_stale_checkpoint();

    assert_ne!(report.fresh_bundle_digest(), report.stale_bundle_digest());
    assert_ne!(
        report.fresh_checkpoint_digest(),
        report.stale_checkpoint_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::CheckpointResumeOrReplay
    );
    assert!(report.checkpoint_drift_is_primary_without_replay_mismatch());
    assert_eq!(report.suppressed_failure_boundary_count(), 0);
    assert_eq!(report.counters().comparison_plan_count(), 1);
    assert_eq!(report.counters().bundle_comparison_count(), 1);
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 1);
    assert_eq!(report.counters().failure_localization_count(), 1);
    assert_eq!(report.counters().stale_checkpoint_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
    assert!(report
        .comparison_report_digest()
        .starts_with("bridge-subscription-certification-comparison-report:sha256:"));
    assert!(report
        .digest()
        .starts_with("bridge-subscription-certification-stale-checkpoint-report:sha256:"));
}
