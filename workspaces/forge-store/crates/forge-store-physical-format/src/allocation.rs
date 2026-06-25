#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AllocationClassKind {
    OrdinaryRecordPage,
    LargeRecordExtent,
    RootManifest,
    SegmentManifest,
    ExtentManifest,
    FreeSpaceMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceMapVocabulary {
    allocation_class: AllocationClassKind,
}

impl FreeSpaceMapVocabulary {
    /// ```compile_fail
    /// use forge_store_physical_format::{AllocationClassKind, FreeSpaceMapVocabulary};
    ///
    /// let _ = FreeSpaceMapVocabulary::for_allocation_class(AllocationClassKind::OrdinaryRecordPage);
    /// ```
    pub const fn for_free_space_map() -> Self {
        Self {
            allocation_class: AllocationClassKind::FreeSpaceMap,
        }
    }

    pub const fn allocation_class(&self) -> AllocationClassKind {
        self.allocation_class
    }
}
