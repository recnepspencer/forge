use std::collections::BTreeMap;

use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionContracts,
    PredicateCertificateConsumptionReceipt,
};
use worth_spatial::facade::planar_predicates::PlanarPredicateFactReceipt;
use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

use super::contract_handles::predicate_consumption_handle;
use super::projection::{predicate_receipt, MOVEMENT, TOPOLOGY};

pub(crate) fn predicate_consumption_receipt(
    world: &'static str,
    segments: Vec<CertifiedSegmentSegment2DReceipt>,
) -> PredicateCertificateConsumptionReceipt {
    let predicates = unique_orientation_predicates(&segments);
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame(segments[0].basis().frame_identity())
        .with_predicate_authority(predicates)
        .with_segment_contacts(segments)
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(world),
        ))
        .expect("collinear-relation predicate consumption plan")
        .certify()
        .expect("collinear-relation predicate consumption receipt")
}

fn unique_orientation_predicates(
    segments: &[CertifiedSegmentSegment2DReceipt],
) -> Vec<PlanarPredicateFactReceipt> {
    let mut receipts = BTreeMap::new();
    for segment in segments {
        for receipt in segment_orientation_predicates(segment) {
            receipts.insert(receipt.fact_digest().to_string(), receipt);
        }
    }
    receipts.into_values().collect()
}

fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
) -> Vec<PlanarPredicateFactReceipt> {
    let basis = segment.basis();
    [
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
    ]
    .into_iter()
    .map(|points| {
        predicate_receipt(
            basis.frame_identity(),
            basis.tolerance_policy_identity(),
            points,
        )
    })
    .collect()
}
