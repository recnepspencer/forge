#[path = "../closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../../../s4_5_compaction_mutation_support/compaction_mutation_root_validation.rs"]
mod compaction_mutation_root_validation;
#[path = "../../../scenarios/physical_isolation/stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "../recovery_source_precedence/source_precedence_fixture.rs"]
mod source_precedence_fixture;

use compaction_mutation_root_validation::{
    generation_counted_page_reference, root_publication_validation,
};
use forge_store_physical_certification::{
    CoverageGapDenial, PhysicalInterleavingSchedule,
    PhysicalIsolationCompactionMutationReplayBinding,
    PhysicalIsolationCompactionMutationScheduledLaneOutput, PhysicalSimulationPlan,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
    RootPublicationValidationWitness,
};
use forge_store_physical_integrity::CompactionSourceIntegrityClearance;
use forge_store_physical_isolation::{
    admit_physical_isolation_entry, admit_physical_read_stability_authority,
    pre_wait_denial_for_hierarchy_inversion, CompactionCandidateRangeSet, CompactionCutoverDelta,
    CompactionDeferredReclaimQueue, CompactionMutationLaneOrigin, CompactionMutationLaneReceipt,
    CompactionProtectedReferenceSet, CompactionReadInterlockPlan, CompactionRewritePublication,
    CompactionSourceIntegrityEvidence, CurrentGenerationPhysicalReference, CurrentPhysicalRoot,
    GenerationCountedPhysicalReference, LatchAcquisitionStep, NewRootPublicationProof,
    OldReachabilityPreservation, PhysicalIsolationEntryRequest, PhysicalLatchKey,
    PhysicalOrderingContract, PhysicalPublicationIntent, PhysicalPublicationReadiness,
    PhysicalReadStabilityAuthority, PublicationLatchReadiness, PublicationRootCandidate,
    ReadCopyUpdateRootPublication, RootSwapOrderingContract, StablePhysicalReadExecution,
};
use forge_store_recovery_physics::CompactionCutoverRecoveryPosture;
use plan_admission::{admit_plan, protected_set};

struct PublicationInputs {
    old_root: CurrentPhysicalRoot,
    new_root: CurrentPhysicalRoot,
    old_candidate: PublicationRootCandidate,
    new_candidate: PublicationRootCandidate,
    old_reachability: OldReachabilityPreservation,
    new_validation: RootPublicationValidationWitness,
}

pub(crate) fn complete_compaction_mutation_receipts() -> Vec<CompactionMutationLaneReceipt> {
    vec![
        in_place_overwrite_receipt(),
        early_reclaim_receipt(),
        stale_epoch_receipt(),
        backend_residue_receipt(),
        latch_hierarchy_inversion_receipt(),
        mixed_root_receipt(),
    ]
}

pub(crate) fn complete_scheduled_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    scheduled_compaction_mutation_lanes(plan, schedule, complete_compaction_mutation_receipts())
}

pub(crate) fn compaction_mutation_origin() -> CompactionMutationLaneOrigin {
    CompactionMutationLaneOrigin::from_plan(&admitted_compaction_plan())
}

pub(crate) fn different_compaction_mutation_origin() -> CompactionMutationLaneOrigin {
    CompactionMutationLaneOrigin::from_plan(&admitted_compaction_plan_for(
        current_generation_page_reference(702),
    ))
}

pub(crate) fn same_footprint_wrong_cutover_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    let mut receipts = complete_compaction_mutation_receipts();
    receipts[0] = same_footprint_wrong_cutover_in_place_receipt();
    scheduled_compaction_mutation_lanes(plan, schedule, receipts)
}

pub(crate) fn detached_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    let binding =
        PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(plan, schedule)?;
    let detached_step_index = binding.compaction_actor_step_index().saturating_add(1);
    complete_compaction_mutation_receipts()
        .into_iter()
        .map(|receipt| {
            PhysicalIsolationCompactionMutationScheduledLaneOutput::from_schedule_step_receipt(
                &binding,
                schedule,
                detached_step_index,
                receipt,
            )
        })
        .collect()
}

