use crate::checkpoint_read_fixture as checkpoint_read;

use super::plan_admission::{admit_plan, protected_set};
use super::publication_support::{publication_inputs_with_new_root_digest, publish_copy_on_write};
use super::reclaim_support::ReclaimFixture;
use super::source_precedence_fixture;
use super::support::{
    current_generation_page_reference, physical_authority_from_complete_closeout,
};
use worth_proof::TransitionOutcome;
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    compare_physical_epoch_vectors_with_evidence, lower_latch_acquisition_plan,
    BackupReachabilityLeaseIndexSnapshot, CompactionCandidateRangeSet, CompactionCutoverDelta,
    CompactionCutoverStabilityProof, CompactionProtectedReferenceSet, CompactionReadInterlockPlan,
    CompactionSourceIntegrityEvidence, EpochComparisonScope, ExecutedIsolationEvidence,
    ExecutedIsolationReceipts, HazardLeaseTable, HazardLeaseTableCapacity, LatchAcquisitionRequest,
    LatchAcquisitionStep, PhysicalEpochVector, PhysicalIsolationCounterSnapshot, PhysicalLatchKey,
    PhysicalPublicationIntent, ReadDuringCompactionVerdict, StablePhysicalReadExecution,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

pub(crate) fn honest_executed_physical_isolation_closeout() -> ExecutedIsolationEvidence {
    let (stable_read, publication, compaction) = admitted_compaction_surfaces();
    let checkpoint = checkpoint_read::admitted_checkpoint_verdict();
    let reclaim_world = ReclaimFixture::new(981);
    let reclaim = worth_store_physical_isolation::ReclaimEligibilityProof::admit(
        reclaim_world.executed_reachability(),
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap())
            .live_index_snapshot(),
        BackupReachabilityLeaseIndexSnapshot::empty(),
    )
    .unwrap();
    let latch_plan =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(vec![
            LatchAcquisitionStep::shared(PhysicalLatchKey::root(
                stable_read.read_plan_release().root_epoch(),
            )),
        ]))
        .unwrap();
    let release = stable_read.read_plan_release();
    let epoch_vector = PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(
        release.root().scope(),
    ))
    .with_root(release.root_epoch())
    .with_manifest(release.root().manifest_epoch())
    .seal()
    .unwrap();
    let epoch_freshness =
        compare_physical_epoch_vectors_with_evidence(epoch_vector, epoch_vector).unwrap();
    ExecutedIsolationEvidence::from_physical_isolation_receipts(ExecutedIsolationReceipts {
        stable_read,
        latch_order_proof: latch_plan.order_proof(),
        epoch_freshness: &epoch_freshness,
        publication: &publication,
        reclaim: &reclaim,
        compaction: &compaction,
        checkpoint: &checkpoint,
    })
    .unwrap()
}

pub(crate) fn assert_expected_io_qos_closeout_counters(counters: PhysicalIsolationCounterSnapshot) {
    assert_eq!(counters.outcome_count(), 1);
    assert_eq!(counters.wait_count(), 0);
    assert_eq!(counters.retry_count(), 0);
    assert_eq!(counters.latch_counter_rows(), 1);
    assert_eq!(counters.latch_wait_count(), 1);
    assert_eq!(counters.reclaim_counter_rows(), 1);
    assert_eq!(counters.blocked_maintenance_count(), 1);
    assert_eq!(counters.reclaim_block_count(), 0);
    assert_eq!(counters.protected_byte_footprint(), 1);
}

fn admitted_compaction_surfaces() -> (
    worth_store_physical_isolation::StablePhysicalReadReceipt,
    worth_store_physical_isolation::PhysicalPublicationReceipt,
    ReadDuringCompactionVerdict,
) {
    let inputs = publication_inputs_with_new_root_digest("s5-phase15-new-root", 701);
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
        worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
            CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
            receipt,
        )
        .unwrap();
    let publication_receipt = publication.publication().clone();
    let recovery_posture = CompactionCutoverRecoveryPosture::admit_visible_product(
        source_precedence_fixture::compaction_visible_product_evidence(9),
    );
    let proof = CompactionCutoverStabilityProof::admit(publication, recovery_posture).unwrap();
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
    (pre_read, publication_receipt, verdict)
}

fn stable_source_evidence(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: worth_store_physical_isolation::CurrentPhysicalRoot,
    reference: worth_store_physical_isolation::CurrentGenerationPhysicalReference,
) -> CompactionSourceIntegrityEvidence {
    let evidence =
        source_precedence_fixture::intact_wal_integrity_evidence_for_owner(reference.owner());
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&evidence).unwrap();
    CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
        StablePhysicalReadExecution::from_execution_ready_handle(
            admit_plan(authority, root, protected_set([reference], 4), 8, 4)
                .into_execution_ready_handle(),
        )
        .complete(),
        clearance,
    )
    .unwrap()
}
