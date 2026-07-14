use forge_proof::TransitionOutcome;
use forge_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionCutoverStabilityProof,
    CompactionDeferredReclaimQueue, CompactionProtectedReferenceSet, CompactionReadInterlockDenial,
    CompactionReadInterlockPlan, PhysicalPublicationIntent, ReadDuringCompactionVerdict,
    StablePhysicalReadExecution,
};
use forge_store_recovery_physics::CompactionCutoverRecoveryPosture;

use super::plan_admission::{admit_plan, protected_set};
use super::publication_support::{publication_inputs, publish_copy_on_write};
use super::shared_production_setup::stable_source_evidence;
use super::source_precedence_fixture;
use super::support::{
    current_generation_page_reference, physical_authority_from_complete_closeout,
};

#[test]
fn read_during_compaction_keeps_old_reader_and_new_reader_stable() {
    let inputs = publication_inputs();
    let old_authority = physical_authority_from_complete_closeout();
    let protected_reference = current_generation_page_reference(701);
    let old_plan = admit_plan(
        &old_authority,
        inputs.old_root,
        protected_set([protected_reference], 4),
        8,
        4,
    );
    let source_evidence =
        stable_source_evidence(&old_authority, inputs.old_root, protected_reference);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&old_plan);
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([protected_reference]).unwrap();
    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        source_evidence,
    )
    .unwrap();
    assert!(plan.reclaim_deferred());
    assert_eq!(plan.counters().protected_ranges(), 1);
    assert_eq!(plan.counters().candidate_ranges(), 1);
    assert_eq!(plan.counters().range_comparisons(), 1);
    assert_eq!(plan.counters().overlapping_ranges(), 1);
    assert_eq!(plan.counters().copied_pages(), 1);

    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );
    let publication =
        forge_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
            CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
            receipt,
        )
        .unwrap();
    assert_eq!(publication.counters().publication_swaps(), 1);

    let recovery_posture = CompactionCutoverRecoveryPosture::admit_visible_product(
        source_precedence_fixture::compaction_visible_product_evidence(9),
    );
    let proof =
        CompactionCutoverStabilityProof::admit(publication.clone(), recovery_posture).unwrap();
    let pre_read = StablePhysicalReadExecution::from_execution_ready_handle(
        old_plan.into_execution_ready_handle(),
    )
    .complete();
    let post_plan = admit_plan(
        &inputs.new_authority,
        inputs.new_root,
        protected_set([current_generation_page_reference(702)], 4),
        8,
        4,
    );
    let post_read = match StablePhysicalReadExecution::from_execution_ready_handle(
        post_plan.into_execution_ready_handle(),
    )
    .complete_with_proof()
    {
        TransitionOutcome::Success(receipt) => receipt,
        other => panic!("post-cutover read should execute: {other:?}"),
    };
    let verdict =
        ReadDuringCompactionVerdict::from_stability_proof(proof, pre_read, post_read).unwrap();
    assert!(verdict.pre_cutover_reader_retained_old_structure());
    assert!(verdict.post_cutover_reader_observed_new_epoch());

    let queue = CompactionDeferredReclaimQueue::admit(publication).unwrap();
    assert_eq!(queue.counters().blocked_reclaims(), 1);
    let (denial, counters) = queue.reject_early_reclaim();
    assert!(matches!(
        denial,
        CompactionReadInterlockDenial::EarlyReclaimBeforeReadRelease { .. }
    ));
    assert_eq!(counters.denied_early_reclaims(), 1);
    let drained = queue.drain_after_release(inputs.old_release).unwrap();
    assert_eq!(
        drained.released().footprint_basis().protected_references(),
        1
    );
}