fn scheduled_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    receipts: impl IntoIterator<Item = CompactionMutationLaneReceipt>,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    let binding =
        PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(plan, schedule)?;
    receipts
        .into_iter()
        .map(|receipt| {
            PhysicalIsolationCompactionMutationScheduledLaneOutput::from_schedule_step_receipt(
                &binding,
                schedule,
                binding.compaction_actor_step_index(),
                receipt,
            )
        })
        .collect()
}

fn in_place_overwrite_receipt() -> CompactionMutationLaneReceipt {
    let (receipt, _) =
        CompactionMutationLaneReceipt::from_in_place_overwrite_denial(admitted_compaction_plan());
    receipt
}

fn same_footprint_wrong_cutover_in_place_receipt() -> CompactionMutationLaneReceipt {
    let (receipt, _) = CompactionMutationLaneReceipt::from_in_place_overwrite_denial(
        admitted_compaction_plan_for_new_digest(
            current_generation_page_reference(701),
            "phase8-compaction-mutant-wrong-cutover",
        ),
    );
    receipt
}

fn early_reclaim_receipt() -> CompactionMutationLaneReceipt {
    let queue = CompactionDeferredReclaimQueue::admit(compaction_publication()).unwrap();
    let (receipt, _) = CompactionMutationLaneReceipt::from_early_reclaim_denial(&queue);
    receipt
}

fn stale_epoch_receipt() -> CompactionMutationLaneReceipt {
    let inputs = publication_inputs();
    let old_authority = physical_authority_from_complete_closeout();
    let reference = current_generation_page_reference(701);
    let expected_plan = admitted_compaction_plan_for(reference);
    let receipt = CompactionMutationLaneReceipt::from_stale_epoch_admission_denial(
        &expected_plan,
        inputs.new_root.epoch(),
        inputs.old_root.epoch(),
        stable_source_evidence(&old_authority, inputs.old_root, reference),
    )
    .unwrap();
    receipt
}

fn backend_residue_receipt() -> CompactionMutationLaneReceipt {
    CompactionMutationLaneReceipt::from_backend_residue_denial(
        compaction_publication(),
        CompactionCutoverRecoveryPosture::missing_generation_identity(
            source_precedence_fixture::trace("phase8-backend-residue-mutant", 11),
        ),
    )
    .unwrap()
}

fn latch_hierarchy_inversion_receipt() -> CompactionMutationLaneReceipt {
    let plan = admitted_compaction_plan();
    let inputs = publication_inputs();
    let mut inverted = [
        LatchAcquisitionStep::shared(PhysicalLatchKey::root(inputs.new_root.epoch())),
        LatchAcquisitionStep::shared(PhysicalLatchKey::root(inputs.old_root.epoch())),
    ];
    if pre_wait_denial_for_hierarchy_inversion(&inverted)
        .unwrap()
        .is_none()
    {
        inverted.reverse();
    }
    let evidence = pre_wait_denial_for_hierarchy_inversion(&inverted)
        .unwrap()
        .unwrap();
    CompactionMutationLaneReceipt::from_latch_hierarchy_inversion_denial(&plan, evidence).unwrap()
}

fn mixed_root_receipt() -> CompactionMutationLaneReceipt {
    CompactionMutationLaneReceipt::from_mixed_root_read_denial(&admitted_compaction_plan())
}

fn compaction_publication() -> CompactionRewritePublication {
    compaction_publication_for(current_generation_page_reference(701))
}

fn compaction_publication_for(
    reference: CurrentGenerationPhysicalReference,
) -> CompactionRewritePublication {
    let inputs = publication_inputs();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
    );
    forge_store_physical_isolation::publish_compaction_rewrite_for_certification(
        CompactionCutoverDelta::lower(admitted_compaction_plan_for(reference), inputs.new_root)
            .unwrap(),
        receipt,
    )
    .unwrap()
}

fn admitted_compaction_plan() -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for(current_generation_page_reference(701))
}

fn admitted_compaction_plan_for(
    reference: CurrentGenerationPhysicalReference,
) -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for_inputs(reference, publication_inputs())
}

fn admitted_compaction_plan_for_new_digest(
    reference: CurrentGenerationPhysicalReference,
    digest: &str,
) -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for_inputs(reference, publication_inputs_for_new_digest(digest))
}

