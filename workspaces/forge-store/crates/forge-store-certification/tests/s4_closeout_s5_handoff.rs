#[path = "s4_closeout/fixture.rs"]
mod fixture;

use forge_store_recovery_physics::RecoveryPhysicsStabilityAssumption;

#[test]
fn s5_readiness_handoff_preserves_recovered_state_and_source_precedence() {
    let bundle = fixture::certify_complete_closeout();
    let report = bundle.closeout_report();
    let readiness = bundle.publish_s5_readiness();
    let admission = readiness.admit_for_s5_startup().unwrap();

    assert_eq!(readiness.recovered_root(), report.recovered_root());
    assert_eq!(admission.recovered_root(), report.recovered_root());
    assert_eq!(
        readiness.admitted_page_lsn_frontier(),
        report.admitted_page_lsn_frontier()
    );
    assert_eq!(
        admission.admitted_page_lsn_frontier(),
        report.admitted_page_lsn_frontier()
    );
    assert_eq!(readiness.counters(), report.counters());
    assert_eq!(
        readiness
            .replay_receipt()
            .recovered_state()
            .source_decision_digest(),
        readiness
            .source_precedence_trace()
            .canonical_replay_digest()
    );
    assert_eq!(
        admission.source_candidate_count(),
        readiness.source_precedence_trace().candidate_count()
    );
}

#[test]
fn s5_readiness_handoff_names_future_scope_reservations() {
    let readiness = fixture::certify_complete_closeout().publish_s5_readiness();

    for assumption in [
        RecoveryPhysicsStabilityAssumption::PersistedBytesAreReadStableForIsolationStartup,
        RecoveryPhysicsStabilityAssumption::DirectoryAndRenameDurabilityAlreadyAdmitted,
        RecoveryPhysicsStabilityAssumption::BackendProfileMatchesDurabilityReceipts,
        RecoveryPhysicsStabilityAssumption::NoS5PhysicalIsolationClaim,
        RecoveryPhysicsStabilityAssumption::NoIoQosClaim,
        RecoveryPhysicsStabilityAssumption::NoBlobLifecycleClaim,
        RecoveryPhysicsStabilityAssumption::NoRepairForensicsClaim,
        RecoveryPhysicsStabilityAssumption::NoSecurityAuthenticityClaim,
        RecoveryPhysicsStabilityAssumption::NoFullPhysicalDatabaseCertificationClaim,
    ] {
        assert!(
            readiness.stability_assumptions().contains(&assumption),
            "missing S.5 readiness assumption: {assumption:?}"
        );
    }
    readiness.admit_for_s5_startup().unwrap();
}
