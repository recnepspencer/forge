use std::num::{NonZeroU32, NonZeroU64};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalDurabilityPolicy, CheckpointMemoryLimit, FilesystemMediaAdmission,
    GroupCommitDelay, GroupCommitLimit, IdempotencyRetentionGenerations,
    MediaOwnedPhysicalRuntime, PendingUnresolvedMutationLimit, PhysicalCheckpointPolicy,
    PhysicalDurabilityDeclaration, PhysicalIdempotencyPolicy, PhysicalRuntimeAdmission,
    PhysicalStore, RetainedWalTailLimit,
};
use worth_store_physical_backend::FilesystemAccessPosture;

pub fn media(label: &str) -> MediaOwnedPhysicalRuntime {
    let runtime = PhysicalStore::admit(
        PhysicalRuntimeAdmission::new(std::env::temp_dir().join(label)).unwrap(),
    )
    .unwrap();
    match runtime
        .try_admit_filesystem_media(FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        ))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("positive authority specimen requires admitted filesystem media"),
    }
}

pub fn admitted(media: &MediaOwnedPhysicalRuntime) -> AdmittedPhysicalDurabilityPolicy {
    let basis = media.physical_durability_admission_basis().unwrap();
    match PhysicalDurabilityDeclaration::builder()
        .group_commit(
            GroupCommitLimit::new(NonZeroU32::new(32).unwrap()),
            GroupCommitDelay::new(NonZeroU64::new(1).unwrap()),
        )
        .idempotency(PhysicalIdempotencyPolicy::new(
            IdempotencyRetentionGenerations::new(NonZeroU64::new(4).unwrap()),
            PendingUnresolvedMutationLimit::new(NonZeroU32::new(1_024).unwrap()),
        ))
        .checkpoint(PhysicalCheckpointPolicy::fuzzy(
            CheckpointMemoryLimit::new(NonZeroU64::new(16 * 1024 * 1024).unwrap()),
            RetainedWalTailLimit::new(NonZeroU64::new(64 * 1024 * 1024).unwrap()),
        ))
        .admit(basis)
        .into_raw()
    {
        TransitionOutcome::Success(policy) => policy,
        _ => panic!("positive authority specimen requires admitted durability"),
    }
}
