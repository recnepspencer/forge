mod phase22_fixture;

use forge_store_recovery_physics::{
    reject_decision_row, CrashBoundaryLayoutReport, PartialPublicationCrashEdge,
    PartialPublicationObservationSet, RecoveryLayoutAccessDenialKind, RecoverySourceLayoutReport,
    UnacknowledgedPublicationOutcome,
};

#[test]
fn phase22_recovery_source_and_crash_boundary_families_deny_residue_and_rollback_shortcuts() {
    let source = phase22_fixture::admitted_source_with_residue();
    let report = RecoverySourceLayoutReport::from_source(&source);
    assert_eq!(report.candidate_count(), 3);
    assert_eq!(report.residue_rejection_count(), 1);
    assert!(report.selected_checkpoint_id().is_some());
    assert_eq!(
        report.selected_wal_range(),
        Some(phase22_fixture::wal_range(30, 45))
    );

    let denial = reject_decision_row(&source.trace().decision_rows()[0]).unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::RecoverySourceRowCannotStandInForRecoveryAuthority
    );

    let report = CrashBoundaryLayoutReport::admit_observations(
        PartialPublicationObservationSet::new()
            .with_persisted_crash_edge(PartialPublicationCrashEdge::before_wal_append("sha256:op")),
    )
    .expect("crash report");
    assert_eq!(
        report.outcome(),
        UnacknowledgedPublicationOutcome::NoWalAppendObserved
    );

    let denial = CrashBoundaryLayoutReport::admit_observations(
        PartialPublicationObservationSet::new().with_backend_residue(
            forge_store_recovery_physics::BackendResidueKind::BackendDirectoryResidue,
            "sha256:residue",
        ),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::BackendResidueCannotStandInForCrashBoundaryAuthority
    );

    let denial = CrashBoundaryLayoutReport::admit_observations(
        PartialPublicationObservationSet::new().with_persisted_crash_edge(
            PartialPublicationCrashEdge::during_checkpoint_cutover("sha256:checkpoint"),
        ),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::AmbiguousResidueCannotStandInForCrashBoundaryAuthority
    );

    let denial = CrashBoundaryLayoutReport::reject_derived_rollback_outcome(
        UnacknowledgedPublicationOutcome::RollbackImageProtected,
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::DerivedRollbackCannotStandInForCrashBoundaryAuthority
    );
}
