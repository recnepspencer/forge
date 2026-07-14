use crate::harness::physical_isolation::{epoch_scope, read_plan};
use crate::harness::recovery::{closeout, source_precedence};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference, RootPublicationValidationWitness,
};
use forge_store_physical_integrity::CompactionSourceIntegrityClearance;
use forge_store_physical_isolation::{
    admit_physical_isolation_entry, admit_physical_read_stability_authority,
    CompactionCandidateRangeSet, CompactionCutoverDelta, CompactionCutoverStabilityProof,
    CompactionProtectedReferenceSet, CompactionReadInterlockPlan, CompactionRewritePublication,
    CompactionSourceIntegrityEvidence, CurrentPhysicalRoot, NewRootPublicationProof,
    OldReachabilityPreservation, PhysicalIsolationEntryRequest, PhysicalPublicationIntent,
    PhysicalPublicationReadiness, PublicationLatchReadiness, PublicationRootCandidate,
    ReadCopyUpdateRootPublication, RootSwapOrderingContract, StablePhysicalReadExecution,
    StablePhysicalReadReceipt,
};
use forge_store_recovery_physics::{CompactionCutoverRecoveryPosture, RecoveryCompletion};

#[derive(Debug)]
pub struct ExecutedCompactionCutover {
    publication: CompactionRewritePublication,
    recovery: CompactionCutoverRecoveryPosture,
    pre_cutover_read: StablePhysicalReadReceipt,
    post_cutover_read: StablePhysicalReadReceipt,
}

impl ExecutedCompactionCutover {
    pub fn into_parts(
        self,
    ) -> (
        CompactionRewritePublication,
        CompactionCutoverRecoveryPosture,
        StablePhysicalReadReceipt,
        StablePhysicalReadReceipt,
    ) {
        (
            self.publication,
            self.recovery,
            self.pre_cutover_read,
            self.post_cutover_read,
        )
    }
}

pub fn admitted_compaction_plan() -> CompactionReadInterlockPlan {
    admitted_compaction_plan_for_seed(17)
}

pub fn admitted_compaction_plan_for_seed(root_seed: u64) -> CompactionReadInterlockPlan {
    let old_authority = physical_authority(closeout::recovery_completion());
    let old_root = epoch_scope::current_root_from_authority(&old_authority);
    let new_root = advancing_root(root_seed, old_root);
    let reference = epoch_scope::current_generation_page_reference(root_seed.max(1));
    let protected_references = read_plan::protected_set([reference], 1);
    let stable_plan =
        read_plan::admit_plan(&old_authority, old_root, protected_references, 4_096, 1);
    let protected = CompactionProtectedReferenceSet::from_read_plan(&stable_plan);
    let stable_read = StablePhysicalReadExecution::from_execution_ready_handle(
        stable_plan.into_execution_ready_handle(),
    )
    .complete();
    let integrity = source_precedence::intact_wal_integrity_evidence_for_owner(reference.owner());
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&integrity)
        .expect("ordinary integrity evidence clears the compaction source");
    let source =
        CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
            stable_read,
            clearance,
        )
        .expect("executed stable read and integrity locality agree");
    let candidates = CompactionCandidateRangeSet::from_current_generation_refs([reference])
        .expect("fixture candidate is current-generation physical authority");

    CompactionReadInterlockPlan::admit(
        protected,
        candidates,
        old_root.epoch(),
        new_root.epoch(),
        source,
    )
    .expect("ordinary compaction plan admission")
}

pub fn execute_compaction_cutover(plan: &CompactionReadInterlockPlan) -> ExecutedCompactionCutover {
    execute_compaction_cutover_for_manifest(
        plan,
        plan.protected().root().manifest_epoch().get() + 1,
    )
}

pub fn execute_compaction_cutover_for_manifest(
    plan: &CompactionReadInterlockPlan,
    target_manifest_epoch: u64,
) -> ExecutedCompactionCutover {
    let old_root = plan.protected().root();
    let delta = CompactionCutoverDelta::lower_to_manifest(plan.clone(), target_manifest_epoch)
        .expect("physical owner lowers the target compaction manifest");
    let new_root = delta.rewritten_root();
    let old_validation = root_validation(old_root);
    let new_validation = root_validation(new_root);
    let old_candidate = PublicationRootCandidate::admit(old_root, old_validation)
        .expect("old root publication candidate");
    let new_candidate = PublicationRootCandidate::admit(new_root, new_validation)
        .expect("new root publication candidate");
    let old_reachability =
        OldReachabilityPreservation::from_protected_footprint(plan.protected().footprint_basis())
            .expect("protected footprint preserves old reachability");
    let validated = PhysicalPublicationIntent::copy_on_write_root_manifest(
        old_candidate,
        new_candidate,
        old_reachability,
    )
    .validate_copy_on_write_inputs()
    .expect("copy-on-write publication inputs");
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &validated,
        NewRootPublicationProof::from_root_validation(new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    let receipt = ReadCopyUpdateRootPublication::publish(
        validated
            .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
            .expect("publication ordering")
            .join_readiness(readiness)
            .expect("publication readiness"),
    )
    .expect("root publication")
    .receipt()
    .clone();
    let publication = CompactionRewritePublication::publish_rewrite(delta, receipt)
        .expect("compaction publication binds the exact rewrite");
    let recovery = CompactionCutoverRecoveryPosture::admit_visible_product(
        source_precedence::compaction_visible_product_evidence(target_manifest_epoch),
    );
    let pre_cutover_read = plan
        .source_integrity()
        .stable_read_receipt()
        .expect("ordinary plan retains its executed source read");
    let stability = CompactionCutoverStabilityProof::admit(publication.clone(), recovery.clone())
        .expect("published rewrite is recovery-visible");
    let post_cutover_read = StablePhysicalReadExecution::from_execution_ready_handle(
        stability
            .plan_post_cutover_read()
            .expect("stability proof lowers the post-cutover read")
            .into_execution_ready_handle(),
    )
    .complete();

    ExecutedCompactionCutover {
        publication,
        recovery,
        pre_cutover_read,
        post_cutover_read,
    }
}

fn advancing_root(root_seed: u64, current: CurrentPhysicalRoot) -> CurrentPhysicalRoot {
    for ordinal in 0..512 {
        let completion = closeout::recovery_completion_with_operation_digest(&format!(
            "compaction-fixture-{root_seed}-{ordinal}"
        ));
        let authority = physical_authority(completion);
        let candidate = epoch_scope::current_root_from_authority(&authority);
        if candidate.epoch().get() > current.epoch().get() {
            return candidate;
        }
    }
    panic!("unable to derive an advancing compaction root")
}

fn physical_authority(
    completion: RecoveryCompletion,
) -> forge_store_physical_isolation::PhysicalReadStabilityAuthority {
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .expect("physical isolation entry");
    admit_physical_read_stability_authority(&entry).expect("physical read stability authority")
}

fn root_validation(root: CurrentPhysicalRoot) -> RootPublicationValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .root_publication_cell(
            PhysicalRootReference::from_raw(root.scope()).expect("nonzero root scope"),
        )
        .with_root_publication_generation(
            PhysicalGeneration::from_raw(root.scope()).expect("nonzero root generation"),
        );
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .expect("root publication validation")
}
