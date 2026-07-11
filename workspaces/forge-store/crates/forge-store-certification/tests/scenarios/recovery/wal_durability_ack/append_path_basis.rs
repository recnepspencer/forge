use forge_store_physical_backend::{
    AdversarialReorderedFlushProfile, BackendDurabilityProfile, PosixFileFsyncDirFsyncProfile,
    SimulatedStrictDurableProfile, WindowsFlushFileBuffersProfile,
};
use forge_store_recovery_physics::{
    LogSequenceNumber, WalAppendPlan, WalAppendProgress, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};

pub fn base_plan<P: BackendDurabilityProfile>(digest_suffix: &str) -> WalAppendPlan<P> {
    WalAppendPlan::new(
        segment(),
        generation(),
        lsn_range(),
        format!("frame-digest-{digest_suffix}"),
        4096,
    )
    .unwrap()
}

pub fn segment() -> WalSegmentId {
    WalSegmentId::new(42).unwrap()
}

pub fn donor_segment() -> WalSegmentId {
    WalSegmentId::new(43).unwrap()
}

pub fn generation() -> WalSegmentGeneration {
    WalSegmentGeneration::new(7).unwrap()
}

pub fn lsn_range() -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(100), LogSequenceNumber::new(101)).unwrap()
}

pub fn posix_progress(digest_suffix: &str) -> WalAppendProgress<PosixFileFsyncDirFsyncProfile> {
    base_plan::<PosixFileFsyncDirFsyncProfile>(digest_suffix).record_written_bytes(4096)
}

pub fn simulated_progress() -> WalAppendProgress<SimulatedStrictDurableProfile> {
    base_plan::<SimulatedStrictDurableProfile>("simulated").record_written_bytes(4096)
}

pub fn windows_progress() -> WalAppendProgress<WindowsFlushFileBuffersProfile> {
    base_plan::<WindowsFlushFileBuffersProfile>("windows").record_written_bytes(4096)
}

pub fn reordered_progress() -> WalAppendProgress<AdversarialReorderedFlushProfile> {
    base_plan::<AdversarialReorderedFlushProfile>("reordered").record_written_bytes(4096)
}
