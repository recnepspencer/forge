use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts,
    PlanarLocalRebuildParityDenialKind, PlanarLocalRebuildScope, PlanarRebindingContinuityEvidence,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPosture, PlanarMotionPostureContracts,
};

use super::neighborhood_fixture::single_candidate_local_neighborhood_receipt;
use super::parity_fixture::local_rebuild_parity_parts;
use super::runtime_handles::{local_rebuild_handle, motion_posture_handle};

#[test]
fn local_planar_rebuild_denies_broad_search_or_missing_neighborhood_before_identity() {
    let world = "phase-19-missing-neighborhood";
    let parts = local_rebuild_parity_parts(world);
    let denial = match PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:missing-neighborhood",
    ))
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "rebinding-continuation:missing-neighborhood",
        "missing-neighborhood-digest",
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    )) {
        Ok(_) => panic!("missing neighborhood must deny before structural identity emission"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarLocalRebuildParityDenialKind::MissingPlanarReceipt
    );
}

#[test]
fn local_planar_rebuild_rejects_correspondence_only_rebinding() {
    let world = "phase-19-correspondence-only";
    let parts = local_rebuild_parity_parts(world);
    let denial = match PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:correspondence-only",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::correspondence_only(
        "rebinding-correspondence-only",
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    )) {
        Ok(_) => panic!("correspondence-only rebinding must deny"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarLocalRebuildParityDenialKind::CorrespondenceOnlyRebinding
    );
}

#[test]
fn local_planar_rebuild_denies_rebinding_bound_to_another_neighborhood() {
    let world = "phase-19-mismatched-rebinding-neighborhood";
    let parts = local_rebuild_parity_parts(world);
    let denial = match PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:mismatched-rebinding-neighborhood",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "rebinding-continuation:mismatched-neighborhood",
        "another-neighborhood-digest",
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    )) {
        Ok(_) => panic!("rebinding continuity must be bound to the neighborhood receipt"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarLocalRebuildParityDenialKind::MismatchedRebindingNeighborhood
    );
}

#[test]
fn local_planar_rebuild_accepts_single_candidate_local_neighborhood_without_broad_search() {
    let world = "phase-19-single-candidate-local-neighborhood";
    let mut parts = local_rebuild_parity_parts(world);
    parts.neighborhood = single_candidate_local_neighborhood_receipt(world);
    let neighborhood_digest = parts.neighborhood.fact_digest().to_string();
    let receipt = PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:single-candidate",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "rebinding-continuation:single-candidate",
        neighborhood_digest,
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    ))
    .expect("single candidate remains a local neighborhood")
    .certify()
    .expect("single candidate local rebuild parity receipt");

    assert_eq!(receipt.counters().local_neighborhood_rows(), 1);
    assert_eq!(receipt.counters().source_receipts_consumed(), 8);
}

#[test]
fn local_planar_rebuild_rejects_motion_posture_rebuilt_from_different_order() {
    let world = "phase-19-motion-posture-mismatch";
    let parts = local_rebuild_parity_parts(world);
    let neighborhood_digest = parts.neighborhood.fact_digest().to_string();
    let alternate_motion = PlanarMotionPosture::from_boolean_readiness(
        parts
            .retained
            .basis()
            .motion_posture_receipt()
            .basis()
            .boolean_readiness_receipt()
            .clone(),
    )
    .after_exact_rotation("motion:alternate-rotation-first")
    .after_exact_translation("motion:alternate-translation-second")
    .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
        world,
    )))
    .expect("alternate motion posture plan")
    .certify()
    .expect("alternate motion posture receipt");

    let denial = match PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "local-rebuild:motion-posture-mismatch",
    ))
    .local_neighborhood(parts.neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "rebinding-continuation:motion-posture-mismatch",
        neighborhood_digest,
    ))
    .structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(parts.retained.clone())
    .projection_consumed_planar_facts(parts.projected)
    .motion_posture(alternate_motion)
    .topology_contract(parts.retained.basis().topology_contract_receipt().clone())
    .recovery_posture(parts.recovery)
    .diagnostics(parts.diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    )) {
        Ok(_) => panic!("local rebuild must deny motion posture rebuilt from another order"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarLocalRebuildParityDenialKind::MismatchedMotionPostureBasis
    );
}
