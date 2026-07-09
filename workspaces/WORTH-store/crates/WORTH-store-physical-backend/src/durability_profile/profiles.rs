use super::{
    profile::sealed, BackendDurabilityProfile, BackendDurabilityProfileId,
    BackendDurabilitySupport, WalDurabilityBarrier, WalDurabilityBarrierSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulatedStrictDurableProfile;

impl sealed::Sealed for SimulatedStrictDurableProfile {}

impl BackendDurabilityProfile for SimulatedStrictDurableProfile {
    const ID: BackendDurabilityProfileId = BackendDurabilityProfileId::SimulatedStrictDurable;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet =
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::SimulatedDurableCommit);
    const SUPPORT: BackendDurabilitySupport = BackendDurabilitySupport::Certified;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosixFileFsyncDirFsyncProfile;

impl sealed::Sealed for PosixFileFsyncDirFsyncProfile {}

impl BackendDurabilityProfile for PosixFileFsyncDirFsyncProfile {
    const ID: BackendDurabilityProfileId = BackendDurabilityProfileId::PosixFileFsyncDirFsync;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet =
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync);
    const SUPPORT: BackendDurabilitySupport = BackendDurabilitySupport::Certified;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowsFlushFileBuffersProfile;

impl sealed::Sealed for WindowsFlushFileBuffersProfile {}

impl BackendDurabilityProfile for WindowsFlushFileBuffersProfile {
    const ID: BackendDurabilityProfileId = BackendDurabilityProfileId::WindowsFlushFileBuffers;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet =
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WindowsFlushFileBuffers)
            .insert(WalDurabilityBarrier::WindowsDirectorySync);
    const SUPPORT: BackendDurabilitySupport = BackendDurabilitySupport::Certified;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapFlushNotDurabilityCertifiedProfile;

impl sealed::Sealed for MmapFlushNotDurabilityCertifiedProfile {}

impl BackendDurabilityProfile for MmapFlushNotDurabilityCertifiedProfile {
    const ID: BackendDurabilityProfileId =
        BackendDurabilityProfileId::MmapFlushNotDurabilityCertified;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet = WalDurabilityBarrierSet::EMPTY;
    const SUPPORT: BackendDurabilitySupport =
        BackendDurabilitySupport::UnsupportedDurabilityCapability;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdversarialLostFlushProfile;

impl sealed::Sealed for AdversarialLostFlushProfile {}

impl BackendDurabilityProfile for AdversarialLostFlushProfile {
    const ID: BackendDurabilityProfileId = BackendDurabilityProfileId::AdversarialLostFlush;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet =
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync);
    const SUPPORT: BackendDurabilitySupport = BackendDurabilitySupport::AdversarialLostFlush;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdversarialReorderedFlushProfile;

impl sealed::Sealed for AdversarialReorderedFlushProfile {}

impl BackendDurabilityProfile for AdversarialReorderedFlushProfile {
    const ID: BackendDurabilityProfileId = BackendDurabilityProfileId::AdversarialReorderedFlush;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet =
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync)
            .insert(WalDurabilityBarrier::OrderedPersistenceFence);
    const SUPPORT: BackendDurabilitySupport = BackendDurabilitySupport::Certified;
}
