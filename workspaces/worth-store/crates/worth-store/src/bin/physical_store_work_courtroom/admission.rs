use std::{
    num::{NonZeroU32, NonZeroU64},
    path::Path,
};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalDurabilityPolicy, CheckpointMemoryLimit, FilesystemMediaAdmission,
    GroupCommitDelay, GroupCommitLimit, IdempotencyRetentionGenerations,
    LiveIdempotencyBindingLimit, MediaOwnedPhysicalRuntime, PendingUnresolvedMutationLimit,
    PhysicalCheckpointPolicy, PhysicalDurabilityDeclaration, PhysicalIdempotencyPolicy,
    PhysicalRuntimeAdmission, PhysicalStore, PhysicalWalPolicy, RecordServingAdmissionOutcome,
    RetainedWalTailLimit, ServingPhysicalRuntime, WalSegmentByteLimit, WalSegmentInventoryLimit,
};
use worth_store_physical_backend::{FilesystemAccessPosture, MediaFaultSchedule};

pub(super) fn admit_media(
    root: &Path,
    fault_schedule: Option<MediaFaultSchedule>,
) -> Result<MediaOwnedPhysicalRuntime, String> {
    let runtime = PhysicalStore::admit(
        PhysicalRuntimeAdmission::new(root)
            .map_err(|denial| format!("courtroom root denied: {denial:?}"))?,
    )
    .map_err(|denial| format!("courtroom runtime denied: {denial:?}"))?;
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let admission = match fault_schedule {
        Some(schedule) => admission.with_fault_schedule(schedule),
        None => admission,
    };
    match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => Ok(media),
        TransitionOutcome::Denied(_) => Err("courtroom media admission was denied".to_owned()),
        TransitionOutcome::Deferred(_) => Err("courtroom media admission was deferred".to_owned()),
        TransitionOutcome::Stale(_) => Err("courtroom media admission was stale".to_owned()),
        TransitionOutcome::RebindRequired(_) => {
            Err("courtroom media admission required rebinding".to_owned())
        }
        TransitionOutcome::Failed(_) => Err("courtroom media required inspection".to_owned()),
    }
}

pub(super) fn require_serving<Denial>(
    outcome: RecordServingAdmissionOutcome<Denial>,
    operation: &str,
) -> Result<ServingPhysicalRuntime, String> {
    match outcome.into_raw() {
        TransitionOutcome::Success(serving) => Ok(serving),
        TransitionOutcome::Denied(_) => Err(format!("{operation} was denied")),
        TransitionOutcome::Deferred(_) => Err(format!("{operation} was deferred")),
        TransitionOutcome::Stale(_) => Err(format!("{operation} was stale")),
        TransitionOutcome::RebindRequired(_) => Err(format!("{operation} required rebinding")),
        TransitionOutcome::Failed(inspection) => Err(format!(
            "{operation} required inspection: {:?}",
            inspection.cause()
        )),
    }
}

pub(super) fn admit_durability(
    media: &MediaOwnedPhysicalRuntime,
) -> Result<AdmittedPhysicalDurabilityPolicy, String> {
    admit_durability_with_checkpoint_memory(
        media,
        CheckpointMemoryLimit::new(NonZeroU64::new(16 * 1024 * 1024).unwrap()),
    )
}

pub(super) fn admit_durability_with_checkpoint_memory(
    media: &MediaOwnedPhysicalRuntime,
    checkpoint_memory: CheckpointMemoryLimit,
) -> Result<AdmittedPhysicalDurabilityPolicy, String> {
    let basis = media
        .physical_durability_admission_basis()
        .map_err(|denial| format!("courtroom durability basis denied: {denial:?}"))?;
    match PhysicalDurabilityDeclaration::builder()
        .group_commit(
            GroupCommitLimit::new(NonZeroU32::new(32).unwrap()),
            GroupCommitDelay::new(NonZeroU64::new(1).unwrap()),
        )
        .wal(PhysicalWalPolicy::segmented(
            WalSegmentByteLimit::new(NonZeroU64::new(8 * 1024 * 1024).unwrap()),
            WalSegmentInventoryLimit::new(NonZeroU32::new(1_024).unwrap()),
        ))
        .idempotency(PhysicalIdempotencyPolicy::new(
            IdempotencyRetentionGenerations::new(NonZeroU64::new(4).unwrap()),
            PendingUnresolvedMutationLimit::new(NonZeroU32::new(1_024).unwrap()),
            LiveIdempotencyBindingLimit::new(NonZeroU32::new(4_096).unwrap()),
        ))
        .checkpoint(PhysicalCheckpointPolicy::fuzzy(
            checkpoint_memory,
            RetainedWalTailLimit::new(NonZeroU64::new(64 * 1024 * 1024).unwrap()),
        ))
        .admit(basis)
        .into_raw()
    {
        TransitionOutcome::Success(policy) => Ok(policy),
        TransitionOutcome::Denied(denial) => {
            Err(format!("courtroom durability policy denied: {denial:?}"))
        }
        TransitionOutcome::Deferred(_) => {
            Err("courtroom durability policy was deferred".to_owned())
        }
        TransitionOutcome::Stale(_) => Err("courtroom durability policy was stale".to_owned()),
        TransitionOutcome::RebindRequired(_) => {
            Err("courtroom durability policy required rebinding".to_owned())
        }
        TransitionOutcome::Failed(_) => {
            Err("courtroom durability policy required inspection".to_owned())
        }
    }
}
