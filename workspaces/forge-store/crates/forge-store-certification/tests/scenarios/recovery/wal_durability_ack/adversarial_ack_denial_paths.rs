use forge_store_physical_backend::{
    AdversarialLostFlushProfile, AdversarialReorderedFlushAuthority,
    AdversarialReorderedFlushProfile, BackendDurabilityBarrierAuthority,
    BackendDurabilityBarrierDenial, MmapFlushNotDurabilityCertifiedProfile,
    PosixFileFsyncDirFsyncAuthority, PosixFileFsyncDirFsyncProfile, WalDurabilityBarrier,
};
use forge_store_recovery_physics::{
    IllegalAcknowledgmentDenial, WalAppendPlan, WalAppendReceipt, WalDurabilityObservation,
    WalDurabilityObservationSequence,
};

use super::append_path_basis::{
    base_plan, donor_segment, generation, lsn_range, posix_progress, reordered_progress,
};
use super::certified_barrier_observations::completed_barrier;

pub fn mismatched_barrier_scope_denial() -> IllegalAcknowledgmentDenial {
    let donor_progress = WalAppendPlan::<PosixFileFsyncDirFsyncProfile>::new(
        donor_segment(),
        generation(),
        lsn_range(),
        "frame-digest-donor",
        4096,
    )
    .unwrap()
    .record_written_bytes(4096);
    let donor_scope = donor_progress.durability_scope();
    let receipt = PosixFileFsyncDirFsyncAuthority::new()
        .certify_completed_barrier(donor_scope, WalDurabilityBarrier::WalFileFsync)
        .unwrap();

    base_plan::<PosixFileFsyncDirFsyncProfile>("scope-target")
        .record_written_bytes(4096)
        .complete_barrier(receipt)
        .unwrap_err()
}

pub fn mmap_receipt() -> WalAppendReceipt<MmapFlushNotDurabilityCertifiedProfile> {
    base_plan::<MmapFlushNotDurabilityCertifiedProfile>("mmap")
        .record_written_bytes(4096)
        .finish()
        .unwrap()
}

pub fn adversarial_lost_flush_profile_receipt() -> WalAppendReceipt<AdversarialLostFlushProfile> {
    base_plan::<AdversarialLostFlushProfile>("lost-profile")
        .record_written_bytes(4096)
        .finish()
        .unwrap()
}

pub fn adversarial_reordered_missing_fence_receipt(
) -> WalAppendReceipt<AdversarialReorderedFlushProfile> {
    let progress = reordered_progress();
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            AdversarialReorderedFlushAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .observe(completed_barrier(
            &progress,
            AdversarialReorderedFlushAuthority::new(),
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn short_write_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = base_plan::<PosixFileFsyncDirFsyncProfile>("short").record_written_bytes(128);
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn delayed_flush_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("delayed");
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .observe(WalDurabilityObservation::DelayedFlush(
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn lost_flush_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("lost");
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .observe(WalDurabilityObservation::LostFlush)
        .unwrap()
        .finish()
        .unwrap()
}

pub fn directory_sync_failed_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("dir-failed");
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .observe(WalDurabilityObservation::BarrierFailed(
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn missing_posix_directory_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("missing-dir");
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn posix_non_required_barrier_denial() -> BackendDurabilityBarrierDenial {
    let progress = posix_progress("wrong-barrier");
    PosixFileFsyncDirFsyncAuthority::new()
        .certify_completed_barrier(
            progress.durability_scope(),
            WalDurabilityBarrier::WindowsFlushFileBuffers,
        )
        .unwrap_err()
}
