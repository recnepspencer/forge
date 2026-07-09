use worth_store_physical_backend::{
    AdversarialReorderedFlushAuthority, AdversarialReorderedFlushProfile,
    PosixFileFsyncDirFsyncAuthority, PosixFileFsyncDirFsyncProfile,
    SimulatedStrictDurabilityAuthority, SimulatedStrictDurableProfile, WalDurabilityBarrier,
    WindowsFlushFileBuffersAuthority, WindowsFlushFileBuffersProfile,
};
use worth_store_recovery_physics::{WalAppendReceipt, WalDurabilityObservationSequence};

use super::append_path_basis::{
    base_plan, posix_progress, reordered_progress, simulated_progress, windows_progress,
};
use super::certified_barrier_observations::{completed_barrier, observation_sequence};

pub fn completed_simulated_receipt() -> WalAppendReceipt<SimulatedStrictDurableProfile> {
    observation_sequence(base_plan::<SimulatedStrictDurableProfile>("simulated"))
        .observe(completed_barrier(
            &simulated_progress(),
            SimulatedStrictDurabilityAuthority::new(),
            WalDurabilityBarrier::SimulatedDurableCommit,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn completed_posix_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    completed_posix_receipt_from_file_then_directory_path()
}

pub fn completed_posix_receipt_from_file_then_directory_path(
) -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("posix");
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

pub fn completed_posix_receipt_from_directory_then_file_path(
) -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    let progress = posix_progress("posix");
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalDirectoryFsync,
        ))
        .unwrap()
        .observe(completed_barrier(
            &progress,
            PosixFileFsyncDirFsyncAuthority::new(),
            WalDurabilityBarrier::WalFileFsync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn completed_windows_receipt() -> WalAppendReceipt<WindowsFlushFileBuffersProfile> {
    let progress = windows_progress();
    WalDurabilityObservationSequence::new(progress.clone())
        .observe(completed_barrier(
            &progress,
            WindowsFlushFileBuffersAuthority::new(),
            WalDurabilityBarrier::WindowsFlushFileBuffers,
        ))
        .unwrap()
        .observe(completed_barrier(
            &progress,
            WindowsFlushFileBuffersAuthority::new(),
            WalDurabilityBarrier::WindowsDirectorySync,
        ))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn adversarial_reordered_completed_receipt(
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
        .observe(completed_barrier(
            &progress,
            AdversarialReorderedFlushAuthority::new(),
            WalDurabilityBarrier::OrderedPersistenceFence,
        ))
        .unwrap()
        .finish()
        .unwrap()
}
