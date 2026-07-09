use super::{
    CheckpointPublicationReadmission, CheckpointPublicationStabilityProof,
    CheckpointReadInterlockPlan, CheckpointRootEpochTransition, ReadDuringCheckpointVerdict,
};
use crate::stable_read_execution::stable_physical_read_receipt_for_certification_root;
use crate::{
    epoch::{manifest_epoch_from_entry_seed, root_epoch_from_entry_seed},
    CheckpointPublicationIdentity, CheckpointPublicationRoot, CheckpointPublicationRootBasis,
    CurrentPhysicalRoot, CurrentPhysicalRootBasis, PhysicalOrderingContract,
};
use worth_store_physical_format::PhysicalRootReference;
use worth_store_recovery_physics::{
    CheckpointCoveredLsnRange, CheckpointCutoverReceipt, CheckpointManifest,
    CheckpointPageLsnFrontier, CheckpointRedoBoundary, CheckpointRootPosture, CheckpointValidation,
    IntegrityDamageMap, LocatedCheckpointCandidate, LogSequenceNumber, PageLsn,
    SharpCheckpointCertificationMode,
};

pub fn read_during_checkpoint_verdict_for_certification_test() -> ReadDuringCheckpointVerdict {
    let old_root = current_root_for_certification_test(41);
    let new_root = current_root_for_certification_test(42);
    let validation = checkpoint_validation_for_certification_test();
    let checkpoint_root = CheckpointPublicationRoot::from_checkpoint_publication(
        CheckpointPublicationRootBasis::new(new_root.epoch()),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_checkpoint_id(validation.checkpoint_id()),
    )
    .expect("certification checkpoint root should admit");
    let cutover_receipt = CheckpointCutoverReceipt::for_certification_test(&validation);
    let readmission = CheckpointPublicationReadmission::admit(
        checkpoint_root,
        new_root,
        &validation,
        cutover_receipt,
    )
    .expect("certification checkpoint readmission should admit");
    let transition = CheckpointRootEpochTransition::admit(old_root, readmission)
        .expect("certification checkpoint transition should admit");
    let pre_read = stable_physical_read_receipt_for_certification_root(old_root, 64);
    let post_read = stable_physical_read_receipt_for_certification_root(new_root, 64);
    let plan = CheckpointReadInterlockPlan::admit(pre_read, transition)
        .expect("certification checkpoint plan should admit");
    let proof =
        CheckpointPublicationStabilityProof::from_plan_and_post_publication_read(plan, post_read)
            .expect("certification checkpoint stability proof should admit");
    ReadDuringCheckpointVerdict::from_stability_proof(proof)
        .expect("certification checkpoint verdict should admit")
}

fn checkpoint_validation_for_certification_test() -> CheckpointValidation {
    let manifest = CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(
            PhysicalRootReference::from_raw(1).expect("root reference"),
        ),
        CheckpointPageLsnFrontier::from_pages([(
            worth_store_physical_format::PhysicalGenerationAuthority::s1()
                .page_cell(
                    worth_store_physical_format::PhysicalSegmentId::from_raw(1).expect("segment"),
                    worth_store_physical_format::PhysicalPageId::from_raw(1).expect("page"),
                )
                .with_page_generation(
                    worth_store_physical_format::PhysicalGeneration::from_raw(1)
                        .expect("generation"),
                ),
            PageLsn::from_lsn(LogSequenceNumber::new(12)),
        )])
        .expect("page lsn frontier"),
        CheckpointCoveredLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(20))
            .expect("covered range"),
        CheckpointRedoBoundary::from_page_lsn(PageLsn::from_lsn(LogSequenceNumber::new(12))),
        SharpCheckpointCertificationMode::certified(),
    )
    .expect("certification checkpoint manifest should admit");
    CheckpointValidation::validate_located_checkpoint(
        LocatedCheckpointCandidate::from_manifest_for_certification_test(manifest),
        &IntegrityDamageMap::new(),
    )
    .expect("certification checkpoint validation should admit")
}

fn current_root_for_certification_test(seed: u64) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_s5_entry(
        CurrentPhysicalRootBasis::new(
            root_epoch_from_entry_seed(seed),
            manifest_epoch_from_entry_seed(seed),
        ),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .expect("certification root ordering should admit")
}
