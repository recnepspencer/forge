#[path = "s4_closeout/fixture.rs"]
mod fixture;

use worth_store_recovery_physics::{
    RecoveryPhysicsCloseoutSuiteLane, RecoveryWorkBound, SyntheticRecoveryShortcutKind,
};

#[test]
fn required_s4_suite_lanes_certify_single_closeout_bundle() {
    let bundle = fixture::certify_complete_closeout();
    let report = bundle.closeout_report();

    assert!(report.suite_status().is_complete());
    assert_eq!(report.suite_status().required_lanes(), 31);
    assert!(report.covers_suite(RecoveryPhysicsCloseoutSuiteLane::RecoveryEntryAuthority));
    assert!(report.covers_suite(RecoveryPhysicsCloseoutSuiteLane::S5RecoveryReadinessHandoff));
    assert_eq!(report.crash_seams().len(), 8);
    assert!(report
        .synthetic_shortcut_rejections()
        .denies(SyntheticRecoveryShortcutKind::RawBytes));
    assert!(report.foundational_exact_counter_assertions() > 0);
}

#[test]
fn closeout_report_is_deterministic_across_independent_certification_paths() {
    let first = fixture::certify_complete_closeout();
    let second = fixture::certify_complete_closeout();

    assert_eq!(first.closeout_report(), second.closeout_report());
    assert_eq!(
        first.closeout_report().work_bound(),
        RecoveryWorkBound::CheckpointIntervalAndWalTail {
            checkpoint_interval_frames: 4,
            wal_tail_frame_limit: 4,
            observed_wal_tail_frames: 1,
        }
    );
}

#[test]
fn closeout_collector_denies_mixed_recovery_authority() {
    assert_eq!(
        fixture::mixed_authority_closeout_denial(),
        worth_store_recovery_physics::RecoveryPhysicsCloseoutDenial::UnboundedRecoveryPlan
    );
}
