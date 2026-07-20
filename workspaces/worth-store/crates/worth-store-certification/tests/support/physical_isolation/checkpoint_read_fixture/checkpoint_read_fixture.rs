use worth_store_test_support::harness::physical_isolation::epoch_scope as support;
use worth_store_test_support::harness::physical_isolation::read_plan as plan_admission;
use worth_store_test_support::harness::recovery::checkpoint_basis as checkpoint_basis_fixture;
use worth_store_test_support::harness::recovery::checkpoint_durability as checkpoint_durability_fixture;

use plan_admission::{admit_plan, protected_set};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};
use worth_store_physical_backend::SimulatedStrictDurableProfile;
use worth_store_physical_isolation::{
    CheckpointPublicationIdentity, CheckpointPublicationReadmission, CheckpointPublicationRoot,
    CheckpointPublicationStabilityProof, CheckpointReadInterlockPlan,
    CheckpointRootEpochTransition, PhysicalOrderingContract, ReadDuringCheckpointVerdict,
    StablePhysicalReadExecution,
};
use worth_store_recovery_physics::{
    CheckpointCutoverReceipt, CheckpointManifest, CheckpointPublicationPlan, CheckpointValidation,
};

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

pub(crate) struct CheckpointInputs {
    pub(crate) old_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    pub(crate) new_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    pub(crate) old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub(crate) checkpoint_root: CheckpointPublicationRoot,
    pub(crate) new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub(crate) validation: CheckpointValidation,
    pub(crate) cutover_receipt: CheckpointCutoverReceipt,
}

pub(crate) fn checkpoint_inputs() -> CheckpointInputs {
    checkpoint_inputs_with_manifest(checkpoint_basis_fixture::manifest(10, 20, 12))
}

pub(crate) fn checkpoint_inputs_with_manifest(manifest: CheckpointManifest) -> CheckpointInputs {
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

fn execute_read(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: worth_store_physical_isolation::CurrentPhysicalRoot,
    reference: worth_store_physical_isolation::CurrentGenerationPhysicalReference,
) -> worth_store_physical_isolation::StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        admit_plan(authority, root, protected_set([reference], 4), 8, 4)
            .into_execution_ready_handle(),
    )
    .complete()
}
