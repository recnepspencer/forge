use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarBoundedConversion, PlanarCleanFailAction, PlanarCleanFailBoundary,
    PlanarCleanFailBoundaryContracts, PlanarCleanFailClass, PlanarCleanFailTruthEffect,
    PlanarDirtyInputKind, PlanarRepairAttempt,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality,
};

use super::clean_fail_fixture::{
    certify_clean_fail_boundary, diagnostic, dirty_input, dirty_input_with_kind, dirty_recovery,
};
use super::runtime_handles::clean_fail_handle;

#[test]
fn dirty_planar_input_fails_cleanly_without_heuristic_repair() {
    let cases = [
        (
            "phase-20-dirty-self-intersection",
            "dirty:self-intersection",
            PlanarDirtyInputKind::SelfIntersectingLoop,
        ),
        (
            "phase-20-dirty-non-manifold",
            "dirty:non-manifold-wire",
            PlanarDirtyInputKind::NonManifoldWire,
        ),
        (
            "phase-20-dirty-thin-wall",
            "dirty:thin-wall",
            PlanarDirtyInputKind::ThinWall,
        ),
        (
            "phase-20-dirty-orientation",
            "dirty:orientation-inconsistency",
            PlanarDirtyInputKind::OrientationInconsistency,
        ),
    ];

    for (world, source, kind) in cases {
        let receipt = certify_clean_fail_boundary(
            world,
            dirty_input_with_kind(world, source, kind),
            dirty_recovery(world, source),
            diagnostic(world, PlanarDiagnosticSubject::topology_failure(source)),
        );

        assert_eq!(receipt.class(), PlanarCleanFailClass::DirtyInput);
        assert_eq!(receipt.basis().input().dirty_input_kind(), Some(kind));
        assert_eq!(
            receipt.basis().diagnostics().trigger_locality(),
            PlanarDiagnosticTriggerLocality::TopologyContract
        );
        assert_eq!(
            receipt.action(),
            PlanarCleanFailAction::InspectWithoutRepair
        );
        assert_eq!(receipt.repair_attempt(), PlanarRepairAttempt::NotAttempted);
        assert_eq!(
            receipt.bounded_conversion(),
            PlanarBoundedConversion::NotAttempted
        );
        assert_eq!(
            receipt.truth_effect(),
            PlanarCleanFailTruthEffect::DoesNotChangePlanarTruth
        );
        assert_eq!(receipt.counters().clean_fail_sources(), 1);
        assert_eq!(receipt.counters().admission_rows_consumed(), 1);
        assert_eq!(receipt.counters().recovery_receipts_consumed(), 1);
        assert_eq!(receipt.counters().diagnostic_receipts_consumed(), 1);
    }
}

#[test]
fn mb_m6_5_dirty_planar_input_clean_fail_localization() {
    let world = "phase-20-mb-m6-5";
    let source = "dirty:stable-id-orientation-reversal";
    let receipt = certify_clean_fail_boundary(
        world,
        dirty_input(world, source),
        dirty_recovery(world, source),
        diagnostic(world, PlanarDiagnosticSubject::motion_failure(source)),
    );

    assert_eq!(receipt.class(), PlanarCleanFailClass::DirtyInput);
    assert_eq!(
        receipt.basis().input().stable_topology_identity(),
        Some("stable-dirty-topology-id")
    );
    assert!(receipt.basis().input().transform_posture_digest().is_some());
    assert_eq!(
        receipt.basis().diagnostics().trigger_locality(),
        PlanarDiagnosticTriggerLocality::MotionOrRotationPosture
    );
    assert_ne!(
        receipt.basis().input().stable_topology_identity(),
        Some(receipt.clean_fail_boundary_digest())
    );
}

#[test]
fn dirty_source_detail_participates_in_clean_fail_identity() {
    let source = "dirty:same-source-different-clean-fail-detail";
    let self_intersection = certify_clean_fail_boundary(
        "phase-20-dirty-detail-self-intersection",
        dirty_input_with_kind(
            "phase-20-dirty-detail-self-intersection",
            source,
            PlanarDirtyInputKind::SelfIntersectingLoop,
        ),
        dirty_recovery("phase-20-dirty-detail-self-intersection", source),
        diagnostic(
            "phase-20-dirty-detail-self-intersection",
            PlanarDiagnosticSubject::topology_failure(source),
        ),
    );
    let thin_wall = certify_clean_fail_boundary(
        "phase-20-dirty-detail-thin-wall",
        dirty_input_with_kind(
            "phase-20-dirty-detail-thin-wall",
            source,
            PlanarDirtyInputKind::ThinWall,
        ),
        dirty_recovery("phase-20-dirty-detail-thin-wall", source),
        diagnostic(
            "phase-20-dirty-detail-thin-wall",
            PlanarDiagnosticSubject::topology_failure(source),
        ),
    );

    assert_ne!(
        self_intersection.clean_fail_boundary_digest(),
        thin_wall.clean_fail_boundary_digest()
    );
    assert_eq!(
        self_intersection.basis().input().dirty_input_kind(),
        Some(PlanarDirtyInputKind::SelfIntersectingLoop)
    );
    assert_eq!(
        thin_wall.basis().input().dirty_input_kind(),
        Some(PlanarDirtyInputKind::ThinWall)
    );
}

#[test]
fn clean_fail_boundary_plan_inspects_each_declared_boundary_axis() {
    let world = "phase-20-inspection-breadth";
    let source = "dirty:inspection-breadth";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle(world));
    let plan = PlanarCleanFailBoundary::from_planar_input(dirty_input(world, source))
        .recovery_posture(dirty_recovery(world, source))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::topology_failure(source),
        ))
        .certify_clean_fail_boundary()
        .compile(&contracts)
        .expect("clean-fail boundary plan");

    assert_eq!(plan.inspected_clean_fail_rows(), 9);
}
