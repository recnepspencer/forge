use super::PhysicalStoreRuntime;
use crate::access::{
    allocation::AllocationAccess,
    extent::ExtentAccess,
    frame::FrameAccess,
    free_space::{fragmentation::FragmentationAccess, search::FreeSpaceAccess},
    manifest::{membership::ManifestAccess, root_discovery::RootDiscoveryAccess},
    page::PageAccess,
    segment::SegmentAccess,
};

impl PhysicalStoreRuntime {
    pub fn page_access(&mut self) -> PageAccess<'_> {
        PageAccess::new(self)
    }

    pub fn frame_access(&mut self) -> FrameAccess<'_> {
        FrameAccess::new(self)
    }

    pub fn segment_access(&mut self) -> SegmentAccess<'_> {
        SegmentAccess::new(self)
    }

    pub fn extent_access(&mut self) -> ExtentAccess<'_> {
        ExtentAccess::new(self)
    }

    pub fn root_manifest_access(&mut self) -> RootDiscoveryAccess<'_> {
        RootDiscoveryAccess::new(self)
    }

    pub fn manifest_index_access(&mut self) -> ManifestAccess<'_> {
        ManifestAccess::new(self)
    }

    pub fn allocation_access(&mut self) -> AllocationAccess<'_> {
        AllocationAccess::new(self)
    }

    pub fn free_space_access(&mut self) -> FreeSpaceAccess<'_> {
        FreeSpaceAccess::new(self)
    }

    pub fn fragmentation_access(&mut self) -> FragmentationAccess<'_> {
        FragmentationAccess::new(self)
    }
}
