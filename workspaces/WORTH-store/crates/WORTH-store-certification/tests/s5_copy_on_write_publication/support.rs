use crate::closeout_fixture;
use crate::plan_admission::{admit_plan, protected_set};
use crate::support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference, RootPublicationValidationWitness,
};
use worth_store_physical_isolation::{
    CrashStableFreeReusePosture, NewRootPublicationProof, OldReachabilityPreservation,
    PhysicalPublicationIntent, PhysicalPublicationReadiness, PhysicalReadPlanReleaseReceipt,
    PublicationLatchReadiness, PublicationRootCandidate, ReadCopyUpdateRootPublication,
    RootSwapOrderingContract,
};
use worth_store_recovery_physics::{
    ExecutedS5PublicationRecoveryReceipt, S5PublicationCrashStage, S5PublicationRecoveryReplayInput,
};

pub(crate) struct PublicationInputs {
    pub(crate) old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub(crate) new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub(crate) old_candidate: PublicationRootCandidate,
    pub(crate) new_candidate: PublicationRootCandidate,
    pub(crate) old_reachability: OldReachabilityPreservation,
    pub(crate) old_release: PhysicalReadPlanReleaseReceipt,
    pub(crate) new_validation: RootPublicationValidationWitness,
}

pub(crate) fn publication_inputs() -> PublicationInputs {
    publication_inputs_with_new_root_digest("s5-phase7-new-root", 701)
}

pub(crate) fn publication_inputs_with_new_root_digest(
    operation_digest: &str,
    reference_generation: u64,
) -> PublicationInputs {
    let old_authority = physical_authority_from_complete_closeout();
    let new_authority = physical_authority_from_operation_digest_closeout(operation_digest);
    let old_root = current_root_from_authority(&old_authority);
    let new_root = current_root_from_authority(&new_authority);
    let old_validation = root_publication_validation(old_root.scope(), 1);
    let new_validation = root_publication_validation(new_root.scope(), 2);
    let old_candidate = PublicationRootCandidate::admit(old_root, old_validation).unwrap();
    let new_candidate = PublicationRootCandidate::admit(new_root, new_validation).unwrap();
    let reference = current_generation_page_reference(reference_generation);
    let old_plan = admit_plan(
        &old_authority,
        old_root,
        protected_set([reference], 4),
        8,
        4,
    );
    let old_reachability = OldReachabilityPreservation::from_protected_footprint(
        old_plan.footprint().declared_footprint_basis(),
    )
    .unwrap();
    let old_release = old_plan.into_execution_ready_handle().release();
    PublicationInputs {
        old_root,
        new_root,
        old_candidate,
        new_candidate,
        old_reachability,
        old_release,
        new_validation,
    }
}

pub(crate) fn mismatched_release_receipt(
    reference_generation: u64,
) -> PhysicalReadPlanReleaseReceipt {
    let old_authority = physical_authority_from_complete_closeout();
    let old_root = current_root_from_authority(&old_authority);
    let reference = current_generation_page_reference(reference_generation);
    admit_plan(
        &old_authority,
        old_root,
        protected_set([reference], 4),
        8,
        4,
    )
    .into_execution_ready_handle()
    .release()
}

pub(crate) fn publish_copy_on_write(
    intent: PhysicalPublicationIntent,
    new_validation: RootPublicationValidationWitness,
    reuse: Option<CrashStableFreeReusePosture>,
) -> worth_store_physical_isolation::PhysicalPublicationReceipt {
    publish_copy_on_write_result(intent, new_validation, reuse)
        .receipt()
        .clone()
}

pub(crate) fn publish_copy_on_write_result(
    intent: PhysicalPublicationIntent,
    new_validation: RootPublicationValidationWitness,
    reuse: Option<CrashStableFreeReusePosture>,
) -> ReadCopyUpdateRootPublication {
    let validated = intent.validate_copy_on_write_inputs().unwrap();
    let lowered = validated
        .clone()
        .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
        .unwrap();
    let mut readiness = PhysicalPublicationReadiness::from_validated_intent(
        &validated,
        NewRootPublicationProof::from_root_validation(new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    if let Some(reuse) = reuse {
        readiness = readiness.with_free_reuse_posture(reuse).unwrap();
    }
    ReadCopyUpdateRootPublication::publish(lowered.join_readiness(readiness).unwrap()).unwrap()
}

pub(crate) fn execute_publication_recovery_replay(
    stage: S5PublicationCrashStage,
) -> ExecutedS5PublicationRecoveryReceipt {
    let recovery_readiness = recovery_readiness_admission();
    recovery_readiness.execute_publication_recovery_replay(
        S5PublicationRecoveryReplayInput::from_crash_stage(stage),
    )
}

pub(crate) fn execute_mixed_tree_recovery_replay() -> ExecutedS5PublicationRecoveryReceipt {
    let recovery_readiness = recovery_readiness_admission();
    let replay = S5PublicationRecoveryReplayInput::mixed_tree_fault_attempt(
        S5PublicationCrashStage::DuringPublication,
    );
    recovery_readiness.execute_publication_recovery_replay(replay)
}

fn recovery_readiness_admission() -> worth_store_recovery_physics::S5RecoveryReadinessAdmission {
    closeout_fixture::certify_complete_closeout()
        .publish_s5_readiness()
        .admit_for_s5_startup()
        .unwrap()
}

pub(crate) fn root_publication_validation(
    root: u64,
    generation: u64,
) -> RootPublicationValidationWitness {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .unwrap()
}
