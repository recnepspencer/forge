use std::collections::BTreeSet;

use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumerKind, PredicateCertificateConsumption,
    PredicateCertificateConsumptionContracts,
};

use super::certified_consumption::segment_orientation_predicates;
use super::runtime_handles::predicate_consumption_handle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, MOVEMENT, TOPOLOGY,
};

#[test]
fn mb_m6_7_projection_consumed_planar_fact_parity_requires_predicate_metadata() {
    let world = "predicate-consumption-parity";
    let parts = complete_bundle_parts(world);
    let segment = parts.segments[0].clone();
    let predicate_receipts = segment_orientation_predicates(&segment);
    let predicate_digests = predicate_receipts
        .iter()
        .map(|receipt| receipt.fact_digest().to_string())
        .collect::<BTreeSet<_>>();
    let contracts =
        PredicateCertificateConsumptionContracts::new(predicate_consumption_handle(world));

    let receipt = PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:bundle")
        .with_predicate_authority(predicate_receipts)
        .with_segment_contacts(vec![segment])
        .compile(&contracts)
        .expect("predicate parity plan")
        .certify()
        .expect("predicate parity receipt");

    let consumed_digests = receipt
        .basis()
        .consumption_rows()
        .iter()
        .map(|row| {
            assert_eq!(
                row.consumer_kind(),
                PredicateCertificateConsumerKind::SegmentContact
            );
            assert!(!row.certified_sign_identity().is_empty());
            assert!(!row.precision_escalation_identity().is_empty());
            assert!(!row.predicate_declaration_digest().is_empty());
            assert!(!row.predicate_envelope_digest().is_empty());
            assert_eq!(row.local_frame_identity(), "frame:bundle");
            assert_eq!(row.topology_basis_identity(), TOPOLOGY);
            assert_eq!(row.movement_rotation_posture_identity(), MOVEMENT);
            row.predicate_fact_digest().to_string()
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(consumed_digests, predicate_digests);
    assert_eq!(receipt.counters().certified_predicate_rows(), 4);
    assert_eq!(receipt.counters().precision_metadata_rows(), 4);
    assert_eq!(receipt.counters().rejected_substitute_rows(), 0);
}
