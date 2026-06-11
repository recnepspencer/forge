use worth_spatial::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleFamily,
    PlanarContractBundleValidationContracts, PlanarContractBundleValidator,
};

use super::proof_fixture::{complete_bundle_parts, MOVEMENT, NEIGHBORHOOD, TOPOLOGY};
use super::runtime_handles::bundle_handle;

#[test]
fn planar_contract_bundle_validator_accepts_complete_retained_and_projection_consumed_bundle() {
    let world = "bundle-complete";
    let parts = complete_bundle_parts(world);
    let bundle = complete_bundle(&parts);
    let contracts = PlanarContractBundleValidationContracts::new(bundle_handle(world));
    let plan = PlanarContractBundleValidator::for_boolean_readiness(bundle)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("bundle plan");

    assert_eq!(plan.inspected_bundle_rows(), 11);
    let receipt = plan.certify().expect("bundle readiness receipt");

    assert!(receipt.is_ready_for_m7());
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.basis().family_rows().len(), 11);
    assert_eq!(receipt.counters().inspected_bundle_rows(), 11);
    assert_eq!(receipt.counters().consumed_certificate_families(), 11);
    assert_eq!(receipt.counters().projection_consumed_rows(), 8);
    assert_eq!(receipt.counters().retained_fact_rows(), 20);
    assert_eq!(receipt.counters().support_posture_rows(), 1);
    assert_eq!(
        receipt
            .basis()
            .family_rows()
            .iter()
            .map(|row| (row.family(), row.receipt_count()))
            .collect::<Vec<_>>(),
        vec![
            (PlanarContractBundleFamily::Admission, 1),
            (PlanarContractBundleFamily::TopologyContractCompleteness, 1),
            (PlanarContractBundleFamily::Precision, 1),
            (PlanarContractBundleFamily::LocalFrame, 1),
            (PlanarContractBundleFamily::ProjectionConsumption, 8),
            (PlanarContractBundleFamily::PredicateAuthority, 3),
            (
                PlanarContractBundleFamily::PredicateCertificateConsumption,
                1
            ),
            (PlanarContractBundleFamily::SegmentContact, 1),
            (PlanarContractBundleFamily::PolygonWinding, 1),
            (PlanarContractBundleFamily::SignedArea, 1),
            (PlanarContractBundleFamily::CoplanarOverlap, 1),
        ]
    );
    assert!(!receipt.fact_digest().is_empty());
}

#[test]
fn boolean_readiness_requires_complete_contract_bundle() {
    let world = "boolean-readiness-complete-bundle";
    let parts = complete_bundle_parts(world);
    let contracts = PlanarContractBundleValidationContracts::new(bundle_handle(world));
    let receipt = PlanarContractBundleValidator::for_boolean_readiness(complete_bundle(&parts))
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("final boss bundle plan")
        .certify()
        .expect("final boss readiness receipt");

    assert!(receipt.is_ready_for_m7());
    assert_eq!(
        receipt.basis().movement_rotation_posture_identity(),
        MOVEMENT
    );
    assert_eq!(receipt.basis().topology_basis_identity(), TOPOLOGY);
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert!(receipt
        .basis()
        .family_rows()
        .iter()
        .all(|row| row.receipt_count() > 0));
}

pub(crate) fn complete_bundle(
    parts: &super::proof_fixture::CompleteBundleParts,
) -> PlanarBooleanReadinessBundle {
    PlanarBooleanReadinessBundle::builder()
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
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:bundle")
        .build()
        .expect("complete bundle")
}
