use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryAction, PlanarRecoveryBlockerKind, PlanarRecoveryPosture,
    PlanarRecoveryPostureContracts, PlanarRecoverySourceKind, PlanarRecoverySourcePosture,
    PlanarRecoveryTargetScope, PlanarRecoveryTruthEffect,
};

use super::contract_subject::{
    planar_recovery_parts, projection_basis_source, retained_projection_basis_source,
};
use super::runtime_handles::recovery_handle;

#[test]
fn planar_recovery_consumes_typed_denial_without_reclassifying_truth() {
    let world = "planar-recovery-typed-denial";
    let parts = planar_recovery_parts(world);
    let contracts = PlanarRecoveryPostureContracts::new(recovery_handle(world));
    let plan = PlanarRecoveryPosture::from_blocked_planar_source(projection_basis_source())
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .prepare_next_step()
        .compile(&contracts)
        .expect("planar recovery plan");

    assert_eq!(plan.inspected_recovery_rows(), 6);
    let receipt = plan.certify().expect("planar recovery receipt");

    assert_eq!(
        receipt.blocker_kind(),
        PlanarRecoveryBlockerKind::ProjectionBasis
    );
    assert_eq!(
        receipt.basis().source().kind(),
        PlanarRecoverySourceKind::ProjectionBasisDenial
    );
    assert_eq!(
        receipt.source_posture(),
        PlanarRecoverySourcePosture::Denied
    );
    assert_eq!(
        receipt.recovery_action(),
        PlanarRecoveryAction::InspectProjectionBasis
    );
    assert_eq!(
        receipt.target_scope(),
        PlanarRecoveryTargetScope::ProjectionBasisInspection
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth
    );
    assert_eq!(receipt.counters().source_rows_inspected(), 1);
    assert_eq!(receipt.counters().basis_receipts_consumed(), 2);
    assert_eq!(receipt.counters().recovery_rows_emitted(), 1);
    assert_eq!(receipt.counters().rejected_basis_rows(), 0);
    assert_eq!(receipt.counters().recovery_breadth(), 6);
}

#[test]
fn planar_recovery_canonical_source_rows_stay_stable_for_same_basis() {
    let world = "planar-recovery-stable";
    let parts = planar_recovery_parts(world);
    let contracts = PlanarRecoveryPostureContracts::new(recovery_handle(world));
    let first =
        PlanarRecoveryPosture::from_blocked_planar_source(retained_projection_basis_source())
            .with_projection_consumed_facts(parts.projected.clone())
            .with_retained_planar_facts(parts.retained.clone())
            .prepare_next_step()
            .compile(&contracts)
            .expect("first recovery plan")
            .certify()
            .expect("first recovery receipt");
    let second =
        PlanarRecoveryPosture::from_blocked_planar_source(retained_projection_basis_source())
            .with_retained_planar_facts(parts.retained)
            .with_projection_consumed_facts(parts.projected)
            .prepare_next_step()
            .compile(&contracts)
            .expect("second recovery plan")
            .certify()
            .expect("second recovery receipt");

    assert_eq!(first.declaration_digest(), second.declaration_digest());
    assert_eq!(
        first.recovery_posture_digest(),
        second.recovery_posture_digest()
    );
    assert_eq!(first.progression_digest(), second.progression_digest());
    assert_eq!(first.route_plan_digest(), second.route_plan_digest());
    assert_eq!(first.query_receipt_digest(), second.query_receipt_digest());
    assert_eq!(first.envelope_digest(), second.envelope_digest());
    assert_eq!(
        first.basis().source().kind(),
        PlanarRecoverySourceKind::RetainedOrProjectionBasisDenial
    );
    assert_eq!(
        first.target_scope(),
        PlanarRecoveryTargetScope::RetainedProjectionBasisInspection
    );
}

#[test]
fn dirty_and_unbounded_recovery_rows_remain_typed() {
    let dirty_world = "planar-recovery-dirty";
    let dirty = PlanarRecoveryPosture::from_blocked_planar_source(
        worth_spatial::facade::planar_recovery::PlanarRecoverySource::dirty_input(
            "dirty:self-intersection",
        ),
    )
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(
        dirty_world,
    )))
    .expect("dirty recovery plan")
    .certify()
    .expect("dirty recovery receipt");
    assert_eq!(dirty.blocker_kind(), PlanarRecoveryBlockerKind::DirtyInput);
    assert_eq!(dirty.source_posture(), PlanarRecoverySourcePosture::Dirty);
    assert_eq!(
        dirty.recovery_action(),
        PlanarRecoveryAction::InspectTopologyAndInputCleanliness
    );
    assert_eq!(
        dirty.target_scope(),
        PlanarRecoveryTargetScope::InputCleanlinessInspection
    );
    assert_eq!(dirty.counters().basis_receipts_consumed(), 0);

    let unbounded_world = "planar-recovery-unbounded";
    let unbounded = PlanarRecoveryPosture::from_blocked_planar_source(
        worth_spatial::facade::planar_recovery::PlanarRecoverySource::unbounded_or_open(
            "unbounded:half-space",
        ),
    )
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(
        unbounded_world,
    )))
    .expect("unbounded recovery plan")
    .certify()
    .expect("unbounded recovery receipt");
    assert_eq!(
        unbounded.blocker_kind(),
        PlanarRecoveryBlockerKind::UnsupportedPlanarClass
    );
    assert_eq!(
        unbounded.recovery_action(),
        PlanarRecoveryAction::ClassifyWithoutBoundedConversion
    );
    assert_eq!(
        unbounded.source_posture(),
        PlanarRecoverySourcePosture::Unsupported
    );
    assert_eq!(
        unbounded.target_scope(),
        PlanarRecoveryTargetScope::SupportReadiness
    );
    assert_eq!(
        unbounded.truth_effect(),
        PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth
    );
}
