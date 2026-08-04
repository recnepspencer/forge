use crate::{
    AllocationClassKind, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationOwner,
    PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotGenerationCell {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot: PhysicalRecordSlot,
    generation: PhysicalGeneration,
}

impl SlotGenerationCell {
    pub(crate) const fn new(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            segment_id,
            page_id,
            slot,
            generation,
        }
    }

    pub const fn segment_id(self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_id(self) -> PhysicalPageId {
        self.page_id
    }

    pub const fn slot(self) -> PhysicalRecordSlot {
        self.slot
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_slot(self.segment_id, self.page_id, self.slot, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotGenerationCellBuilder {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot: PhysicalRecordSlot,
}

impl SlotGenerationCellBuilder {
    pub(crate) const fn new(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
    ) -> Self {
        Self {
            segment_id,
            page_id,
            slot,
        }
    }

    pub const fn with_slot_generation(self, generation: PhysicalGeneration) -> SlotGenerationCell {
        SlotGenerationCell::new(self.segment_id, self.page_id, self.slot, generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentGenerationCell {
    segment_id: PhysicalSegmentId,
    extent_id: PhysicalExtentId,
    generation: PhysicalGeneration,
}

impl ExtentGenerationCell {
    pub(crate) const fn new(
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            segment_id,
            extent_id,
            generation,
        }
    }

    pub const fn segment_id(self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn extent_id(self) -> PhysicalExtentId {
        self.extent_id
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_extent(self.segment_id, self.extent_id, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentGenerationCellBuilder {
    segment_id: PhysicalSegmentId,
    extent_id: PhysicalExtentId,
}

impl ExtentGenerationCellBuilder {
    pub(crate) const fn new(segment_id: PhysicalSegmentId, extent_id: PhysicalExtentId) -> Self {
        Self {
            segment_id,
            extent_id,
        }
    }

    pub const fn with_extent_generation(
        self,
        generation: PhysicalGeneration,
    ) -> ExtentGenerationCell {
        ExtentGenerationCell::new(self.segment_id, self.extent_id, generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootPublicationCell {
    root_reference: PhysicalRootReference,
    generation: PhysicalGeneration,
}

impl RootPublicationCell {
    pub(crate) const fn new(
        root_reference: PhysicalRootReference,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            root_reference,
            generation,
        }
    }

    pub const fn root_reference(self) -> PhysicalRootReference {
        self.root_reference
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_root_publication(self.root_reference, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootPublicationCellBuilder {
    root_reference: PhysicalRootReference,
}

impl RootPublicationCellBuilder {
    pub(crate) const fn new(root_reference: PhysicalRootReference) -> Self {
        Self { root_reference }
    }

    pub const fn with_root_publication_generation(
        self,
        generation: PhysicalGeneration,
    ) -> RootPublicationCell {
        RootPublicationCell::new(self.root_reference, generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeSpaceReuseAddress {
    PageSlot {
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
    },
    Extent {
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceReuseCell {
    address: FreeSpaceReuseAddress,
    allocation_class: AllocationClassKind,
    generation: PhysicalGeneration,
}

impl FreeSpaceReuseCell {
    pub(crate) const fn new(
        address: FreeSpaceReuseAddress,
        allocation_class: AllocationClassKind,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            address,
            allocation_class,
            generation,
        }
    }

    pub const fn address(self) -> FreeSpaceReuseAddress {
        self.address
    }

    pub const fn allocation_class(self) -> AllocationClassKind {
        self.allocation_class
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_free_space(
            self.address,
            self.allocation_class,
            self.generation,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceReuseCellBuilder {
    address: FreeSpaceReuseAddress,
    allocation_class: AllocationClassKind,
}

impl FreeSpaceReuseCellBuilder {
    pub(crate) const fn new(
        address: FreeSpaceReuseAddress,
        allocation_class: AllocationClassKind,
    ) -> Self {
        Self {
            address,
            allocation_class,
        }
    }

    pub const fn with_free_space_generation(
        self,
        generation: PhysicalGeneration,
    ) -> FreeSpaceReuseCell {
        FreeSpaceReuseCell::new(self.address, self.allocation_class, generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageGenerationCell {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    generation: PhysicalGeneration,
}

impl PageGenerationCell {
    pub(crate) const fn new(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            segment_id,
            page_id,
            generation,
        }
    }

    pub const fn segment_id(self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_id(self) -> PhysicalPageId {
        self.page_id
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_page(self.segment_id, self.page_id, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageGenerationCellBuilder {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
}

impl PageGenerationCellBuilder {
    pub(crate) const fn new(segment_id: PhysicalSegmentId, page_id: PhysicalPageId) -> Self {
        Self {
            segment_id,
            page_id,
        }
    }

    pub const fn with_page_generation(self, generation: PhysicalGeneration) -> PageGenerationCell {
        PageGenerationCell::new(self.segment_id, self.page_id, generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentGenerationCell {
    segment_id: PhysicalSegmentId,
    generation: PhysicalGeneration,
}

impl SegmentGenerationCell {
    pub(crate) const fn new(segment_id: PhysicalSegmentId, generation: PhysicalGeneration) -> Self {
        Self {
            segment_id,
            generation,
        }
    }

    pub const fn segment_id(self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_segment(self.segment_id, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentGenerationCellBuilder {
    segment_id: PhysicalSegmentId,
}

impl SegmentGenerationCellBuilder {
    pub(crate) const fn new(segment_id: PhysicalSegmentId) -> Self {
        Self { segment_id }
    }

    pub const fn with_segment_generation(
        self,
        generation: PhysicalGeneration,
    ) -> SegmentGenerationCell {
        SegmentGenerationCell::new(self.segment_id, generation)
    }
}
