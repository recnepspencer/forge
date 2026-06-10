use worth_spatial::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleDenialKind, PlanarContractBundleFamily,
    PlanarContractBundleValidationContracts, PlanarContractBundleValidationFactError,
    PlanarContractBundleValidator,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};

use super::complete_bundle::complete_bundle;
use super::proof_fixture::{
    complete_bundle_parts, stray_projection_receipt, MOVEMENT, NEIGHBORHOOD, TOPOLOGY,
};
use super::runtime_handles::{bundle_handle, predicate_handle};

#[test]
fn planar_contract_bundle_validator_rejects_missing_or_mismatched_certificate_family() {
    let parts = complete_bundle_parts("bundle-denials");

    let missing_topology_contract = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("topology strings without a completeness receipt must deny");
    assert_eq!(
        missing_topology_contract.kind(),
        PlanarContractBundleDenialKind::MissingCertificateFamily
    );
    assert_eq!(
        missing_topology_contract.family(),
        Some(PlanarContractBundleFamily::TopologyContractCompleteness)
    );

    let missing_projection = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("missing projections must deny");
    assert_eq!(
        missing_projection.kind(),
        PlanarContractBundleDenialKind::MissingProjectionConsumption
    );
    assert_eq!(
        missing_projection.family(),
        Some(PlanarContractBundleFamily::ProjectionConsumption)
    );
    assert_eq!(
        missing_projection.counters().rejected_missing_family_rows(),
        1
    );

    let wrong_movement = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture("movement:wrong")
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("movement mismatch must deny");
    assert_eq!(
        wrong_movement.kind(),
        PlanarContractBundleDenialKind::MismatchedMovementRotationPosture
    );

    let wrong_topology = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis("topology:wrong")
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("topology mismatch must deny");
    assert_eq!(
        wrong_topology.kind(),
        PlanarContractBundleDenialKind::TopologyBasisMismatch
    );

    let stale_projection_consumption = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(vec![parts.projections[0].clone()])
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("stale projection consumption must deny");
    assert_eq!(
        stale_projection_consumption.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        stale_projection_consumption.family(),
        Some(PlanarContractBundleFamily::PolygonWinding)
    );
    assert_eq!(
        stale_projection_consumption
            .counters()
            .rejected_missing_family_rows(),
        0
    );

    let complete = complete_bundle(&parts);
    assert_eq!(complete.family_rows().len(), 11);

    let missing_predicate_consumption = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("missing predicate-consumption receipt must deny");
    assert_eq!(
        missing_predicate_consumption.kind(),
        PlanarContractBundleDenialKind::MissingCertificateFamily
    );
    assert_eq!(
        missing_predicate_consumption.family(),
        Some(PlanarContractBundleFamily::PredicateCertificateConsumption)
    );
}

#[test]
fn planar_contract_bundle_validator_rejects_unconsumed_projection_rows() {
    let parts = complete_bundle_parts("bundle-stray-projection");
    let mut projections = parts.projections.clone();
    projections.push(stray_projection_receipt(
        "bundle-stray-projection",
        &parts.frame,
    ));

    let unconsumed_projection = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(projections)
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("unconsumed projection row must deny");

    assert_eq!(
        unconsumed_projection.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        unconsumed_projection.family(),
        Some(PlanarContractBundleFamily::ProjectionConsumption)
    );

    let mut duplicate_projections = parts.projections.clone();
    duplicate_projections.push(parts.projections[0].clone());
    let duplicate_projection = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(duplicate_projections)
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("duplicate projection row must deny");
    assert_eq!(
        duplicate_projection.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        duplicate_projection.family(),
        Some(PlanarContractBundleFamily::ProjectionConsumption)
    );
}

#[test]
fn planar_contract_bundle_validator_rejects_unconsumed_predicate_and_segment_rows() {
    let parts = complete_bundle_parts("bundle-stray-predicate");
    let mut predicates = parts.predicates.clone();
    predicates.push(stray_predicate_authority_receipt());

    let unconsumed_predicate = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(predicates)
        .segment_contacts(parts.segments.clone())
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("unconsumed predicate authority row must deny");
    assert_eq!(
        unconsumed_predicate.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        unconsumed_predicate.family(),
        Some(PlanarContractBundleFamily::PredicateAuthority)
    );

    let mut duplicate_segments = parts.segments.clone();
    duplicate_segments.push(parts.segments[0].clone());
    let duplicate_segment = PlanarBooleanReadinessBundle::builder()
        .admission(parts.admission.clone())
        .topology_contract(parts.topology_contract.clone())
        .precision(parts.precision.clone())
        .local_frame(parts.frame.clone())
        .projection_consumption(parts.projections.clone())
        .predicate_authority(parts.predicates.clone())
        .segment_contacts(duplicate_segments)
        .winding(parts.winding.clone())
        .signed_area(parts.signed_area.clone())
        .coplanar_overlap(parts.overlap.clone())
        .predicate_consumption(parts.predicate_consumption.clone())
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect_err("duplicate segment-contact row must deny");
    assert_eq!(
        duplicate_segment.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        duplicate_segment.family(),
        Some(PlanarContractBundleFamily::PredicateCertificateConsumption)
    );
}

#[test]
fn planar_contract_bundle_validator_rejects_stale_planar_neighborhood_context() {
    let parts = complete_bundle_parts("bundle-stale-neighborhood");
    let stale_neighborhood_contracts =
        PlanarContractBundleValidationContracts::new(bundle_handle("bundle-stale-neighborhood"));
    let stale_neighborhood_result =
        PlanarContractBundleValidator::for_boolean_readiness(complete_bundle(&parts))
            .within_planar_neighborhood("neighborhood:wrong")
            .compile(&stale_neighborhood_contracts);
    let denial = match stale_neighborhood_result {
        Err(denial) => denial,
        Ok(_) => panic!("stale neighborhood must deny before Query certification"),
    };

    let PlanarContractBundleValidationFactError::BundleBasis { denial } = denial else {
        panic!("stale neighborhood must produce localized bundle-basis denial");
    };
    assert_eq!(
        denial.kind(),
        PlanarContractBundleDenialKind::MismatchedCertificateFamily
    );
    assert_eq!(
        denial.family(),
        Some(PlanarContractBundleFamily::TopologyContractCompleteness)
    );

    let matching_neighborhood_contracts =
        PlanarContractBundleValidationContracts::new(bundle_handle("bundle-stale-neighborhood"));
    PlanarContractBundleValidator::for_boolean_readiness(complete_bundle(&parts))
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&matching_neighborhood_contracts)
        .expect("matching neighborhood remains admissible");
}

fn stray_predicate_authority_receipt() -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:bundle",
        TOPOLOGY,
        MOVEMENT,
        "tolerance:bundle",
        [[0.0, 0.0], [3.0e-9, 0.0], [0.0, 3.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("stray predicate authority receipt")
}
