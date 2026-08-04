use crate::harness::physical_isolation::{epoch_scope, publication, read_plan};
use crate::harness::recovery::source_precedence;
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionProtectedReferenceSet, CompactionReadInterlockPlan,
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
    let integrity = source_precedence::intact_wal_integrity_evidence_for_owner(reference.owner());
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
