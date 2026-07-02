#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_copy_on_write_publication/crash_recovery.rs"]
mod crash_recovery;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s5_copy_on_write_publication/support.rs"]
mod publication_support;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
mod support;

use forge_foundational::{
    FoundationalBoundaryEvidenceContinuityAttachmentScope, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRowFamily,
};
use forge_store_physical_isolation::{
    AllocatorPublicationFence, CrashStableFreeReusePosture, NewRootPublicationProof,
    PhysicalIdentityReuse, PhysicalOrderingContract, PhysicalOrderingSite,
    PhysicalPublicationDenial, PhysicalPublicationIntent, PhysicalPublicationReadiness,
    PhysicalPublicationReleasePosture, PublicationCrashRecoveryOutcome, PublicationLatchReadiness,
    PublicationRootCandidate, ReadCopyUpdateRootPublication, RootSwapOrderingContract,
};
use forge_store_recovery_physics::S5PublicationCrashStage;
use publication_support::{
    execute_publication_recovery_replay, mismatched_release_receipt, publication_inputs,
    publish_copy_on_write, publish_copy_on_write_result, root_publication_validation,
};
use support::current_generation_page_reference;

#[test]
fn copy_on_write_publication_preserves_old_reachability_and_publishes_new_root() {
    let inputs = publication_inputs();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );

    assert_eq!(
        receipt.epochs().root().old().get(),
        inputs.old_root.epoch().get()
    );
    assert_eq!(
        receipt.epochs().root().new().get(),
        inputs.new_root.epoch().get()
    );
    assert_eq!(
        receipt.epochs().manifest().old().get(),
        inputs.old_root.manifest_epoch().get()
    );
    assert_eq!(
        receipt.epochs().manifest().new().get(),
        inputs.new_root.manifest_epoch().get()
    );
    assert_eq!(
        receipt.release_posture(),
        PhysicalPublicationReleasePosture::OldReachabilityRetainedUntilReadRelease
    );
    assert!(receipt.old_reachability().retained_until_release());
    assert_eq!(receipt.counters().intent_validations(), 1);
    assert_eq!(receipt.counters().old_reachability_checks(), 1);
    assert_eq!(receipt.counters().epoch_checks(), 1);
    assert_eq!(receipt.counters().ordering_checks(), 1);
    assert_eq!(receipt.counters().readiness_joins(), 1);
    assert_eq!(receipt.counters().root_swaps(), 1);

    for stage in [
        S5PublicationCrashStage::BeforePublication,
        S5PublicationCrashStage::DuringPublication,
        S5PublicationCrashStage::AfterPublication,
    ] {
        let recovery_receipt = execute_publication_recovery_replay(stage);
        let outcome =
            PublicationCrashRecoveryOutcome::admit_recovery_receipt(&receipt, recovery_receipt)
                .unwrap();
        assert!(!outcome.mixed_tree());
    }

    let foundational = receipt.lower_to_foundational_evidence();
    assert_eq!(
        foundational.executed_receipt().receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Execution
    );
    assert_eq!(foundational.diagnostic_rows().len(), 1);
    assert_eq!(
        foundational.diagnostic_rows()[0].family(),
        FoundationalDiagnosticRowFamily::ProvenanceReady
    );
    assert_eq!(
        foundational.diagnostic_rows()[0].outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Accepted
    );
    assert_eq!(
        foundational.lineage().subject().handle().get(),
        inputs.new_root.epoch().get()
    );
    assert_eq!(
        foundational
            .lineage()
            .related_subjects()
            .unwrap()
            .subjects()[0]
            .handle()
            .get(),
        inputs.old_root.epoch().get()
    );
    assert_eq!(
        foundational.continuity_scope(),
        FoundationalBoundaryEvidenceContinuityAttachmentScope::ObjectLevel
    );
    assert_eq!(
        receipt
            .admit_old_reachability_release(inputs.old_release)
            .unwrap()
            .footprint_basis(),
        inputs.old_reachability.footprint_basis()
    );
    assert!(matches!(
        receipt
            .admit_old_reachability_release(mismatched_release_receipt(777))
            .unwrap_err(),
        PhysicalPublicationDenial::ReclaimBeforeReadPlanRelease { .. }
    ));
}

#[test]
fn publication_exposes_post_swap_reader_root() {
    let inputs = publication_inputs();
    let publication = publish_copy_on_write_result(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );

    assert_eq!(
        publication.root_swap().pre_swap_reader_root().epoch().get(),
        inputs.old_root.epoch().get()
    );
    assert_eq!(
        publication
            .root_swap()
            .post_swap_reader_root()
            .epoch()
            .get(),
        inputs.new_root.epoch().get()
    );
}

#[test]
fn publication_denies_in_place_overwrite_and_missing_old_reachability() {
    let inputs = publication_inputs();
    let in_place = PhysicalPublicationIntent::in_place_reachable_overwrite_attempt(
        inputs.old_candidate,
        inputs.new_candidate,
        inputs.old_reachability,
    )
    .validate_copy_on_write_inputs()
    .unwrap_err();
    assert_eq!(
        in_place,
        PhysicalPublicationDenial::InPlaceReachableOverwrite
    );

    let missing = PhysicalPublicationIntent::missing_old_reachability_attempt(
        inputs.old_candidate,
        inputs.new_candidate,
    )
    .validate_copy_on_write_inputs()
    .unwrap_err();
    assert_eq!(missing, PhysicalPublicationDenial::MissingOldReachability);
}

