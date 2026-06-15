use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarBoundedConversion, PlanarCleanFailAction, PlanarCleanFailClass,
    PlanarCleanFailTruthEffect, PlanarOpenInputKind, PlanarRepairAttempt,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality,
};

use super::clean_fail_fixture::{
    certify_clean_fail_boundary, diagnostic, open_input_with_kind, unbounded_input,
    unbounded_recovery,
};

#[test]
fn unbounded_half_space_posture_classifies_without_bounded_conversion() {
    let world = "phase-20-unbounded-half-space";
    let source = "unbounded:half-space-group";
    let receipt = certify_clean_fail_boundary(
        world,
        unbounded_input(world, source),
        unbounded_recovery(world, source),
        diagnostic(
            world,
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );

    assert_eq!(receipt.class(), PlanarCleanFailClass::UnboundedOrOpen);
    assert_eq!(
        receipt.basis().input().open_input_kind(),
        Some(PlanarOpenInputKind::HalfSpaceGroup)
    );
    assert_eq!(
        receipt.basis().diagnostics().trigger_locality(),
        PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass
    );
    assert_eq!(
        receipt.action(),
        PlanarCleanFailAction::ClassifyWithoutBoundedConversion
    );
    assert_eq!(
        receipt.bounded_conversion(),
        PlanarBoundedConversion::NotAttempted
    );
    assert_eq!(receipt.repair_attempt(), PlanarRepairAttempt::NotAttempted);
    assert_eq!(
        receipt.truth_effect(),
        PlanarCleanFailTruthEffect::DoesNotChangePlanarTruth
    );
    assert!(!receipt.clean_fail_boundary_digest().is_empty());
    assert_ne!(receipt.clean_fail_boundary_digest(), source);
}

#[test]
fn unbounded_half_space_posture_stays_pre_boolean() {
    let world = "phase-20-unbounded-open-posture";
    let source = "unbounded:open-domain-transform-cycle";
    let receipt = certify_clean_fail_boundary(
        world,
        open_input_with_kind(world, source, PlanarOpenInputKind::OpenPlanarDomain),
        unbounded_recovery(world, source),
        diagnostic(
            world,
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );

    assert_eq!(receipt.class(), PlanarCleanFailClass::UnboundedOrOpen);
    assert_eq!(
        receipt.basis().input().open_input_kind(),
        Some(PlanarOpenInputKind::OpenPlanarDomain)
    );
    assert_eq!(
        receipt.basis().input().stable_topology_identity(),
        Some("stable-unbounded-topology-id")
    );
    assert!(receipt.basis().input().transform_posture_digest().is_some());
    assert_eq!(receipt.counters().bounded_conversions_denied(), 0);
}

#[test]
fn open_source_detail_participates_in_clean_fail_identity() {
    let source = "unbounded:same-source-different-open-detail";
    let half_space = certify_clean_fail_boundary(
        "phase-20-open-detail-half-space",
        open_input_with_kind(
            "phase-20-open-detail-half-space",
            source,
            PlanarOpenInputKind::HalfSpaceGroup,
        ),
        unbounded_recovery("phase-20-open-detail-half-space", source),
        diagnostic(
            "phase-20-open-detail-half-space",
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );
    let open_domain = certify_clean_fail_boundary(
        "phase-20-open-detail-domain",
        open_input_with_kind(
            "phase-20-open-detail-domain",
            source,
            PlanarOpenInputKind::OpenPlanarDomain,
        ),
        unbounded_recovery("phase-20-open-detail-domain", source),
        diagnostic(
            "phase-20-open-detail-domain",
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );

    assert_ne!(
        half_space.clean_fail_boundary_digest(),
        open_domain.clean_fail_boundary_digest()
    );
    assert_eq!(
        half_space.basis().input().open_input_kind(),
        Some(PlanarOpenInputKind::HalfSpaceGroup)
    );
    assert_eq!(
        open_domain.basis().input().open_input_kind(),
        Some(PlanarOpenInputKind::OpenPlanarDomain)
    );
}
