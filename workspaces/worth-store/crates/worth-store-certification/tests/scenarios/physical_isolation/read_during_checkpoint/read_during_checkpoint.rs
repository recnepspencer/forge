use crate::checkpoint_read_fixture::{
    admitted_checkpoint_verdict, checkpoint_inputs, checkpoint_inputs_with_manifest,
};
use worth_store_test_support::harness::recovery::checkpoint_basis as checkpoint_basis_fixture;
use worth_store_test_support::harness::recovery::checkpoint_durability as checkpoint_durability_fixture;

use worth_store_physical_backend::SimulatedStrictDurableProfile;
use worth_store_physical_isolation::{
    reject_copied_checkpoint_report_as_checkpoint_interlock,
    reject_same_run_self_comparison_as_checkpoint_interlock, CheckpointPublicationIdentity,
    CheckpointPublicationReadmission, CheckpointPublicationRoot,
    CheckpointReadInterlockDenial, PhysicalOrderingContract,
};
use worth_store_recovery_physics::{
    CheckpointCutoverReceipt, CheckpointManifest, CheckpointPublicationPlan,
    CheckpointRecoveryCounterSnapshot, CheckpointRootPosture, SharpCheckpointCertificationMode,
};

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
