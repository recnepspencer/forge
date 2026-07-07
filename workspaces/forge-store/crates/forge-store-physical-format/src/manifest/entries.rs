use crate::{
    AllocationClassKind, ExtentGenerationCell, FreeSpaceReuseCell, SegmentGenerationCell,
    SlotGenerationCell,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentManifestEntry {
    segment: SegmentGenerationCell,
}

impl SegmentManifestEntry {
    pub const fn new(segment: SegmentGenerationCell) -> Self {
        Self { segment }
    }

    pub const fn segment(self) -> SegmentGenerationCell {
        self.segment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentPageManifestEntry {
    page_slot: SlotGenerationCell,
}

impl SegmentPageManifestEntry {
    pub const fn new(page_slot: SlotGenerationCell) -> Self {
        Self { page_slot }
    }

    pub const fn page_slot(self) -> SlotGenerationCell {
        self.page_slot
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentManifestEntry {
    extent: ExtentGenerationCell,
}

impl ExtentManifestEntry {
    pub const fn new(extent: ExtentGenerationCell) -> Self {
        Self { extent }
    }

    pub const fn extent(self) -> ExtentGenerationCell {
        self.extent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationClassManifestEntry {
    allocation_class: AllocationClassKind,
}

impl AllocationClassManifestEntry {
    pub const fn new(allocation_class: AllocationClassKind) -> Self {
        Self { allocation_class }
    }

    pub const fn allocation_class(self) -> AllocationClassKind {
        self.allocation_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceManifestEntry {
    reuse_cell: FreeSpaceReuseCell,
}

impl FreeSpaceManifestEntry {
    pub const fn new(reuse_cell: FreeSpaceReuseCell) -> Self {
        Self { reuse_cell }
    }

    pub const fn reuse_cell(self) -> FreeSpaceReuseCell {
        self.reuse_cell
    }
}
