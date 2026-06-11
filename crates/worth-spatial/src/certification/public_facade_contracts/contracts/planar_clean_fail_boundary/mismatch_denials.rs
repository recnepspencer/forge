use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts, PlanarCleanFailBoundaryDenialKind,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject;

use super::clean_fail_fixture::{diagnostic, dirty_input, dirty_recovery, unbounded_recovery};
use super::runtime_handles::clean_fail_handle;

#[test]
fn clean_fail_boundary_denies_recovery_receipt_for_different_source() {
    let world = "phase-20-mismatched-recovery";
    let source = "dirty:primary-source";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle(world));
    let denial = PlanarCleanFailBoundary::from_planar_input(dirty_input(world, source))
        .recovery_posture(dirty_recovery(world, "dirty:other-source"))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::topology_failure(source),
        ))
        .certify_clean_fail_boundary()
        .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("mismatched recovery source must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::MismatchedRecoveryPosture
    );
}

#[test]
fn clean_fail_boundary_denies_diagnostic_receipt_for_different_source() {
    let world = "phase-20-mismatched-diagnostic";
    let source = "unbounded:primary-source";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle(world));
    let denial = PlanarCleanFailBoundary::from_planar_input(
        super::clean_fail_fixture::unbounded_input(world, source),
    )
    .recovery_posture(unbounded_recovery(world, source))
    .diagnostics(diagnostic(
        world,
        PlanarDiagnosticSubject::unsupported_planar_class("unbounded:other-source"),
    ))
    .certify_clean_fail_boundary()
    .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("mismatched diagnostic source must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::MismatchedDiagnostics
    );
}
