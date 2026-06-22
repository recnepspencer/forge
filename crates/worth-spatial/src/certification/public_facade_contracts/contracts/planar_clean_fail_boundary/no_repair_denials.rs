use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts, PlanarCleanFailBoundaryDenialKind,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject;

use super::clean_fail_fixture::{
    diagnostic, dirty_input, dirty_recovery, unbounded_input, unbounded_recovery,
};
use super::runtime_handles::clean_fail_handle;

#[test]
fn clean_fail_boundary_denies_heuristic_dirty_input_repair() {
    let world = "phase-20-deny-repair";
    let source = "dirty:repair-attempt";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle(world));
    let denial = PlanarCleanFailBoundary::from_planar_input(dirty_input(world, source))
        .recovery_posture(dirty_recovery(world, source))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::topology_failure(source),
        ))
        .with_heuristic_repair_attempt()
        .certify_clean_fail_boundary()
        .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("dirty repair must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::HeuristicRepairAttempted
    );
    assert_eq!(denial.counters().repair_attempts_denied(), 1);
}

#[test]
fn clean_fail_boundary_denies_unbounded_bounded_conversion() {
    let world = "phase-20-deny-bounded-conversion";
    let source = "unbounded:bounded-conversion";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle(world));
    let denial = PlanarCleanFailBoundary::from_planar_input(unbounded_input(world, source))
        .recovery_posture(unbounded_recovery(world, source))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ))
        .with_bounded_conversion_attempt()
        .certify_clean_fail_boundary()
        .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("bounded conversion must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::BoundedConversionAttempted
    );
    assert_eq!(denial.counters().bounded_conversions_denied(), 1);
}
