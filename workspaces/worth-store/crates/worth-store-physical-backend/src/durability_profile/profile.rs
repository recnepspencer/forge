use super::WalDurabilityBarrierSet;

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendDurabilityProfileId {
    SimulatedStrictDurable,
    PosixFileFsyncDirFsync,
    WindowsFlushFileBuffers,
    MmapFlushNotDurabilityCertified,
    AdversarialLostFlush,
    AdversarialReorderedFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendDurabilitySupport {
    Certified,
    UnsupportedDurabilityCapability,
    AdversarialLostFlush,
}

pub trait BackendDurabilityProfile: sealed::Sealed + Copy + Clone + Eq + 'static {
    const ID: BackendDurabilityProfileId;
    const REQUIRED_BARRIERS: WalDurabilityBarrierSet;
    const SUPPORT: BackendDurabilitySupport;
}
