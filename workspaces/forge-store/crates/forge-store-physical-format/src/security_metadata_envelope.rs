use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    SegmentManifestEntry, SegmentPageManifestEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSecurityMetadataEnvelope<T, M> {
    artifact: T,
    security_metadata: M,
}

impl<T, M: Copy> PhysicalSecurityMetadataEnvelope<T, M> {
    pub const fn artifact(&self) -> &T {
        &self.artifact
    }

    pub const fn security_metadata(&self) -> M {
        self.security_metadata
    }
}

pub type SegmentSecurityMetadataEnvelope<M> =
    PhysicalSecurityMetadataEnvelope<SegmentManifestEntry, M>;
pub type SegmentPageSecurityMetadataEnvelope<M> =
    PhysicalSecurityMetadataEnvelope<SegmentPageManifestEntry, M>;
pub type ExtentSecurityMetadataEnvelope<M> =
    PhysicalSecurityMetadataEnvelope<ExtentManifestEntry, M>;
pub type AllocationClassSecurityMetadataEnvelope<M> =
    PhysicalSecurityMetadataEnvelope<AllocationClassManifestEntry, M>;
pub type FreeSpaceSecurityMetadataEnvelope<M> =
    PhysicalSecurityMetadataEnvelope<FreeSpaceManifestEntry, M>;
