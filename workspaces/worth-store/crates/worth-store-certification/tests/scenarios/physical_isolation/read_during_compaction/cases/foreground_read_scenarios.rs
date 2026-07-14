use worth_store_physical_isolation::{
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionCutoverStabilityProof,
    CompactionProtectedReferenceSet, CompactionReadInterlockDenial, CompactionReadInterlockPlan,
    PhysicalPublicationIntent, ReadDuringCompactionVerdict,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

use super::plan_admission::{admit_plan, protected_set};
use super::publication_support::{publication_inputs, publish_copy_on_write};
use super::shared_production_setup::{execute_read, stable_source_evidence};
use super::source_precedence_fixture;
use super::support::{
    current_generation_page_reference, physical_authority_from_complete_closeout,
};

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
    let publication =
        worth_store_physical_isolation::CompactionRewritePublication::publish_rewrite(
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
    let post_read = execute_read(
        &inputs.new_authority,
        inputs.new_root,
        current_generation_page_reference(702),
    );

    assert!(matches!(
        ReadDuringCompactionVerdict::from_stability_proof(proof, wrong_pre, post_read),
        Err(CompactionReadInterlockDenial::PreCutoverReadReceiptMismatch)
    ));
}
