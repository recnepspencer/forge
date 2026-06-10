use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionDenial,
    PredicateCertificateConsumptionDenialKind,
};

use super::certified_consumption::segment_orientation_predicates;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, MOVEMENT, TOPOLOGY,
};

#[test]
fn predicate_certificate_consumption_validator_rejects_epsilon_topology_and_kernel_summary_substitutes(
) {
    let parts = complete_bundle_parts("predicate-consumption-denials");
    let segment = parts.segments[0].clone();
    let predicate_receipts = segment_orientation_predicates(&segment);

    let missing_authority = compile_denial(
        PredicateCertificateConsumption::for_planar_workload()
            .expecting_topology_basis(TOPOLOGY)
            .expecting_movement_rotation_posture(MOVEMENT)
            .expecting_local_frame("frame:bundle")
            .with_segment_contacts(vec![segment.clone()]),
    );
    assert_eq!(
        missing_authority.kind(),
        PredicateCertificateConsumptionDenialKind::MissingPredicateAuthority
    );

    let mut duplicate_predicates = predicate_receipts.clone();
    duplicate_predicates.push(predicate_receipts[0].clone());
    let duplicate_authority = compile_denial(
        PredicateCertificateConsumption::for_planar_workload()
            .expecting_topology_basis(TOPOLOGY)
            .expecting_movement_rotation_posture(MOVEMENT)
            .expecting_local_frame("frame:bundle")
            .with_predicate_authority(duplicate_predicates)
            .with_segment_contacts(vec![segment.clone()]),
    );
    assert_eq!(
        duplicate_authority.kind(),
        PredicateCertificateConsumptionDenialKind::DuplicatePredicateReceipt
    );

    let wrong_topology = compile_denial(
        PredicateCertificateConsumption::for_planar_workload()
            .expecting_topology_basis("topology:summary-substitute")
            .expecting_movement_rotation_posture(MOVEMENT)
            .expecting_local_frame("frame:bundle")
            .with_predicate_authority(predicate_receipts.clone())
            .with_segment_contacts(vec![segment.clone()]),
    );
    assert_eq!(
        wrong_topology.kind(),
        PredicateCertificateConsumptionDenialKind::TopologyBasisMismatch
    );

    let wrong_movement = compile_denial(
        PredicateCertificateConsumption::for_planar_workload()
            .expecting_topology_basis(TOPOLOGY)
            .expecting_movement_rotation_posture("movement:epsilon-substitute")
            .expecting_local_frame("frame:bundle")
            .with_predicate_authority(predicate_receipts.clone())
            .with_segment_contacts(vec![segment.clone()]),
    );
    assert_eq!(
        wrong_movement.kind(),
        PredicateCertificateConsumptionDenialKind::MovementRotationPostureMismatch
    );

    let unconsumed = compile_denial(
        PredicateCertificateConsumption::for_planar_workload()
            .expecting_topology_basis(TOPOLOGY)
            .expecting_movement_rotation_posture(MOVEMENT)
            .expecting_local_frame("frame:bundle")
            .with_predicate_authority(vec![predicate_receipts[0].clone()])
            .with_segment_contacts(vec![segment]),
    );
    assert_eq!(
        unconsumed.kind(),
        PredicateCertificateConsumptionDenialKind::MissingConsumedPredicateReceipt
    );
}

fn compile_denial(
    intent: PredicateCertificateConsumption,
) -> PredicateCertificateConsumptionDenial {
    let contracts =
        worth_spatial::facade::planar_predicate_consumption::PredicateCertificateConsumptionContracts::new(
        super::runtime_handles::predicate_consumption_handle("predicate-consumption-denials"),
    );
    match intent.compile(&contracts) {
        Err(denial) => denial,
        Ok(_) => panic!("predicate consumption substitute must deny before certification"),
    }
}
