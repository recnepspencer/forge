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
