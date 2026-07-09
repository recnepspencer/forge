use super::{CompactionCutoverDelta, CompactionReadInterlockPlan, CompactionRewritePublication};
use crate::{
    CurrentPhysicalRoot, CurrentPhysicalRootBasis, ManifestEpoch, NewRootPublicationProof,
    OldReachabilityPreservation, PhysicalOrderingContract, PhysicalPublicationIntent,
    PhysicalPublicationReadiness, PhysicalReadIoPosture, PhysicalReadPlanReleaseReceipt,
    PhysicalReadProtectedFootprintBasis, PublicationLatchReadiness, PublicationRootCandidate,
    ReadCopyUpdateRootPublication, RootSwapOrderingContract, StablePhysicalReadExecutionCounters,
    StablePhysicalReadReceipt,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference, RootPublicationValidationWitness,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

pub struct CompactionCutoverEvidenceForCertification {
    publication: CompactionRewritePublication,
    recovery: CompactionCutoverRecoveryPosture,
    pre_cutover_read: StablePhysicalReadReceipt,
    post_cutover_read: StablePhysicalReadReceipt,
}

impl CompactionCutoverEvidenceForCertification {
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

pub fn compaction_cutover_evidence_for_certification_plan(
    plan: &CompactionReadInterlockPlan,
) -> CompactionCutoverEvidenceForCertification {
    compaction_cutover_evidence_for_certification_rewrite_manifest(
        plan,
        plan.protected().root().manifest_epoch().get() + 1,
    )
}

pub fn compaction_cutover_evidence_for_certification_rewrite_manifest(
    plan: &CompactionReadInterlockPlan,
    rewritten_manifest_epoch: u64,
) -> CompactionCutoverEvidenceForCertification {
    let old_root = plan.protected().root();
    let new_root = CurrentPhysicalRoot::from_s5_entry(
        CurrentPhysicalRootBasis::new(
            plan.target_epoch(),
            ManifestEpoch::from_admitted_physical_basis(rewritten_manifest_epoch),
        ),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .expect("certification target root should admit");
    let old_validation = root_validation_for_certification_root(old_root);
    let new_validation = root_validation_for_certification_root(new_root);
    let old_candidate = PublicationRootCandidate::admit(old_root, old_validation)
        .expect("old root candidate should admit");
    let new_candidate = PublicationRootCandidate::admit(new_root, new_validation)
        .expect("new root candidate should admit");
    let old_reachability =
        OldReachabilityPreservation::from_protected_footprint(plan.protected().footprint_basis())
            .expect("protected footprint should preserve old reachability");
    let validated = PhysicalPublicationIntent::copy_on_write_root_manifest(
        old_candidate,
        new_candidate,
        old_reachability,
    )
    .validate_copy_on_write_inputs()
    .expect("copy-on-write publication should validate");
    let readiness = PhysicalPublicationReadiness::from_validated_intent(
        &validated,
        NewRootPublicationProof::from_root_validation(new_validation),
        PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
    );
    let publication_receipt = ReadCopyUpdateRootPublication::publish(
        validated
            .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
            .expect("publication lowering should admit")
            .join_readiness(readiness)
            .expect("publication readiness should join"),
    )
    .expect("publication should swap")
    .receipt()
    .clone();
    let publication = CompactionRewritePublication::publish(
        CompactionCutoverDelta::lower(plan.clone(), new_root)
            .expect("compaction cutover delta should lower"),
        publication_receipt,
    )
    .expect("compaction rewrite publication should admit");
    CompactionCutoverEvidenceForCertification {
        publication,
        recovery:
            CompactionCutoverRecoveryPosture::visible_after_admitted_cutover_for_certification_test(
                new_root.scope(),
            ),
        pre_cutover_read: read_receipt_for_root_and_footprint(
            old_root,
            plan.protected().footprint_basis(),
        ),
        post_cutover_read: read_receipt_for_root_and_footprint(
            new_root,
            plan.protected().footprint_basis(),
        ),
    }
}

fn root_validation_for_certification_root(
    root: CurrentPhysicalRoot,
) -> RootPublicationValidationWitness {
    let root_reference =
        PhysicalRootReference::from_raw(root.scope()).expect("nonzero root reference");
    let generation = PhysicalGeneration::from_raw(root.scope()).expect("nonzero generation");
    let cell = PhysicalGenerationAuthority::s1()
        .root_publication_cell(root_reference)
        .with_root_publication_generation(generation);
    let admission = PhysicalReferenceAuthority::s1().admit_root_publication(cell);
    PhysicalReferenceAuthority::s1()
        .validate_root_publication(admission, cell)
        .expect("root publication validation should admit")
}

fn read_receipt_for_root_and_footprint(
    root: CurrentPhysicalRoot,
    footprint: PhysicalReadProtectedFootprintBasis,
) -> StablePhysicalReadReceipt {
    StablePhysicalReadReceipt::new(
        PhysicalReadPlanReleaseReceipt::new(root, footprint),
        StablePhysicalReadExecutionCounters::for_certification_test(64),
        PhysicalReadIoPosture::ordinary(),
    )
}
