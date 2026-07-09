use worth_store_physical_backend::{
    BackendDurabilityBarrierAuthority, PosixFileFsyncDirFsyncAuthority,
    PosixFileFsyncDirFsyncProfile, WalDurabilityBarrier,
};
use worth_store_recovery_physics::{
    LogSequenceNumber, WalAppendPlan, WalAppendProgress, WalAppendReceipt,
    WalDurabilityObservationSequence, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};

pub fn completed_posix_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("posix");
    WalDurabilityObservationSequence::new(progress.clone())
        .completed(completed_barrier(
            &progress,
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .completed(completed_barrier(
            &progress,
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn completed_posix_receipt_for_range(
    start: u64,
    end_exclusive: u64,
) -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress_for_range("posix-range", start, end_exclusive);
    WalDurabilityObservationSequence::new(progress.clone())
        .completed(completed_barrier(
            &progress,
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .completed(completed_barrier(
            &progress,
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

fn posix_progress(digest_suffix: &str) -> WalAppendProgress<PosixFileFsyncDirFsyncProfile> {
    posix_progress_for_range(digest_suffix, 100, 101)
}

fn posix_progress_for_range(
    digest_suffix: &str,
    start: u64,
    end_exclusive: u64,
) -> WalAppendProgress<PosixFileFsyncDirFsyncProfile> {
    WalAppendPlan::new(
        WalSegmentId::new(42).unwrap(),
        WalSegmentGeneration::new(7).unwrap(),
        WalLsnRange::new(
            LogSequenceNumber::new(start),
            LogSequenceNumber::new(end_exclusive),
        )
        .unwrap(),
        format!("page-lsn-frame-digest-{digest_suffix}"),
        4096,
    )
    .unwrap()
    .record_written_bytes(4096)
}

fn completed_barrier(
    progress: &WalAppendProgress<PosixFileFsyncDirFsyncProfile>,
    barrier: WalDurabilityBarrier,
) -> worth_store_physical_backend::WalDurabilityBarrierReceipt<
    PosixFileFsyncDirFsyncProfile,
    worth_store_recovery_physics::WalAppendDurabilityScope,
> {
    PosixFileFsyncDirFsyncAuthority::new()
        .certify_completed_barrier(progress.durability_scope(), barrier)
        .unwrap()
}
