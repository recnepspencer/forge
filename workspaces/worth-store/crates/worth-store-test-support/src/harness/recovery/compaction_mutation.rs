use super::source_precedence as source_precedence_fixture;
use crate::harness::physical_isolation::{
    epoch_scope::generation_counted_page_reference,
    publication::{self, PublicationInputs},
    read_plan::{admit_plan, protected_set},
};
use worth_store_physical_certification::{
    CoverageGapDenial, PhysicalInterleavingSchedule,
    PhysicalIsolationCompactionMutationReplayBinding,
    PhysicalIsolationCompactionMutationScheduledLaneOutput, PhysicalSimulationPlan,
};
use worth_store_physical_format::{PhysicalGeneration, PhysicalStoreIdentity};
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    pre_wait_denial_for_hierarchy_inversion, CompactionCandidateRangeSet, CompactionCutoverDelta,
    CompactionDeferredReclaimQueue, CompactionMutationLaneOrigin, CompactionMutationLaneReceipt,
    CompactionProtectedReferenceSet, CompactionReadInterlockPlan, CompactionRewritePublication,
    CompactionSourceIntegrityEvidence, CurrentGenerationPhysicalReference, CurrentPhysicalRoot,
    LatchAcquisitionStep, PhysicalLatchKey, PhysicalPublicationIntent,
    PhysicalReadStabilityAuthority, StablePhysicalReadExecution,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

pub fn complete_compaction_mutation_receipts() -> Vec<CompactionMutationLaneReceipt> {
    vec![
        in_place_overwrite_receipt(),
        early_reclaim_receipt(),
        stale_epoch_receipt(),
        backend_residue_receipt(),
        latch_hierarchy_inversion_receipt(),
        mixed_root_receipt(),
    ]
}

pub fn complete_scheduled_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    scheduled_compaction_mutation_lanes(plan, schedule, complete_compaction_mutation_receipts())
}

pub fn compaction_mutation_origin() -> CompactionMutationLaneOrigin {
    CompactionMutationLaneOrigin::from_plan(&admitted_compaction_plan())
}

pub fn different_compaction_mutation_origin() -> CompactionMutationLaneOrigin {
    CompactionMutationLaneOrigin::from_plan(&admitted_compaction_plan_for(
        current_generation_page_reference(702),
    ))
}

pub fn same_footprint_wrong_cutover_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>, CoverageGapDenial> {
    let mut receipts = complete_compaction_mutation_receipts();
    receipts[0] = same_footprint_wrong_cutover_in_place_receipt();
    scheduled_compaction_mutation_lanes(plan, schedule, receipts)
}

pub fn detached_compaction_mutation_lanes(
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
        admitted_compaction_plan_for_published_successor(current_generation_page_reference(701)),
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
    let reference = current_generation_page_reference(701);
    let expected_plan = admitted_compaction_plan_for(reference);
    CompactionMutationLaneReceipt::from_stale_epoch_admission_denial(
        &expected_plan,
        inputs.new_root.epoch(),
        inputs.old_root.epoch(),
        stable_source_evidence(&inputs.old_authority, inputs.old_root, reference),
    )
    .unwrap()
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
    let receipt = publication::publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );
    worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
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

fn admitted_compaction_plan_for_published_successor(
    reference: CurrentGenerationPhysicalReference,
) -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for_inputs(reference, successor_publication_inputs())
}

fn admitted_compaction_plan_for_inputs(
    reference: CurrentGenerationPhysicalReference,
    inputs: PublicationInputs,
) -> CompactionReadInterlockPlan {
    let old_plan = admit_plan(
        &inputs.old_authority,
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
        stable_source_evidence(&inputs.old_authority, inputs.old_root, reference),
    )
    .unwrap()
}

fn publication_inputs() -> PublicationInputs {
    publication::publication_inputs_with_root_generation(701)
}

fn successor_publication_inputs() -> PublicationInputs {
    let prior = publication_inputs();
    let receipt = publication::publish_inputs(&prior);
    publication::successor_publication_inputs_for_store(
        &receipt,
        &PhysicalStoreIdentity::physical_format_default(),
        702,
    )
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
) -> worth_store_physical_isolation::StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        admit_plan(authority, root, protected_set([reference], 4), 8, 4)
            .into_execution_ready_handle(),
    )
    .complete()
}

fn current_generation_page_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_page_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}
