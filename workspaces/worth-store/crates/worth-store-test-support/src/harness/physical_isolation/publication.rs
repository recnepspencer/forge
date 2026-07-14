use super::epoch_scope::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_complete_closeout_for_store,
    physical_authority_from_operation_digest_closeout,
    physical_authority_from_operation_digest_closeout_for_store,
};
use super::read_plan::{admit_plan, protected_set};
use crate::harness::recovery::closeout as closeout_fixture;
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference, RootPublicationValidationWitness,
};
use worth_store_physical_isolation::{
    CopyOnWritePublicationPlan, CrashStableFreeReusePosture, NewRootPublicationProof,
    OldReachabilityPreservation, PhysicalPublicationIntent, PhysicalPublicationReadiness,
    PhysicalReadPlanReleaseReceipt, PublicationLatchReadiness, PublicationRootCandidate,
    ReadCopyUpdateRootPublication, RootSwapOrderingContract,
};
use worth_store_recovery_physics::{
    ExecutedPublicationRecoveryReceipt, PublicationCrashStage, PublicationRecoveryReplayInput,
};

pub struct PublicationInputs {
    pub new_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    pub old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    pub old_candidate: PublicationRootCandidate,
    pub new_candidate: PublicationRootCandidate,
    pub old_reachability: OldReachabilityPreservation,
    pub old_release: PhysicalReadPlanReleaseReceipt,
    pub new_validation: RootPublicationValidationWitness,
}

pub fn publication_inputs() -> PublicationInputs {
    publication_inputs_with_new_root_digest("s5-phase7-new-root", 701)
}

pub fn publication_inputs_with_new_root_digest(
    operation_digest: &str,
    reference_generation: u64,
) -> PublicationInputs {
    let old_authority = physical_authority_from_complete_closeout();
    let old_root = current_root_from_authority(&old_authority);
    let (new_authority, new_root) = publication_successor(old_root, operation_digest);
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
        new_authority,
        old_root,
        new_root,
        old_candidate,
        new_candidate,
        old_reachability,
        old_release,
        new_validation,
    }
}

pub fn publication_inputs_for_store(
    store_identity: &worth_store_physical_format::PhysicalStoreIdentity,
    operation_digest: &str,
    reference_generation: u64,
) -> PublicationInputs {
    let old_authority = physical_authority_from_complete_closeout_for_store(store_identity);
    let old_root = current_root_from_authority(&old_authority);
    let (new_authority, new_root) =
        publication_successor_for_store(old_root, operation_digest, store_identity);
    publication_inputs_from_roots(
        old_authority,
        new_authority,
        old_root,
        new_root,
        reference_generation,
    )
}

pub fn successor_publication_inputs_for_store(
    prior: &PublicationInputs,
    store_identity: &worth_store_physical_format::PhysicalStoreIdentity,
    operation_digest: &str,
    reference_generation: u64,
) -> PublicationInputs {
    let old_authority = prior.new_authority.clone();
    let old_root = prior.new_root;
    let (new_authority, new_root) =
        publication_successor_for_store(old_root, operation_digest, store_identity);
    let new_validation = root_publication_validation(new_root.scope(), 3);
    publication_inputs_from_validations(
        old_authority,
        new_authority,
        old_root,
        new_root,
        prior.new_validation,
        new_validation,
        reference_generation,
    )
}

fn publication_successor(
    old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    operation_digest: &str,
) -> (
    worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    worth_store_physical_isolation::CurrentPhysicalRoot,
) {
    (0..1_024)
        .find_map(|attempt| {
            let candidate_digest = format!("{operation_digest}.{attempt}");
            let authority = physical_authority_from_operation_digest_closeout(&candidate_digest);
            let candidate = current_root_from_authority(&authority);
            (candidate.epoch().get() > old_root.epoch().get()
                && candidate.manifest_epoch().get() > old_root.manifest_epoch().get())
            .then_some((authority, candidate))
        })
        .expect("fixture must derive a publication successor with newer root and manifest epochs")
}

fn publication_successor_for_store(
    old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    operation_digest: &str,
    store_identity: &worth_store_physical_format::PhysicalStoreIdentity,
) -> (
    worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    worth_store_physical_isolation::CurrentPhysicalRoot,
) {
    (0..1_024)
        .find_map(|attempt| {
            let candidate_digest = format!("{operation_digest}.{attempt}");
            let authority = physical_authority_from_operation_digest_closeout_for_store(
                &candidate_digest,
                store_identity,
            );
            let candidate = current_root_from_authority(&authority);
            (candidate.epoch().get() > old_root.epoch().get()
                && candidate.manifest_epoch().get() > old_root.manifest_epoch().get())
            .then_some((authority, candidate))
        })
        .expect("fixture must derive a same-Store publication successor")
}

fn publication_inputs_from_roots(
    old_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    new_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    reference_generation: u64,
) -> PublicationInputs {
    let old_validation = root_publication_validation(old_root.scope(), 1);
    let new_validation = root_publication_validation(new_root.scope(), 2);
    publication_inputs_from_validations(
        old_authority,
        new_authority,
        old_root,
        new_root,
        old_validation,
        new_validation,
        reference_generation,
    )
}

fn publication_inputs_from_validations(
    old_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    new_authority: worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    old_validation: RootPublicationValidationWitness,
    new_validation: RootPublicationValidationWitness,
    reference_generation: u64,
) -> PublicationInputs {
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
        new_authority,
        old_root,
        new_root,
        old_candidate,
        new_candidate,
        old_reachability,
        old_release,
        new_validation,
    }
}

pub fn admitted_copy_on_write_plan(inputs: &PublicationInputs) -> CopyOnWritePublicationPlan {
    let intent = PhysicalPublicationIntent::copy_on_write_root_manifest(
        inputs.old_candidate,
        inputs.new_candidate,
        inputs.old_reachability,
    );
    let validated = intent.validate_copy_on_write_inputs().unwrap();
    let lowered = validated
        .clone()
        .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
        .unwrap();
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &validated,
        NewRootPublicationProof::from_root_validation(inputs.new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    lowered.join_readiness(readiness).unwrap()
}

pub fn mismatched_release_receipt(reference_generation: u64) -> PhysicalReadPlanReleaseReceipt {
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

pub fn publish_copy_on_write(
    intent: PhysicalPublicationIntent,
    new_validation: RootPublicationValidationWitness,
    reuse: Option<CrashStableFreeReusePosture>,
) -> worth_store_physical_isolation::PhysicalPublicationReceipt {
    publish_copy_on_write_result(intent, new_validation, reuse)
        .receipt()
        .clone()
}

pub fn publish_copy_on_write_result(
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

pub fn execute_publication_recovery_replay(
    stage: PublicationCrashStage,
) -> ExecutedPublicationRecoveryReceipt {
    let recovery_readiness = recovery_readiness_admission();
    recovery_readiness.execute_publication_recovery_replay(
        PublicationRecoveryReplayInput::from_crash_stage(stage),
    )
}

pub fn execute_mixed_tree_recovery_replay() -> ExecutedPublicationRecoveryReceipt {
    let recovery_readiness = recovery_readiness_admission();
    let replay = PublicationRecoveryReplayInput::mixed_tree_fault_attempt(
        PublicationCrashStage::DuringPublication,
    );
    recovery_readiness.execute_publication_recovery_replay(replay)
}

fn recovery_readiness_admission() -> worth_store_recovery_physics::RecoveryCompletion {
    closeout_fixture::recovery_completion()
}

pub fn root_publication_validation(root: u64, generation: u64) -> RootPublicationValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .unwrap()
}