#[test]
fn publication_denies_stale_epochs_and_weak_root_swap_ordering() {
    let inputs = publication_inputs();
    let stale = PhysicalPublicationIntent::copy_on_write_root_manifest(
        inputs.old_candidate,
        inputs.old_candidate,
        inputs.old_reachability,
    )
    .validate_copy_on_write_inputs()
    .unwrap_err();
    assert_eq!(stale, PhysicalPublicationDenial::StaleRootPublicationEpoch);

    let weak = RootSwapOrderingContract::from_ordering(
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::Validation),
    )
    .unwrap_err();
    assert!(matches!(weak, PhysicalPublicationDenial::WeakOrdering(_)));

    let intent = PhysicalPublicationIntent::copy_on_write_root_manifest(
        inputs.old_candidate,
        inputs.new_candidate,
        inputs.old_reachability,
    )
    .validate_copy_on_write_inputs()
    .unwrap();
    let stronger = RootSwapOrderingContract::from_ordering(
        PhysicalOrderingContract::sequentially_consistent_for(PhysicalOrderingSite::RootSwap),
    )
    .unwrap();
    let lowered = intent.clone().lower_with_ordering(stronger).unwrap();
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &intent,
        NewRootPublicationProof::from_root_validation(inputs.new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    let publication =
        ReadCopyUpdateRootPublication::publish(lowered.join_readiness(readiness).unwrap()).unwrap();
    assert_eq!(
        publication.receipt().ordering().ordering().strength(),
        forge_store_physical_isolation::PhysicalOrderingStrength::SequentiallyConsistent
    );
}

#[test]
fn identity_reuse_requires_allocator_publication_fence() {
    let inputs = publication_inputs();
    let intent = PhysicalPublicationIntent::copy_on_write_root_manifest_with_identity_reuse(
        inputs.old_candidate,
        inputs.new_candidate,
        inputs.old_reachability,
    )
    .validate_copy_on_write_inputs()
    .unwrap();
    assert_eq!(intent.identity_reuse(), PhysicalIdentityReuse::Requested);
    let lowered = intent
        .clone()
        .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
        .unwrap();
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &intent,
        NewRootPublicationProof::from_root_validation(inputs.new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    let denial = lowered.join_readiness(readiness).unwrap_err();
    assert_eq!(
        denial,
        PhysicalPublicationDenial::IdentityReuseWithoutCrashStableFence
    );

    let fence = AllocatorPublicationFence::from_ordering(
        PhysicalOrderingContract::sequentially_consistent_for(
            PhysicalOrderingSite::AllocatorPublication,
        ),
    )
    .unwrap();
    let old_identity = current_generation_page_reference(701);
    let reused_identity = current_generation_page_reference(702);
    let released = inputs
        .old_reachability
        .admit_release(inputs.old_release)
        .unwrap();
    let reuse =
        CrashStableFreeReusePosture::admit(fence, old_identity, reused_identity, released).unwrap();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest_with_identity_reuse(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        Some(reuse),
    );
    assert_eq!(
        receipt.release_posture(),
        PhysicalPublicationReleasePosture::IdentityReuseProtectedByAllocatorFence
    );
    assert_eq!(receipt.free_reuse(), Some(reuse));
    assert_eq!(
        reuse.fence().ordering().strength(),
        forge_store_physical_isolation::PhysicalOrderingStrength::SequentiallyConsistent
    );
}

#[test]
fn identity_reuse_denies_owner_mismatch_and_stale_generation() {
    let inputs = publication_inputs();
    let released = inputs
        .old_reachability
        .admit_release(inputs.old_release)
        .unwrap();
    let fence = AllocatorPublicationFence::from_ordering(
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::AllocatorPublication),
    )
    .unwrap();
    let old_identity = current_generation_page_reference(711);
    let stale_identity = current_generation_page_reference(711);
    assert_eq!(
        CrashStableFreeReusePosture::admit(fence, old_identity, stale_identity, released)
            .unwrap_err(),
        PhysicalPublicationDenial::IdentityReuseWithoutGenerationAdvance
    );
    let other_owner = support::current_generation_extent_reference(712);
    assert_eq!(
        CrashStableFreeReusePosture::admit(fence, old_identity, other_owner, released).unwrap_err(),
        PhysicalPublicationDenial::IdentityReuseOwnerMismatch
    );
}

#[test]
fn readiness_denies_new_root_proof_mismatch_before_publish() {
    let inputs = publication_inputs();
    let mismatched_validation = root_publication_validation(2702, 99);
    let intent = PhysicalPublicationIntent::copy_on_write_root_manifest(
        inputs.old_candidate,
        inputs.new_candidate,
        inputs.old_reachability,
    )
    .validate_copy_on_write_inputs()
    .unwrap();
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &intent,
        NewRootPublicationProof::from_root_validation(mismatched_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );

    let denial = intent
        .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
        .unwrap()
        .join_readiness(readiness)
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalPublicationDenial::NewRootPublicationProofMismatch
    );
}

#[test]
fn root_candidate_denies_validation_for_unrelated_published_root() {
    let inputs = publication_inputs();
    let unrelated_validation = root_publication_validation(inputs.new_root.scope() + 99, 1);

    assert_eq!(
        PublicationRootCandidate::admit(inputs.new_root, unrelated_validation).unwrap_err(),
        PhysicalPublicationDenial::RootPublicationValidationRootMismatch
    );
}
