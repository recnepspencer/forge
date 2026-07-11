use super::{
    compaction_cutover_evidence_for_certification_plan,
    compaction_read_interlock_plan_for_certification_test, CompactionCutoverDelta,
    CompactionCutoverStabilityProof, CompactionCutoverState, CompactionCutoverTransitionKind,
    CompactionDeferredReclaimQueue, CompactionMutationLaneReceipt,
    CompactionMutationLaneReceiptKind,
};

#[test]
fn real_cutover_typestates_emit_the_formal_model_states() {
    let plan = compaction_read_interlock_plan_for_certification_test();
    assert_eq!(plan.cutover_state(), CompactionCutoverState::PlanAdmitted);

    let (publication, recovery, pre_cutover_read, _) =
        compaction_cutover_evidence_for_certification_plan(&plan).into_parts();
    assert_eq!(
        publication.delta().cutover_state(),
        CompactionCutoverState::RewriteLowered
    );
    assert_eq!(
        publication.delta().cutover_transition(),
        CompactionCutoverTransitionKind::LowerRewrite.transition()
    );
    assert_eq!(
        publication.cutover_transition(),
        CompactionCutoverTransitionKind::PublishRewrite.transition()
    );
    assert_eq!(
        publication.cutover_state(),
        CompactionCutoverState::PublicationCommitted
    );

    let proof = CompactionCutoverStabilityProof::admit(publication.clone(), recovery).unwrap();
    assert_eq!(
        proof.cutover_state(),
        CompactionCutoverState::RecoveryVisibilityAdmitted
    );
    assert_eq!(
        proof.cutover_transition(),
        CompactionCutoverTransitionKind::AdmitRecoveryVisibility.transition()
    );

    let reclaim = CompactionDeferredReclaimQueue::admit(publication).unwrap();
    assert_eq!(
        reclaim.cutover_state(),
        CompactionCutoverState::ReclaimDeferred
    );
    assert_eq!(
        reclaim.cutover_transition(),
        CompactionCutoverTransitionKind::DeferReclaim.transition()
    );
    let drained = reclaim
        .drain_after_release(pre_cutover_read.read_plan_release())
        .unwrap();
    assert_eq!(drained.cutover_state(), CompactionCutoverState::Reclaimed);
    assert_eq!(
        drained.cutover_transition(),
        CompactionCutoverTransitionKind::DrainReclaimAfterReadRelease.transition()
    );
}

#[test]
fn ordinary_mutation_denials_emit_the_aggregated_owner_facts() {
    use crate::{pre_wait_denial_for_hierarchy_inversion, LatchAcquisitionStep, PhysicalLatchKey};
    use forge_store_recovery_physics::{
        CompactionCutoverRecoveryPosture, RecoveryCandidateDiscoveryTrace,
    };

    let plan = compaction_read_interlock_plan_for_certification_test();
    let (publication, _, _, _) =
        compaction_cutover_evidence_for_certification_plan(&plan).into_parts();
    let queue = CompactionDeferredReclaimQueue::admit(publication.clone()).unwrap();
    let (in_place, _) = CompactionMutationLaneReceipt::from_in_place_overwrite_denial(plan.clone());
    let (early_reclaim, _) = CompactionMutationLaneReceipt::from_early_reclaim_denial(&queue);
    let backend_residue = CompactionMutationLaneReceipt::from_backend_residue_denial(
        publication,
        CompactionCutoverRecoveryPosture::missing_generation_identity(
            RecoveryCandidateDiscoveryTrace::new("phase34", "backend-residue", 1),
        ),
    )
    .unwrap();
    let mut inverted = [
        LatchAcquisitionStep::shared(PhysicalLatchKey::root(plan.target_epoch())),
        LatchAcquisitionStep::shared(PhysicalLatchKey::root(plan.source_epoch())),
    ];
    if pre_wait_denial_for_hierarchy_inversion(&inverted)
        .unwrap()
        .is_none()
    {
        inverted.reverse();
    }
    let latch = CompactionMutationLaneReceipt::from_latch_hierarchy_inversion_denial(
        &plan,
        pre_wait_denial_for_hierarchy_inversion(&inverted)
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let mixed_root = CompactionMutationLaneReceipt::from_mixed_root_read_denial(&plan);

    for (receipt, expected_kind) in [
        (
            in_place,
            CompactionMutationLaneReceiptKind::InPlaceOverwriteDenied,
        ),
        (
            early_reclaim,
            CompactionMutationLaneReceiptKind::EarlyReclaimDenied,
        ),
        (
            backend_residue,
            CompactionMutationLaneReceiptKind::BackendResidueCandidateSelectionDenied,
        ),
        (
            latch,
            CompactionMutationLaneReceiptKind::LatchHierarchyInversionDenied,
        ),
        (
            mixed_root,
            CompactionMutationLaneReceiptKind::MixedRootReadDenied,
        ),
    ] {
        assert_eq!(receipt.kind(), expected_kind);
        assert!(super::compaction_cutover_outcome_facts()
            .any(|fact| fact == receipt.cutover_transition()));
    }
}
