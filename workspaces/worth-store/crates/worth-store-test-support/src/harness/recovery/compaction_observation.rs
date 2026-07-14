use super::{closeout as closeout_fixture, source_precedence as source_precedence_fixture};

use worth_store_physical_certification::CompactionInterlockObservation;
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
    RootPublicationValidationWitness,
};
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    admit_physical_isolation_entry, admit_physical_read_stability_authority,
    admit_seed_stable_read_plan, CompactionCandidateRangeSet, CompactionCutoverDelta,
    CompactionCutoverStabilityProof, CompactionDeferredReclaimQueue,
    CompactionInterlockFoundationalEvidence, CompactionProtectedReferenceSet,
    CompactionReadInterlockPlan, CompactionSourceIntegrityEvidence,
    CurrentGenerationPhysicalReference, CurrentPhysicalRoot, GenerationCountedPhysicalReference,
    NewRootPublicationProof, OldReachabilityPreservation, PhysicalIsolationEntryRequest,
    PhysicalOrderingContract, PhysicalPublicationIntent, PhysicalPublicationReadiness,
    PhysicalReadPlanReleaseSemantics, PostProtectionPhysicalReadObservation,
    ProtectedPhysicalReferenceSet, PublicationLatchReadiness, PublicationRootCandidate,
    PublishedReaderHazard, ReadCopyUpdateRootPublication, ReadDuringCompactionVerdict,
    ReadPlanAdmissionScratchArena, RootSwapOrderingContract, StablePhysicalReadExecution,
    StablePhysicalReadPlan, TraversalAdmissionGuard, UnprotectedReadIntent,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

pub fn store_compaction_observation() -> CompactionInterlockObservation {
    CompactionInterlockObservation::from_store_interlock_evidence(compaction_interlock_evidence())
        .unwrap()
}

pub fn publication_only_compaction_observation() -> CompactionInterlockObservation {
    CompactionInterlockObservation::from_store_interlock_evidence(publication_only_evidence())
        .unwrap()
}

fn compaction_interlock_evidence() -> CompactionInterlockFoundationalEvidence {
    let (old_authority, new_authority) = ordered_compaction_authorities();
    let old_root = current_root_from_authority(&old_authority);
    let new_root = current_root_from_authority(&new_authority);
    let protected_reference = current_generation_page_reference(901);
    let read_plan = admit_local_plan(&old_authority, old_root, protected_reference);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&read_plan);
    let old_reachability = OldReachabilityPreservation::from_protected_footprint(
        read_plan.footprint().declared_footprint_basis(),
    )
    .unwrap();
    let pre_cutover_read = StablePhysicalReadExecution::from_execution_ready_handle(
        read_plan.into_execution_ready_handle(),
    )
    .complete();
    let evidence = source_precedence_fixture::intact_wal_integrity_evidence_for_owner(
        protected_reference.owner(),
    );
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&evidence).unwrap();
    let source =
        CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
            pre_cutover_read,
            clearance,
        )
        .unwrap();
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([protected_reference]).unwrap();
    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        old_root.epoch(),
        new_root.epoch(),
        source,
    )
    .unwrap();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            PublicationRootCandidate::admit(
                old_root,
                root_publication_validation(old_root.scope(), 1),
            )
            .unwrap(),
            PublicationRootCandidate::admit(
                new_root,
                root_publication_validation(new_root.scope(), 2),
            )
            .unwrap(),
            old_reachability,
        ),
        root_publication_validation(new_root.scope(), 2),
    );
    let publication =
        worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
            CompactionCutoverDelta::lower(plan, new_root).unwrap(),
            receipt,
        )
        .unwrap();
    let proof = CompactionCutoverStabilityProof::admit(
        publication.clone(),
        CompactionCutoverRecoveryPosture::admit_visible_product(
            source_precedence_fixture::compaction_visible_product_evidence(15),
        ),
    )
    .unwrap();
    let post_cutover_read = StablePhysicalReadExecution::from_execution_ready_handle(
        admit_local_plan(
            &new_authority,
            new_root,
            current_generation_page_reference(902),
        )
        .into_execution_ready_handle(),
    )
    .complete();
    let verdict = ReadDuringCompactionVerdict::from_stability_proof(
        proof,
        pre_cutover_read,
        post_cutover_read,
    )
    .unwrap();
    let drained = CompactionDeferredReclaimQueue::admit(publication)
        .unwrap()
        .drain_after_release(pre_cutover_read.read_plan_release())
        .unwrap();
    CompactionInterlockFoundationalEvidence::after_executed_interlock(&verdict, &drained)
}

