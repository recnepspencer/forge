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
use std::num::NonZeroU64;
use worth_store_physical_format::{
    inspect_checkpoint_stream, store_namespace::ProposedStoreIdentity,
    store_namespace::StoreNamespaceIdentityRecord, store_namespace::StoreNamespaceVersion,
    CheckpointBindingCompactionHeader, CheckpointRootBasis, CheckpointStreamEncoder,
    CheckpointWalSourceRange, PhysicalCheckpointIdentity, PhysicalCheckpointSource,
    VerifiedCheckpointStream,
};

pub fn read_during_checkpoint_verdict_for_certification_test() -> ReadDuringCheckpointVerdict {
    let old_root = current_root_for_certification_test(41);
    let new_root = current_root_for_certification_test(42);
    let checkpoint = checkpoint_for_certification_test();
    let checkpoint_root = CheckpointPublicationRoot::from_checkpoint_publication(
        CheckpointPublicationRootBasis::new(new_root.epoch()),
        PhysicalOrderingContract::root_swap_acquire_release(),
        CheckpointPublicationIdentity::from_physical_checkpoint_identity(
            checkpoint.source().identity(),
        ),
    )
    .expect("certification checkpoint root should admit");
    let readmission =
        CheckpointPublicationReadmission::admit(checkpoint_root, new_root, &checkpoint)
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

fn checkpoint_for_certification_test() -> VerifiedCheckpointStream {
    let store_identity = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([7; 16]).expect("store identity"),
    )
    .published_identity();
    let source = PhysicalCheckpointSource::concurrent(
        PhysicalCheckpointIdentity::new(
            store_identity,
            NonZeroU64::new(1).expect("checkpoint sequence"),
        ),
        CheckpointWalSourceRange::new(10, 20).expect("checkpoint WAL range"),
        CheckpointRootBasis::new(1, 1),
        1,
    );
    let (encoder, mut bytes) = CheckpointStreamEncoder::begin(source);
    let (encoder, compaction_header) = encoder.begin_binding_compaction(
        CheckpointBindingCompactionHeader::new(1, 12).expect("compaction binding header"),
    );
    bytes.extend(compaction_header);
    let (_footer, footer) = encoder.finish();
    bytes.extend(footer);
    inspect_checkpoint_stream(&bytes, 0, 0).expect("certification checkpoint stream")
}

fn current_root_for_certification_test(seed: u64) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_physical_isolation_entry(
        CurrentPhysicalRootBasis::new(
            root_epoch_from_entry_seed(seed),
            manifest_epoch_from_entry_seed(seed),
            worth_store_physical_format::PhysicalStoreIdentity::physical_format_default()
                .authority_identity(),
        ),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .expect("certification root ordering should admit")
}
