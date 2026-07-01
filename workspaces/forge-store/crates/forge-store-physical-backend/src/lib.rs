#![forbid(unsafe_code)]

mod durability_profile;
mod operation_boundary;

use forge_store_physical_format::PhysicalReference;

#[cfg(feature = "certification-test-authority")]
pub use durability_profile::{
    AdversarialLostFlushAuthority, AdversarialReorderedFlushAuthority,
    BackendDurabilityBarrierAuthority, BackendDurabilityBarrierDenial,
    BackendDurabilityBarrierDenialKind, MmapFlushNotDurabilityCertifiedAuthority,
    PosixFileFsyncDirFsyncAuthority, SimulatedStrictDurabilityAuthority,
    WindowsFlushFileBuffersAuthority,
};
pub use durability_profile::{
    AdversarialLostFlushProfile, AdversarialReorderedFlushProfile, BackendDurabilityProfile,
    BackendDurabilityProfileId, BackendDurabilitySupport, MmapFlushNotDurabilityCertifiedProfile,
    PosixFileFsyncDirFsyncProfile, SimulatedStrictDurableProfile, WalDurabilityBarrier,
    WalDurabilityBarrierReceipt, WalDurabilityBarrierSet, WindowsFlushFileBuffersProfile,
};
pub use operation_boundary::ProductionStorageBoundarySeam;

pub trait PhysicalStoreBackend {
    type Error;

    fn append_framed_record(&mut self, bytes: &[u8]) -> Result<PhysicalReference, Self::Error>;

    fn read_framed_record(&self, reference: PhysicalReference) -> Result<Vec<u8>, Self::Error>;
}
