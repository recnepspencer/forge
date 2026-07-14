use worth_store_physical_backend::BackendDurabilityProfile;
use worth_store_recovery_physics::WalDurabilityCrashPosture;

pub fn assert_unacknowledged_replayable_posture<P: BackendDurabilityProfile>(
    posture: &WalDurabilityCrashPosture<P>,
) {
    assert!(posture.is_replayable_after_crash());
    assert!(!posture.is_acknowledged());
    assert_eq!(posture.profile_id(), P::ID);
    assert_eq!(posture.crash_basis().profile_id(), P::ID);
    assert_eq!(
        posture.crash_basis().required_barriers(),
        P::REQUIRED_BARRIERS
    );
    assert_eq!(
        posture.crash_basis().completed_barriers(),
        P::REQUIRED_BARRIERS
    );
    assert_eq!(posture.crash_basis().segment_id().get(), 42);
    assert_eq!(posture.crash_basis().generation().get(), 7);
    assert_eq!(posture.crash_basis().lsn_range().start().get(), 100);
    assert_eq!(posture.crash_basis().lsn_range().end_exclusive().get(), 101);
    assert!(!posture.crash_basis().frame_digest().as_str().is_empty());
}
