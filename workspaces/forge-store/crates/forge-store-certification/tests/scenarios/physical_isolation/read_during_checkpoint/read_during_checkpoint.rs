use forge_store_test_support::harness::physical_isolation::epoch_scope as support;
use forge_store_test_support::harness::physical_isolation::read_plan as plan_admission;
use forge_store_test_support::harness::recovery::checkpoint_basis as checkpoint_basis_fixture;
use forge_store_test_support::harness::recovery::checkpoint_durability as checkpoint_durability_fixture;

use forge_store_physical_backend::SimulatedStrictDurableProfile;
use forge_store_physical_isolation::{
    reject_copied_checkpoint_report_as_checkpoint_interlock,
    reject_same_run_self_comparison_as_checkpoint_interlock, CheckpointPublicationIdentity,
    CheckpointPublicationReadmission, CheckpointPublicationRoot,
    CheckpointPublicationStabilityProof, CheckpointReadInterlockDenial,
    CheckpointReadInterlockPlan, CheckpointRootEpochTransition, PhysicalOrderingContract,
    ReadDuringCheckpointVerdict, StablePhysicalReadExecution,
};
use forge_store_recovery_physics::{
    CheckpointCutoverReceipt, CheckpointManifest, CheckpointPublicationPlan,
    CheckpointRecoveryCounterSnapshot, CheckpointRootPosture, CheckpointValidation,
    SharpCheckpointCertificationMode,
};
use plan_admission::{admit_plan, protected_set};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};

#[test]
fn read_during_checkpoint_keeps_old_reader_and_new_reader_stable() {
    let verdict = admitted_checkpoint_verdict();

    assert!(verdict.old_reader_retained_old_root());
    assert!(verdict.post_publication_reader_observed_new_epoch());
    assert_ne!(
        verdict.proof().pre_publication_root().epoch(),
        verdict.proof().post_publication_root().epoch()
    );
}

pub(crate) fn admitted_checkpoint_verdict() -> ReadDuringCheckpointVerdict {
    let inputs = checkpoint_inputs();
    let old_reference = current_generation_page_reference(901);
    let new_reference = current_generation_page_reference(902);
    let pre_read = execute_read(&inputs.old_authority, inputs.old_root, old_reference);
    let post_read = execute_read(&inputs.new_authority, inputs.new_root, new_reference);

    let readmission = CheckpointPublicationReadmission::admit(
        inputs.checkpoint_root,
        inputs.new_root,
        &inputs.validation,
        inputs.cutover_receipt,
    )
    .unwrap();
    let transition = CheckpointRootEpochTransition::admit(inputs.old_root, readmission).unwrap();
    let plan = CheckpointReadInterlockPlan::admit(pre_read, transition).unwrap();
    let proof =
        CheckpointPublicationStabilityProof::from_plan_and_post_publication_read(plan, post_read)
            .unwrap();
    ReadDuringCheckpointVerdict::from_stability_proof(proof).unwrap()
}

#[test]
fn checkpoint_publication_root_requires_readmission_as_current_root() {
    let inputs = checkpoint_inputs();
    let stale_checkpoint_root = CheckpointPublicationRoot::from_checkpoint_publication(
        inputs
            .old_authority
            .root_epoch_basis()
            .checkpoint_publication_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_checkpoint_id(inputs.validation.checkpoint_id()),
    )
    .unwrap();

    let denial = CheckpointPublicationReadmission::admit(
        stale_checkpoint_root,
        inputs.new_root,
        &inputs.validation,
        inputs.cutover_receipt,
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        CheckpointReadInterlockDenial::CheckpointPublicationRootNotReadmitted { .. }
    ));
}

#[test]
fn checkpoint_readmission_rejects_same_range_cutover_from_different_checkpoint() {
    let inputs = checkpoint_inputs();
    let other_manifest_with_same_range = checkpoint_basis_fixture::manifest(10, 20, 13);
    let other_validation = checkpoint_durability_fixture::validate(other_manifest_with_same_range);
    let other_durability = checkpoint_durability_fixture::checkpoint_durability(&other_validation);
    let other_publication =
        CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
            other_validation,
            other_durability,
        )
        .unwrap();
    let same_range_other_checkpoint_receipt = CheckpointCutoverReceipt::publish(other_publication);

    let denial = CheckpointPublicationReadmission::admit(
        inputs.checkpoint_root,
        inputs.new_root,
        &inputs.validation,
        same_range_other_checkpoint_receipt,
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        CheckpointReadInterlockDenial::CheckpointCutoverReceiptMismatch
    ));
}

