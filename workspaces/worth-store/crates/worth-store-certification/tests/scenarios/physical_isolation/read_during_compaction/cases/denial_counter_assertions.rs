use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionCutoverStabilityProof,
    CompactionProtectedReferenceSet, CompactionReadInterlockDenial, CompactionReadInterlockPlan,
    CompactionSourceIntegrityEvidence, PhysicalPublicationIntent,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

use super::plan_admission::{admit_plan, protected_set};
use super::publication_support::{publication_inputs, publish_copy_on_write};
use super::shared_production_setup::{execute_read, stable_source_evidence};
use super::source_precedence_fixture;
use super::support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

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
    let publication =
        worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
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
    let inputs = super::publication_support::publication_inputs_with_new_root_digest(
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
        worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
            CompactionCutoverDelta::lower(plan, inputs.new_root).unwrap(),
            receipt,
        ),
        Err(CompactionReadInterlockDenial::PublicationReachabilityFootprintMismatch { .. })
    ));
}
