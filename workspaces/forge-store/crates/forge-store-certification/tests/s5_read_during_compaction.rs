#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s5_copy_on_write_publication/support.rs"]
#[allow(dead_code)]
mod publication_support;
#[path = "s4_recovery_source_precedence/source_precedence_fixture.rs"]
#[allow(dead_code)]
mod source_precedence_fixture;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
#[allow(dead_code)]
mod support;

use forge_proof::TransitionOutcome;
use forge_store_physical_integrity::CompactionSourceIntegrityClearance;
use forge_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionCutoverStabilityProof,
    CompactionDeferredReclaimQueue, CompactionProtectedReferenceSet, CompactionReadInterlockDenial,
    CompactionReadInterlockPlan, CompactionRewritePublication, CompactionSourceIntegrityEvidence,
    PhysicalPublicationIntent, ReadDuringCompactionVerdict, StablePhysicalReadExecution,
};
use forge_store_recovery_physics::CompactionCutoverRecoveryPosture;
use plan_admission::{admit_plan, protected_set};
use publication_support::{publication_inputs, publish_copy_on_write};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
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
    let publication = CompactionRewritePublication::publish(
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
    let new_authority = physical_authority_from_operation_digest_closeout("s5-phase7-new-root");
    let post_plan = admit_plan(
        &new_authority,
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

#[test]
fn compaction_denies_quarantine_stale_epoch_in_place_and_backend_residue() {
    let inputs = publication_inputs();
    let old_authority = physical_authority_from_complete_closeout();
    let reference = current_generation_page_reference(701);
    let old_plan = admit_plan(
        &old_authority,
        inputs.old_root,
        protected_set([reference], 4),
        8,
        4,
    );
    let stable_evidence = stable_source_evidence(&old_authority, inputs.old_root, reference);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&old_plan);
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([reference]).unwrap();
    let quarantine = CompactionReadInterlockPlan::admit(
        protected.clone(),
        candidates.clone(),
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        CompactionSourceIntegrityEvidence::from_quarantine_clearance(
            CompactionSourceIntegrityClearance::from_quarantine_record(
                &source_precedence_fixture::wal_tail_quarantine_record(),
            ),
        ),
    );
    assert!(matches!(
        quarantine,
        Err(CompactionReadInterlockDenial::QuarantinedCandidateRange)
    ));

    let stale_epoch = CompactionReadInterlockPlan::admit(
        protected.clone(),
        candidates.clone(),
        inputs.new_root.epoch(),
        inputs.old_root.epoch(),
        stable_evidence,
    );
    assert!(matches!(
        stale_epoch,
        Err(CompactionReadInterlockDenial::StaleCompactionSourceEpoch { .. })
    ));

    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        stable_evidence,
    )
    .unwrap();
    let (overwrite, counters) = plan.clone().deny_in_place_overwrite();
    assert_eq!(
        overwrite,
        CompactionReadInterlockDenial::InPlaceOverwriteOfProtectedStructure
    );
    assert_eq!(counters.denied_in_place_overwrites(), 1);

    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );
    let publication = CompactionRewritePublication::publish(
        CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
        receipt,
    )
    .unwrap();
    let residue = CompactionCutoverRecoveryPosture::missing_generation_identity(
        source_precedence_fixture::trace("backend-residue", 11),
    );
    assert!(matches!(
        CompactionCutoverStabilityProof::admit(publication, residue),
        Err(CompactionReadInterlockDenial::BackendResidueCandidateSelection(_))
    ));

    assert_eq!(
        current_root_from_authority(&old_authority).epoch().get(),
        inputs.old_root.epoch().get()
    );
}