#[test]
fn checkpoint_readmission_rejects_same_epoch_root_from_different_checkpoint_identity() {
    let inputs = checkpoint_inputs();
    let other_validation =
        checkpoint_durability_fixture::validate(checkpoint_basis_fixture::manifest(10, 20, 14));
    let wrong_identity_root = CheckpointPublicationRoot::from_checkpoint_publication(
        inputs
            .new_authority
            .root_epoch_basis()
            .checkpoint_publication_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_checkpoint_id(other_validation.checkpoint_id()),
    )
    .unwrap();

    let denial = CheckpointPublicationReadmission::admit(
        wrong_identity_root,
        inputs.new_root,
        &inputs.validation,
        inputs.cutover_receipt,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        CheckpointReadInterlockDenial::CheckpointPublicationRootCheckpointMismatch
    );
}

#[test]
fn copied_checkpoint_reports_and_self_comparison_do_not_admit_interlock() {
    assert_eq!(
        reject_copied_checkpoint_report_as_checkpoint_interlock(),
        CheckpointReadInterlockDenial::CopiedCheckpointReportCannotAdmitReadInterlock
    );
    assert_eq!(
        reject_same_run_self_comparison_as_checkpoint_interlock(),
        CheckpointReadInterlockDenial::SameRunSelfComparisonCannotAdmitReadInterlock
    );
}

#[test]
fn checkpoint_readmission_rejects_half_published_cutover_range_report() {
    let inputs = checkpoint_inputs();
    let corrupt_receipt =
        CheckpointCutoverReceipt::recovered_corrupt_cutover_report_for_certification(
            inputs.validation.checkpoint_id().clone(),
            inputs.cutover_receipt.profile_id(),
            checkpoint_basis_fixture::covered_range(10, 21),
            CheckpointRecoveryCounterSnapshot::new().with_cutover_decision(),
        );

    let denial = CheckpointPublicationReadmission::admit(
        inputs.checkpoint_root,
        inputs.new_root,
        &inputs.validation,
        corrupt_receipt,
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        CheckpointReadInterlockDenial::CheckpointCutoverRangeMismatch { .. }
    ));
}

#[test]
fn checkpoint_readmission_rejects_frontier_outside_cutover_range() {
    let inputs = checkpoint_inputs_with_manifest(frontier_outside_cutover_manifest());

    let denial = CheckpointPublicationReadmission::admit(
        inputs.checkpoint_root,
        inputs.new_root,
        &inputs.validation,
        inputs.cutover_receipt,
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        CheckpointReadInterlockDenial::PageLsnFrontierOutsideCutoverRange { .. }
    ));
}

struct CheckpointInputs {
    old_authority: forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    new_authority: forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    old_root: forge_store_physical_isolation::CurrentPhysicalRoot,
    checkpoint_root: CheckpointPublicationRoot,
    new_root: forge_store_physical_isolation::CurrentPhysicalRoot,
    validation: CheckpointValidation,
    cutover_receipt: CheckpointCutoverReceipt,
}

fn checkpoint_inputs() -> CheckpointInputs {
    checkpoint_inputs_with_manifest(checkpoint_basis_fixture::manifest(10, 20, 12))
}

fn checkpoint_inputs_with_manifest(manifest: CheckpointManifest) -> CheckpointInputs {
    let old_authority = physical_authority_from_complete_closeout();
    let new_authority = physical_authority_from_operation_digest_closeout("s5-phase9-checkpoint");
    let old_root = current_root_from_authority(&old_authority);
    let new_root = current_root_from_authority(&new_authority);
    let validation = checkpoint_durability_fixture::validate(manifest);
    let checkpoint_root = CheckpointPublicationRoot::from_checkpoint_publication(
        new_authority
            .root_epoch_basis()
            .checkpoint_publication_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_checkpoint_id(validation.checkpoint_id()),
    )
    .unwrap();
    let durability = checkpoint_durability_fixture::checkpoint_durability(&validation);
    let plan = CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
        validation.clone(),
        durability,
    )
    .unwrap();
    let cutover_receipt = CheckpointCutoverReceipt::publish(plan);
    CheckpointInputs {
        old_authority,
        new_authority,
        old_root,
        checkpoint_root,
        new_root,
        validation,
        cutover_receipt,
    }
}

fn frontier_outside_cutover_manifest() -> CheckpointManifest {
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(checkpoint_basis_fixture::root_record_reference()),
        checkpoint_basis_fixture::frontier(21),
        checkpoint_basis_fixture::covered_range(10, 20),
        checkpoint_basis_fixture::redo_boundary(12),
        SharpCheckpointCertificationMode::certified(),
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
