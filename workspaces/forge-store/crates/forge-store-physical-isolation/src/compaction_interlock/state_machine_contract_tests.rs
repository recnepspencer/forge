use super::{
    compaction_cutover_evidence_for_certification_plan,
    compaction_read_interlock_plan_for_certification_test, CompactionCutoverStabilityProof,
    CompactionCutoverState, CompactionDeferredReclaimQueue, CompactionMutationLaneReceipt,
    CompactionMutationLaneReceiptKind,
};
use std::collections::BTreeSet;

#[test]
fn independently_admitted_equal_looking_reader_horizons_do_not_alias() {
    let first = super::compaction_read_interlock_plan_for_certification_root_seed(17);
    let second = super::compaction_read_interlock_plan_for_certification_root_seed(17);

    assert_eq!(first.protected(), second.protected());
    assert_eq!(first.candidates(), second.candidates());
    assert_eq!(first.source_epoch(), second.source_epoch());
    assert_eq!(first.target_epoch(), second.target_epoch());
    assert_ne!(first, second);
    assert_eq!(first, first.clone());
}

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
        publication.delta().owner_case().id().name(),
        "physical.compaction.lower_rewrite"
    );
    assert_eq!(
        publication.owner_case().id().name(),
        "physical.compaction.publish_rewrite"
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
        proof.owner_case().id().name(),
        "physical.compaction.admit_recovery_visibility"
    );

    let reclaim = CompactionDeferredReclaimQueue::admit(publication).unwrap();
    assert_eq!(
        reclaim.cutover_state(),
        CompactionCutoverState::ReclaimDeferred
    );
    assert_eq!(
        reclaim.owner_case().id().name(),
        "physical.compaction.defer_reclaim"
    );
    let drained = reclaim
        .drain_after_release(pre_cutover_read.read_plan_release())
        .unwrap();
    assert_eq!(drained.cutover_state(), CompactionCutoverState::Reclaimed);
    assert_eq!(
        drained.owner_case().id().name(),
        "physical.compaction.drain_reclaim_after_read_release"
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
            RecoveryCandidateDiscoveryTrace::new(
                "compaction-owner-inventory",
                "backend-residue",
                1,
            ),
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
        assert!(super::compaction_owner_case_inventory().any(|case| case == receipt.owner_case()));
    }
}

#[test]
fn advertised_inventory_equals_cases_emitted_by_ordinary_owner_operations() {
    use crate::{pre_wait_denial_for_hierarchy_inversion, LatchAcquisitionStep, PhysicalLatchKey};
    use forge_store_recovery_physics::{
        CompactionCutoverRecoveryPosture, RecoveryCandidateDiscoveryTrace,
    };

    let plan = compaction_read_interlock_plan_for_certification_test();
    let (publication, recovery, pre_cutover_read, _) =
        compaction_cutover_evidence_for_certification_plan(&plan).into_parts();
    let proof = CompactionCutoverStabilityProof::admit(publication.clone(), recovery).unwrap();
    let queue = CompactionDeferredReclaimQueue::admit(publication.clone()).unwrap();
    let drained = queue
        .clone()
        .drain_after_release(pre_cutover_read.read_plan_release())
        .unwrap();

    let (in_place, _) = CompactionMutationLaneReceipt::from_in_place_overwrite_denial(plan.clone());
    let (early_reclaim, _) = CompactionMutationLaneReceipt::from_early_reclaim_denial(&queue);
    let stale_source =
        crate::RootEpoch::from_admitted_physical_basis(plan.source_epoch().get() + 1);
    let stale_target =
        crate::RootEpoch::from_admitted_physical_basis(plan.target_epoch().get() + 1);
    let stale = CompactionMutationLaneReceipt::from_stale_epoch_admission_denial(
        &plan,
        stale_source,
        stale_target,
        plan.source_integrity(),
    )
    .unwrap();
    let backend_residue = CompactionMutationLaneReceipt::from_backend_residue_denial(
        publication,
        CompactionCutoverRecoveryPosture::missing_generation_identity(
            RecoveryCandidateDiscoveryTrace::new(
                "compaction-owner-inventory",
                "backend-residue",
                1,
            ),
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

    let observed = [
        Some(proof.publication().delta().owner_case()),
        Some(proof.publication().owner_case()),
        Some(proof.owner_case()),
        Some(queue.owner_case()),
        Some(drained.owner_case()),
        Some(in_place.owner_case()),
        Some(early_reclaim.owner_case()),
        Some(stale.owner_case()),
        Some(backend_residue.owner_case()),
        Some(latch.owner_case()),
        Some(mixed_root.owner_case()),
    ]
    .into_iter()
    .flatten()
    .map(|case| case.id().name())
    .collect::<BTreeSet<_>>();
    let advertised = super::compaction_owner_case_inventory()
        .map(|case| case.id().name())
        .collect::<BTreeSet<_>>();

    assert_eq!(observed, advertised);
    assert_eq!(advertised.len(), 11);
}
