use crate::harness::physical_isolation::{epoch_scope, publication, read_plan};
use crate::harness::recovery::{deterministic_selected_compaction_product, wal_tail};
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionDeferredReclaimQueue,
    CompactionInterlockFoundationalEvidence, CompactionProtectedReferenceSet,
    CompactionReadInterlockPlan, CompactionRecoveryEvidence, CompactionRewritePublication,
    CompactionSourceIntegrityEvidence, StablePhysicalReadExecution,
};

pub fn admitted_compaction_plan() -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for_seed(17)
}

pub fn admitted_compaction_plan_for_seed(root_seed: u64) -> CompactionReadInterlockPlan {
    let inputs = publication::publication_inputs_with_root_generation(root_seed.max(1));
    let old_root = inputs.old_root;
    let new_root = inputs.new_root;
    let reference = epoch_scope::current_generation_page_reference(root_seed.max(1));
    let protected_references = read_plan::protected_set([reference], 1);
    let stable_plan = read_plan::admit_plan(
        &inputs.old_authority,
        old_root,
        protected_references,
        4_096,
        1,
    );
    let protected = CompactionProtectedReferenceSet::from_read_plan(&stable_plan);
    let stable_read = StablePhysicalReadExecution::from_execution_ready_handle(
        stable_plan.into_execution_ready_handle(),
    )
    .complete();
    let integrity = wal_tail::intact_wal_integrity_evidence_for_owner(reference.owner());
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&integrity)
        .expect("ordinary integrity evidence clears the compaction source");
    let source =
        CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
            stable_read,
            clearance,
        )
        .expect("executed stable read and integrity locality agree");
    let candidates = CompactionCandidateRangeSet::from_current_generation_refs([reference])
        .expect("fixture candidate is current-generation physical authority");

    CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        old_root.epoch(),
        new_root.epoch(),
        source,
    )
    .expect("ordinary compaction plan admission")
}

pub fn compaction_interlock_foundational_evidence_for_seed(
    root_seed: u64,
) -> CompactionInterlockFoundationalEvidence {
    let seed = root_seed.max(1);
    let plan = admitted_compaction_plan_for_seed(seed);
    let publication_inputs = publication::publication_inputs_with_root_generation(seed);
    let reference = epoch_scope::current_generation_page_reference(seed);
    let pre_cutover_read = StablePhysicalReadExecution::from_execution_ready_handle(
        read_plan::admit_plan(
            &publication_inputs.old_authority,
            publication_inputs.old_root,
            read_plan::protected_set([reference], 1),
            4_096,
            1,
        )
        .into_execution_ready_handle(),
    )
    .complete();
    let delta = CompactionCutoverDelta::lower_to_manifest(
        plan,
        publication_inputs.new_root.manifest_epoch().get(),
    )
    .expect("admitted compaction plan lowers to the publication manifest");
    let publication = publication::admitted_copy_on_write_plan(&publication_inputs).complete();
    let rewrite = CompactionRewritePublication::publish_rewrite(delta, publication)
        .expect("lowered compaction rewrite binds to the executed publication");
    let recovery =
        CompactionRecoveryEvidence::selected_product(deterministic_selected_compaction_product());
    let proof = worth_store_physical_isolation::CompactionCutoverStabilityProof::admit(
        rewrite.clone(),
        recovery,
    )
    .expect("published compaction admits recovery visibility");
    let post_cutover_read = StablePhysicalReadExecution::from_execution_ready_handle(
        proof
            .plan_post_cutover_read()
            .expect("published compaction admits a successor read plan")
            .into_execution_ready_handle(),
    )
    .complete();
    let verdict = worth_store_physical_isolation::execute_read_during_compaction_cutover(
        rewrite.clone(),
        recovery,
        pre_cutover_read,
        post_cutover_read,
    )
    .expect("executed compaction reads observe both root epochs");
    let reclaim = CompactionDeferredReclaimQueue::admit(rewrite)
        .expect("published compaction defers reclaim until read release")
        .drain_after_release(pre_cutover_read.read_plan_release())
        .expect("executed pre-cutover read releases the protected footprint");
    CompactionInterlockFoundationalEvidence::after_executed_interlock(&verdict, &reclaim)
}
