#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s4_recovery_source_precedence/source_precedence_fixture.rs"]
#[allow(dead_code)]
mod source_precedence_fixture;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
#[allow(dead_code)]
mod support;

use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionProtectedReferenceSet, CompactionReadInterlockDenial,
    CompactionReadInterlockPlan, CompactionSourceIntegrityEvidence, CurrentPhysicalRoot,
    StablePhysicalReadExecution,
};
use plan_admission::{admit_plan, protected_set};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};

#[test]
fn compaction_range_interlock_counts_bounded_multi_range_intersections() {
    let authority = physical_authority_from_complete_closeout();
    let target_authority = physical_authority_from_operation_digest_closeout("phase8-range-bounds");
    let root = current_root_from_authority(&authority);
    let target_root = current_root_from_authority(&target_authority);
    let first = current_generation_page_reference(801);
    let second = current_generation_page_reference(802);
    let plan = admit_plan(&authority, root, protected_set([first, second], 4), 8, 4);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&plan);
    let candidates =
        CompactionCandidateRangeSet::from_current_generation_refs([first, second]).unwrap();

    let interlock = CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        root.epoch(),
        target_root.epoch(),
        stable_source_evidence(&authority, root, first),
    )
    .expect_err("one-owner source evidence cannot cover a multi-owner candidate footprint");
    assert_eq!(
        interlock,
        CompactionReadInterlockDenial::SourceEvidenceMismatch
    );

    let protected = CompactionProtectedReferenceSet::from_read_plan(&plan);
    let candidates = CompactionCandidateRangeSet::from_current_generation_refs([first]).unwrap();
    let counters = candidates.intersect_protected(&protected);
    assert_eq!(counters.protected_ranges(), 2);
    assert_eq!(counters.candidate_ranges(), 1);
    assert_eq!(counters.range_comparisons(), 2);
    assert_eq!(counters.overlapping_ranges(), 1);
}

fn stable_source_evidence(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
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
