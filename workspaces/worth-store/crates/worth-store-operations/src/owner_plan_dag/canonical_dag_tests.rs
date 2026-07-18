use super::{
    CanonicalOwnerPlanDag, OwnerPlanDagDenial, OwnerPlanEffect, OwnerPlanExecutionStage,
    OwnerPlanFootprint, OwnerPlanNode, StoreOwnerKind,
};

#[test]
fn overlapping_observations_may_run_in_parallel() {
    CanonicalOwnerPlanDag::admit(
        vec![
            observation(StoreOwnerKind::PhysicalIntegrity, [1; 32]),
            observation(StoreOwnerKind::LayoutIndexes, [2; 32]),
        ],
        Vec::new(),
    )
    .expect("read-only owner observations do not invent a write conflict");
}

#[test]
fn an_overlapping_observation_and_mutation_require_explicit_order() {
    let denial = CanonicalOwnerPlanDag::admit(
        vec![
            observation(StoreOwnerKind::PhysicalIntegrity, [1; 32]),
            mutation([2; 32]),
        ],
        Vec::new(),
    )
    .unwrap_err();
    assert_eq!(denial, OwnerPlanDagDenial::AmbiguousOverlappingMutation);
}

#[test]
fn caller_permutation_cannot_change_canonical_owner_plan_meaning() {
    let first = observation(StoreOwnerKind::PhysicalIntegrity, [1; 32]);
    let second = observation(StoreOwnerKind::LayoutIndexes, [2; 32]);
    let forward =
        CanonicalOwnerPlanDag::admit(vec![first.clone(), second.clone()], Vec::new()).unwrap();
    let reverse = CanonicalOwnerPlanDag::admit(vec![second, first], Vec::new()).unwrap();

    assert_eq!(forward.explanation(), reverse.explanation());
    assert_eq!(
        forward.explanation().plan_fingerprint(),
        reverse.explanation().plan_fingerprint()
    );
}

#[test]
fn footprint_expansion_changes_the_plan_identity_before_authorization() {
    let exact =
        CanonicalOwnerPlanDag::admit(vec![observation_with_footprint([3; 32], 0, 10)], Vec::new())
            .unwrap();
    let expanded =
        CanonicalOwnerPlanDag::admit(vec![observation_with_footprint([3; 32], 0, 11)], Vec::new())
            .unwrap();

    assert_ne!(
        exact.explanation().plan_fingerprint(),
        expanded.explanation().plan_fingerprint()
    );
}

fn observation(owner: StoreOwnerKind, identity: [u8; 32]) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_observation_binding(
        owner,
        OwnerPlanEffect::ValidatePhysicalIntegrity,
        OwnerPlanExecutionStage::PostVerification,
        OwnerPlanFootprint::bounded(0, 10).unwrap(),
        10,
        identity,
        identity,
    )
}

fn mutation(identity: [u8; 32]) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_binding(
        StoreOwnerKind::PhysicalBackend,
        OwnerPlanEffect::CopyBackupComponent,
        OwnerPlanFootprint::bounded(0, 10).unwrap(),
        10,
        true,
        identity,
        identity,
    )
}

fn observation_with_footprint(identity: [u8; 32], start: u64, end: u64) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_observation_binding(
        StoreOwnerKind::PhysicalIntegrity,
        OwnerPlanEffect::ValidatePhysicalIntegrity,
        OwnerPlanExecutionStage::PostVerification,
        OwnerPlanFootprint::bounded(start, end).unwrap(),
        end - start,
        identity,
        identity,
    )
}
