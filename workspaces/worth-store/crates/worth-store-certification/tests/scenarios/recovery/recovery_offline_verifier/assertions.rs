use worth_store_recovery_physics::{
    OfflineRecoveryVerificationReport, RecoveryCounterSnapshot, RecoveryDeterminismReport,
    RuntimeRecoveryReportDenial,
};

pub fn assert_independent_offline_report(report: &OfflineRecoveryVerificationReport) {
    assert_eq!(report.live_runtime_constructions(), 0);
    assert_eq!(report.runtime_cache_reads(), 0);
    assert_eq!(report.semantic_decode_attempts(), 3);
    assert_eq!(report.inspected_records(), 3);
    assert!(report.inspected_bytes() > 0);
    assert!(report.recovered_state().is_some());
    assert!(report.counters().is_some());
}

pub fn assert_deterministic_recovery(report: &RecoveryDeterminismReport) {
    assert!(report.is_deterministic());
    assert!(report.first_runtime_verifier_comparison().is_equivalent());
    assert!(report.second_runtime_verifier_comparison().is_equivalent());
    assert!(report.allowed_nondeterministic_metadata().is_empty());
}

pub fn assert_runtime_report_denial(
    denial: RuntimeRecoveryReportDenial,
    expected: RuntimeRecoveryReportDenial,
) {
    assert_eq!(denial, expected);
}

pub fn assert_expected_recovery_counters(counters: RecoveryCounterSnapshot) {
    assert_eq!(counters.replayed_frames(), 1);
    assert_eq!(counters.skipped_frames(), 0);
    assert_eq!(counters.validated_checkpoints(), 1);
    assert_eq!(counters.scanned_segments(), 1);
    assert_eq!(counters.page_redos(), 1);
    assert_eq!(counters.forbidden_full_store_scans(), 0);
}