#[test]
fn compaction_rejects_minted_source_and_publication_mismatches() {
    let inputs = publication_support::publication_inputs_with_new_root_digest(
        "s5-phase8-mismatched-reachability",
        702,
    );
    let old_authority = physical_authority_from_complete_closeout();
    let protected_reference = current_generation_page_reference(701);
    let mismatched_reference = current_generation_page_reference(702);
    let old_plan = admit_plan(
        &old_authority,
        inputs.old_root,
        protected_set([protected_reference], 4),
        8,
        4,
    );
    let protected = CompactionProtectedReferenceSet::from_read_plan(&old_plan);
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([protected_reference]).unwrap();
    let mismatched_evidence =
        stable_source_evidence(&old_authority, inputs.old_root, mismatched_reference);

    assert!(matches!(
        CompactionReadInterlockPlan::admit(
            protected.clone(),
            candidates.clone(),
            inputs.old_root.epoch(),
            inputs.new_root.epoch(),
            mismatched_evidence,
        ),
        Err(CompactionReadInterlockDenial::SourceEvidenceMismatch)
    ));

    let unrelated_clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(
        &source_precedence_fixture::intact_wal_integrity_evidence(),
    )
    .unwrap();
    let unrelated_source =
        CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
            execute_read(&old_authority, inputs.old_root, protected_reference),
            unrelated_clearance,
        )
        .unwrap();
    assert!(matches!(
        CompactionReadInterlockPlan::admit(
            protected.clone(),
            candidates.clone(),
            inputs.old_root.epoch(),
            inputs.new_root.epoch(),
            unrelated_source,
        ),
        Err(CompactionReadInterlockDenial::SourceEvidenceMismatch)
    ));

    let mixed_candidate_footprint = CompactionCandidateRangeSet::from_current_generation_refs([
        protected_reference,
        mismatched_reference,
    ])
    .unwrap();
    assert!(matches!(
        CompactionReadInterlockPlan::admit(
            protected.clone(),
            mixed_candidate_footprint,
            inputs.old_root.epoch(),
            inputs.new_root.epoch(),
            stable_source_evidence(&old_authority, inputs.old_root, protected_reference),
        ),
        Err(CompactionReadInterlockDenial::SourceEvidenceMismatch)
    ));

    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        stable_source_evidence(&old_authority, inputs.old_root, protected_reference),
    )
    .unwrap();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );

    assert!(matches!(
        CompactionRewritePublication::publish(
            CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
            receipt,
        ),
        Err(CompactionReadInterlockDenial::PublicationReachabilityFootprintMismatch { .. })
    ));
}

#[test]
fn read_during_compaction_verdict_rejects_unbound_read_receipts() {
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
    let protected = CompactionProtectedReferenceSet::from_read_plan(&old_plan);
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([protected_reference]).unwrap();
    let plan = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        inputs.old_root.epoch(),
        inputs.new_root.epoch(),
        stable_source_evidence(&old_authority, inputs.old_root, protected_reference),
    )
    .unwrap();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );
    let publication = CompactionRewritePublication::publish(
        CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
        receipt,
    )
    .unwrap();
    let proof = CompactionCutoverStabilityProof::admit(
        publication,
        CompactionCutoverRecoveryPosture::admit_visible_product(
            source_precedence_fixture::compaction_visible_product_evidence(12),
        ),
    )
    .unwrap();
    let wrong_pre = execute_read(
        &old_authority,
        inputs.old_root,
        current_generation_page_reference(702),
    );
    let new_authority = physical_authority_from_operation_digest_closeout("s5-phase7-new-root");
    let post_read = execute_read(
        &new_authority,
        inputs.new_root,
        current_generation_page_reference(702),
    );

    assert!(matches!(
        ReadDuringCompactionVerdict::from_stability_proof(proof, wrong_pre, post_read),
        Err(CompactionReadInterlockDenial::PreCutoverReadReceiptMismatch)
    ));
}

fn stable_source_evidence(
    authority: &forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: forge_store_physical_isolation::CurrentPhysicalRoot,
    reference: forge_store_physical_isolation::CurrentGenerationPhysicalReference,
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
    authority: &forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: forge_store_physical_isolation::CurrentPhysicalRoot,
    reference: forge_store_physical_isolation::CurrentGenerationPhysicalReference,
) -> forge_store_physical_isolation::StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        admit_plan(authority, root, protected_set([reference], 4), 8, 4)
            .into_execution_ready_handle(),
    )
    .complete()
}