fn publication_only_evidence() -> CompactionInterlockFoundationalEvidence {
    let (old_authority, new_authority) = ordered_compaction_authorities();
    let old_root = current_root_from_authority(&old_authority);
    let new_root = current_root_from_authority(&new_authority);
    let protected_reference = current_generation_page_reference(901);
    let read_plan = admit_local_plan(&old_authority, old_root, protected_reference);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&read_plan);
    let old_reachability = OldReachabilityPreservation::from_protected_footprint(
        read_plan.footprint().declared_footprint_basis(),
    )
    .unwrap();
    let evidence = source_precedence_fixture::intact_wal_integrity_evidence_for_owner(
        protected_reference.owner(),
    );
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&evidence).unwrap();
    let source =
        CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
            StablePhysicalReadExecution::from_execution_ready_handle(
                read_plan.into_execution_ready_handle(),
            )
            .complete(),
            clearance,
        )
        .unwrap();
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([protected_reference]).unwrap();
    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        old_root.epoch(),
        new_root.epoch(),
        source,
    )
    .unwrap();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            PublicationRootCandidate::admit(
                old_root,
                root_publication_validation(old_root.scope(), 1),
            )
            .unwrap(),
            PublicationRootCandidate::admit(
                new_root,
                root_publication_validation(new_root.scope(), 2),
            )
            .unwrap(),
            old_reachability,
        ),
        root_publication_validation(new_root.scope(), 2),
    );
    worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
        CompactionCutoverDelta::lower(plan, new_root).unwrap(),
        receipt,
    )
    .unwrap()
    .foundational_evidence()
}

fn publish_copy_on_write(
    intent: PhysicalPublicationIntent,
    new_validation: RootPublicationValidationWitness,
) -> worth_store_physical_isolation::PhysicalPublicationReceipt {
    let validated = intent.validate_copy_on_write_inputs().unwrap();
    let lowered = validated
        .clone()
        .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
        .unwrap();
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &validated,
        NewRootPublicationProof::from_root_validation(new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    ReadCopyUpdateRootPublication::publish(lowered.join_readiness(readiness).unwrap())
        .unwrap()
        .receipt()
        .clone()
}

fn root_publication_validation(root: u64, generation: u64) -> RootPublicationValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .unwrap()
}

fn admit_local_plan(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    reference: CurrentGenerationPhysicalReference,
) -> StablePhysicalReadPlan {
    let references = protected_set([reference], 4);
    let observed_references = references.clone();
    let intent = UnprotectedReadIntent::for_known_footprint(root, references, 8)
        .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
    let hazard = PublishedReaderHazard::publish(authority, intent).unwrap();
    let observed = PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
        authority,
        &hazard,
        root,
        observed_references,
    )
    .unwrap();
    let validated = hazard
        .observe_authority_after_publication(authority, observed)
        .unwrap()
        .validate()
        .unwrap();
    let receipt = TraversalAdmissionGuard::from_validated_root(validated)
        .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(4))
        .unwrap();
    admit_seed_stable_read_plan(receipt.into_cursor().finish()).unwrap()
}

fn protected_set<const N: usize>(
    references: [CurrentGenerationPhysicalReference; N],
    scratch_capacity: usize,
) -> ProtectedPhysicalReferenceSet {
    ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        references,
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(scratch_capacity),
    )
    .unwrap()
}

fn current_generation_page_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_page_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

fn generation_counted_page_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(17).unwrap();
    let page = PhysicalPageId::from_raw(23).unwrap();
    let slot = PhysicalRecordSlot::from_raw(1).unwrap();
    let cell = generations
        .slot_cell(segment, page, slot)
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_page_slot(cell))
}

fn current_root_from_authority(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_physical_isolation_entry(
        authority.root_epoch_basis().current_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .unwrap()
}

fn physical_authority_from_complete_closeout(
) -> worth_store_physical_isolation::PhysicalReadStabilityAuthority {
    physical_authority_from_completion(closeout_fixture::recovery_completion())
}

fn ordered_compaction_authorities() -> (
    worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    worth_store_physical_isolation::PhysicalReadStabilityAuthority,
) {
    let current = physical_authority_from_complete_closeout();
    let current_root = current_root_from_authority(&current);

    for ordinal in 0..256 {
        let successor = physical_authority_from_completion(
            closeout_fixture::recovery_completion_with_operation_digest(&format!(
                "s5-compaction-successor-{ordinal}"
            )),
        );
        let successor_root = current_root_from_authority(&successor);
        if successor_root.epoch().get() > current_root.epoch().get()
            && successor_root.manifest_epoch().get() > current_root.manifest_epoch().get()
        {
            return (current, successor);
        }
    }

    panic!("failed to derive a compaction successor with an advancing epoch vector")
}

fn physical_authority_from_completion(
    completion: worth_store_recovery_physics::RecoveryCompletion,
) -> worth_store_physical_isolation::PhysicalReadStabilityAuthority {
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();
    admit_physical_read_stability_authority(&entry).unwrap()
}
