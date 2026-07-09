#[path = "s4_closeout/fixture.rs"]
mod fixture;

use worth_store_recovery_physics::{
    RecoveryPhysicsCloseoutDenial, RuntimeRecoveryReportDenial, SyntheticRecoveryShortcutKind,
    SyntheticRecoveryShortcutRejectionBoundary, WalCheckpointLsnRecoveryPhysicsSuite,
};

#[test]
fn all_required_synthetic_shortcuts_have_named_s4_boundaries() {
    let report = fixture::certify_complete_closeout()
        .closeout_report()
        .synthetic_shortcut_rejections()
        .clone();

    assert!(report.denies(SyntheticRecoveryShortcutKind::RawBytes));
    assert!(report.denies(SyntheticRecoveryShortcutKind::LiveStateReuse));
    assert!(report.denies(SyntheticRecoveryShortcutKind::BackendResidueGuessing));
    assert!(report.denies(SyntheticRecoveryShortcutKind::UnsupportedDurabilityClaim));
    assert!(report.denies(SyntheticRecoveryShortcutKind::InvalidPageLsn));
    assert!(report.denies(SyntheticRecoveryShortcutKind::TornCheckpoint));
    assert!(report.denies(SyntheticRecoveryShortcutKind::UnboundedRecoveryPlan));
    assert!(report
        .rejections()
        .iter()
        .any(|row| row.boundary()
            == SyntheticRecoveryShortcutRejectionBoundary::BoundedRecoveryBudget));
}

#[test]
fn closeout_denies_when_shortcut_rejection_coverage_is_missing() {
    let denial = fixture::evidence_with_missing_shortcut_rejection_denial();

    assert_eq!(
        denial,
        RecoveryPhysicsCloseoutDenial::MissingSyntheticShortcutRejection
    );
}

#[test]
fn crash_seams_require_fresh_runtime_fault_scheduler_evidence() {
    assert_eq!(
        fixture::same_process_runtime_report_denial(),
        RuntimeRecoveryReportDenial::SameProcessLiveStateReuse
    );
    assert_eq!(
        fixture::missing_crash_scheduler_evidence_denial(),
        RecoveryPhysicsCloseoutDenial::MissingCrashFaultSchedulerEvidence
    );
}

#[test]
fn unrelated_denials_cannot_satisfy_synthetic_shortcut_coverage() {
    assert_eq!(
        fixture::unrelated_residue_shortcut_denial(),
        RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence
    );
    assert_eq!(
        fixture::unrelated_budget_shortcut_denial(),
        RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence
    );
}

#[test]
fn closeout_denies_when_a_required_crash_seam_is_absent() {
    let denial = WalCheckpointLsnRecoveryPhysicsSuite::from_required_s4_lanes()
        .certify(fixture::evidence_with_missing_crash_seam())
        .unwrap_err();

    assert_eq!(denial, RecoveryPhysicsCloseoutDenial::MissingCrashSeam);
}
