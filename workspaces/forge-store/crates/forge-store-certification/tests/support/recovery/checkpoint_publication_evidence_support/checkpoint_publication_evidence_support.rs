use forge_store_physical_backend::SimulatedStrictDurableProfile;
use forge_store_physical_isolation::{
    CheckpointInterlockFoundationalEvidence, CheckpointPublicationIdentity,
    CheckpointPublicationReadmission, CheckpointPublicationRoot,
    CheckpointPublicationStabilityProof, CheckpointReadInterlockPlan,
    CheckpointRootEpochTransition, PhysicalOrderingContract, ReadDuringCheckpointVerdict,
    StablePhysicalReadExecution,
};
use forge_store_recovery_physics::{CheckpointCutoverReceipt, CheckpointPublicationPlan};

use super::{
    checkpoint_basis_fixture, checkpoint_durability_fixture, epoch_support, plan_admission,
};

pub(crate) fn checkpoint_evidence_for_operation(
    operation_digest: &str,
    covered_start: u64,
    covered_end: u64,
    redo_boundary: u64,
) -> CheckpointInterlockFoundationalEvidence {
    let verdict = admitted_checkpoint_verdict_for(
        operation_digest,
        covered_start,
        covered_end,
        redo_boundary,
    );
    CheckpointInterlockFoundationalEvidence::after_executed_interlock(&verdict)
}

fn admitted_checkpoint_verdict_for(
    operation_digest: &str,
    covered_start: u64,
    covered_end: u64,
    redo_boundary: u64,
) -> ReadDuringCheckpointVerdict {
    let old_authority = epoch_support::physical_authority_from_complete_closeout();
    let new_authority =
        epoch_support::physical_authority_from_operation_digest_closeout(operation_digest);
    let old_root = epoch_support::current_root_from_authority(&old_authority);
    let new_root = epoch_support::current_root_from_authority(&new_authority);
    let validation = checkpoint_durability_fixture::validate(checkpoint_basis_fixture::manifest(
        covered_start,
        covered_end,
        redo_boundary,
    ));
    let checkpoint_root = CheckpointPublicationRoot::from_checkpoint_publication(
        new_authority
            .root_epoch_basis()
            .checkpoint_publication_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_checkpoint_id(validation.checkpoint_id()),
    )
    .unwrap();
    let durability = checkpoint_durability_fixture::checkpoint_durability(&validation);
    let publication = CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
        validation.clone(),
        durability,
    )
    .unwrap();
    let cutover = CheckpointCutoverReceipt::publish(publication);
    let readmission =
        CheckpointPublicationReadmission::admit(checkpoint_root, new_root, &validation, cutover)
            .unwrap();
    let transition = CheckpointRootEpochTransition::admit(old_root, readmission).unwrap();
    let pre_read = execute_read(
        &old_authority,
        old_root,
        epoch_support::current_generation_page_reference(911),
    );
    let post_read = execute_read(
        &new_authority,
        new_root,
        epoch_support::current_generation_page_reference(912),
    );
    let plan = CheckpointReadInterlockPlan::admit(pre_read, transition).unwrap();
    let proof =
        CheckpointPublicationStabilityProof::from_plan_and_post_publication_read(plan, post_read)
            .unwrap();
    ReadDuringCheckpointVerdict::from_stability_proof(proof).unwrap()
}

fn execute_read(
    authority: &forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: forge_store_physical_isolation::CurrentPhysicalRoot,
    reference: forge_store_physical_isolation::CurrentGenerationPhysicalReference,
) -> forge_store_physical_isolation::StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        plan_admission::admit_plan(
            authority,
            root,
            plan_admission::protected_set([reference], 4),
            8,
            4,
        )
        .into_execution_ready_handle(),
    )
    .complete()
}
