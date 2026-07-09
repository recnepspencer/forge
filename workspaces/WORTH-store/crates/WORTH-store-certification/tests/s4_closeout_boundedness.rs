#[path = "s4_closeout/fixture.rs"]
mod fixture;

use worth_store_recovery_physics::RecoveryWorkBound;

#[test]
fn closeout_boundedness_is_counter_backed_by_checkpoint_interval_and_wal_tail() {
    let bundle = fixture::certify_complete_closeout();
    let report = bundle.closeout_report();
    let counters = report.counters();

    assert_eq!(counters.validated_checkpoints(), 1);
    assert_eq!(counters.replayed_frames(), 1);
    assert_eq!(counters.skipped_frames(), 0);
    assert_eq!(counters.scanned_segments(), 1);
    assert_eq!(counters.page_redos(), 1);
    assert_eq!(counters.forbidden_full_store_scans(), 0);
    assert_eq!(
        report.work_bound(),
        RecoveryWorkBound::CheckpointIntervalAndWalTail {
            checkpoint_interval_frames: 4,
            wal_tail_frame_limit: 4,
            observed_wal_tail_frames: 1,
        }
    );
}

#[test]
fn closeout_denies_boundedness_without_checkpoint_interval_authority() {
    assert_eq!(
        fixture::unbounded_closeout_denial(),
        worth_store_recovery_physics::RecoveryPhysicsCloseoutDenial::BoundednessAuthorityMismatch
    );
}
