use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry, PhysicalFrameHeader,
    PhysicalPageHeader, PhysicalRootManifest, SegmentManifestEntry, SegmentPageManifestEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSecurityMetadataEnvelope<T, M> {
    artifact: T,
    security_metadata: M,
}

impl<T, M> PhysicalSecurityMetadataEnvelope<T, M> {
    const fn new(artifact: T, security_metadata: M) -> Self {
        Self {
            artifact,
            security_metadata,
        }
    }
}

impl<T, M: Copy> PhysicalSecurityMetadataEnvelope<T, M> {
    pub const fn artifact(&self) -> &T {
        &self.artifact
    }

    pub const fn security_metadata(&self) -> M {
        self.security_metadata
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<PhysicalPageHeader, M> {
    pub const fn page_header(header: PhysicalPageHeader, security_metadata: M) -> Self {
        Self::new(header, security_metadata)
    }

    pub const fn header(&self) -> PhysicalPageHeader {
        self.artifact
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<PhysicalFrameHeader, M> {
    pub const fn frame_header(header: PhysicalFrameHeader, security_metadata: M) -> Self {
        Self::new(header, security_metadata)
    }

    pub const fn header(&self) -> PhysicalFrameHeader {
        self.artifact
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<PhysicalRootManifest, M> {
    pub const fn root_manifest(manifest: PhysicalRootManifest, security_metadata: M) -> Self {
        Self::new(manifest, security_metadata)
    }

    pub const fn manifest(&self) -> &PhysicalRootManifest {
        &self.artifact
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

impl<M> PhysicalSecurityMetadataEnvelope<SegmentManifestEntry, M> {
    pub const fn segment_manifest_entry(entry: SegmentManifestEntry, security_metadata: M) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<SegmentPageManifestEntry, M> {
    pub const fn segment_page_manifest_entry(
        entry: SegmentPageManifestEntry,
        security_metadata: M,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<ExtentManifestEntry, M> {
    pub const fn extent_manifest_entry(entry: ExtentManifestEntry, security_metadata: M) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<AllocationClassManifestEntry, M> {
    pub const fn allocation_class_manifest_entry(
        entry: AllocationClassManifestEntry,
        security_metadata: M,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}

impl<M> PhysicalSecurityMetadataEnvelope<FreeSpaceManifestEntry, M> {
    pub const fn free_space_manifest_entry(
        entry: FreeSpaceManifestEntry,
        security_metadata: M,
    ) -> Self {
        Self::new(entry, security_metadata)
    }
}
