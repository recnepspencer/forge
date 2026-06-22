use std::collections::BTreeMap;

use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionContracts,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

use super::runtime_handles::predicate_consumption_handle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, MOVEMENT, TOPOLOGY,
};
use crate::public_api_planar_contract_bundle::runtime_handles::predicate_handle;

#[test]
fn predicate_certificate_consumption_validator_accepts_only_worth_math_certified_signs() {
    let world = "predicate-consumption-certified";
    let parts = complete_bundle_parts(world);
    let segment = parts.segments[0].clone();
    let predicate_receipts = segment_orientation_predicates(&segment);
    let contracts =
        PredicateCertificateConsumptionContracts::new(predicate_consumption_handle(world));

    let plan = PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:bundle")
        .with_predicate_authority(predicate_receipts)
        .with_segment_contacts(vec![segment])
        .compile(&contracts)
        .expect("predicate consumption plan");

    assert_eq!(plan.inspected_predicate_rows(), 4);
    let receipt = plan.certify().expect("predicate consumption receipt");
    assert_eq!(receipt.certified_predicate_rows(), 4);
    assert_eq!(receipt.counters().consumer_rows(), 4);
    assert_eq!(receipt.counters().precision_metadata_rows(), 4);
    assert_eq!(receipt.counters().rejected_substitute_rows(), 0);
    assert!(receipt.proves_no_second_predicate_engine());
    assert!(receipt.basis().consumption_rows().iter().all(|row| {
        !row.certified_sign_identity().is_empty()
            && !row.precision_escalation_identity().is_empty()
            && row.local_frame_identity() == "frame:bundle"
            && row.topology_basis_identity() == TOPOLOGY
            && row.movement_rotation_posture_identity() == MOVEMENT
    }));
}

pub(crate) fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
) -> Vec<PlanarPredicateFactReceipt> {
    let basis = segment.basis();
    let orientation_points = [
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_start_point_2d(),
        ],
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_end_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_start_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_end_point_2d(),
        ],
    ];
    let mut receipts_by_digest = BTreeMap::new();
    for points in orientation_points {
        let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
            basis.frame_identity(),
            basis.topology_basis_identity(),
            basis.movement_rotation_posture_identity(),
            basis.tolerance_policy_identity(),
            points,
        );
        let receipt = planar_predicate_authority_facts(
            &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
                predicate_basis,
            )),
            &predicate_handle(),
        )
        .expect("segment orientation predicate receipt");
        receipts_by_digest.insert(receipt.fact_digest().to_string(), receipt);
    }
    receipts_by_digest.into_values().collect()
}