fn admitted_compaction_plan_for_inputs(
    reference: CurrentGenerationPhysicalReference,
    inputs: PublicationInputs,
) -> CompactionReadInterlockPlan {
    let old_authority = physical_authority_from_complete_closeout();
    let old_plan = admit_plan(
        &old_authority,
        inputs.old_root,
        protected_set([reference], 4),
        8,
        4,
    );
    CompactionReadInterlockPlan::admit(
        CompactionProtectedReferenceSet::from_read_plan(&old_plan),
        CompactionCandidateRangeSet::from_current_generation_refs([reference]).unwrap(),
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        stable_source_evidence(&old_authority, inputs.old_root, reference),
    )
    .unwrap()
}

fn publication_inputs() -> PublicationInputs {
    publication_inputs_for_new_digest("phase8-compaction-mutant")
}

fn publication_inputs_for_new_digest(digest: &str) -> PublicationInputs {
    let old_authority = physical_authority_from_complete_closeout();
    let old_root = current_root_from_authority(&old_authority);
    let new_authority = advancing_authority_for_digest(old_root, digest);
    let new_root = current_root_from_authority(&new_authority);
    let old_validation = root_publication_validation(old_root.scope(), 1);
    let new_validation = root_publication_validation(new_root.scope(), 2);
    let old_candidate = PublicationRootCandidate::admit(old_root, old_validation).unwrap();
    let new_candidate = PublicationRootCandidate::admit(new_root, new_validation).unwrap();
    let reference = current_generation_page_reference(701);
    let old_plan = admit_plan(
        &old_authority,
        old_root,
        protected_set([reference], 4),
        8,
        4,
    );
    let old_reachability = OldReachabilityPreservation::from_protected_footprint(
        old_plan.footprint().declared_footprint_basis(),
    )
    .unwrap();
    PublicationInputs {
        old_root,
        new_root,
        old_candidate,
        new_candidate,
        old_reachability,
        new_validation,
    }
}

fn advancing_authority_for_digest(
    current_root: CurrentPhysicalRoot,
    digest: &str,
) -> PhysicalReadStabilityAuthority {
    for ordinal in 0..256 {
        let candidate = physical_authority_from_operation_digest_closeout_with_digest(&format!(
            "{digest}-{ordinal}"
        ));
        let candidate_root = current_root_from_authority(&candidate);
        if candidate_root.epoch().get() > current_root.epoch().get()
            && candidate_root.manifest_epoch().get() > current_root.manifest_epoch().get()
        {
            return candidate;
        }
    }

    panic!("failed to derive a compaction successor with an advancing epoch vector")
}

fn publish_copy_on_write(
    intent: PhysicalPublicationIntent,
    new_validation: RootPublicationValidationWitness,
) -> forge_store_physical_isolation::PhysicalPublicationReceipt {
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

fn stable_source_evidence(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    reference: CurrentGenerationPhysicalReference,
) -> CompactionSourceIntegrityEvidence {
    let evidence =
        source_precedence_fixture::intact_wal_integrity_evidence_for_owner(reference.owner());
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&evidence).unwrap();
    CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
        execute_read(authority, root, reference),
        clearance,
    )
    .unwrap()
}

fn execute_read(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    reference: CurrentGenerationPhysicalReference,
) -> forge_store_physical_isolation::StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        admit_plan(authority, root, protected_set([reference], 4), 8, 4)
            .into_execution_ready_handle(),
    )
    .complete()
}

fn physical_authority_from_complete_closeout() -> PhysicalReadStabilityAuthority {
    physical_authority_from_completion(closeout_fixture::recovery_completion())
}

fn physical_authority_from_operation_digest_closeout_with_digest(
    digest: &str,
) -> PhysicalReadStabilityAuthority {
    physical_authority_from_completion(closeout_fixture::recovery_completion_with_operation_digest(
        digest,
    ))
}

fn physical_authority_from_completion(
    completion: forge_store_recovery_physics::RecoveryCompletion,
) -> PhysicalReadStabilityAuthority {
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();
    admit_physical_read_stability_authority(&entry).unwrap()
}

fn current_root_from_authority(authority: &PhysicalReadStabilityAuthority) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_physical_isolation_entry(
        authority.root_epoch_basis().current_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .unwrap()
}

fn current_generation_page_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_page_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}
