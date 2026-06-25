use crate::{
    AllocationClassKind, FreeSpaceReuseAddress, PhysicalExtentId, PhysicalGeneration,
    PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalCellReuseDomain {
    SlotAllocation,
    ExtentAllocation,
    FreeSpaceReuse,
    RootPublication,
    Page,
    Segment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalGenerationOwner {
    domain: PhysicalCellReuseDomain,
    segment_id: Option<PhysicalSegmentId>,
    page_id: Option<PhysicalPageId>,
    extent_id: Option<PhysicalExtentId>,
    slot: Option<PhysicalRecordSlot>,
    root_reference: Option<PhysicalRootReference>,
    allocation_class: Option<AllocationClassKind>,
    generation: PhysicalGeneration,
}

impl PhysicalGenerationOwner {
    pub(crate) const fn for_slot(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            domain: PhysicalCellReuseDomain::SlotAllocation,
            segment_id: Some(segment_id),
            page_id: Some(page_id),
            extent_id: None,
            slot: Some(slot),
            root_reference: None,
            allocation_class: None,
            generation,
        }
    }

    pub(crate) const fn for_extent(
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            domain: PhysicalCellReuseDomain::ExtentAllocation,
            segment_id: Some(segment_id),
            page_id: None,
            extent_id: Some(extent_id),
            slot: None,
            root_reference: None,
            allocation_class: None,
            generation,
        }
    }

    pub(crate) const fn for_free_space(
        address: FreeSpaceReuseAddress,
        allocation_class: AllocationClassKind,
        generation: PhysicalGeneration,
    ) -> Self {
        match address {
            FreeSpaceReuseAddress::PageSlot {
                segment_id,
                page_id,
                slot,
            } => Self {
                domain: PhysicalCellReuseDomain::FreeSpaceReuse,
                segment_id: Some(segment_id),
                page_id: Some(page_id),
                extent_id: None,
                slot: Some(slot),
                root_reference: None,
                allocation_class: Some(allocation_class),
                generation,
            },
            FreeSpaceReuseAddress::Extent {
                segment_id,
                extent_id,
            } => Self {
                domain: PhysicalCellReuseDomain::FreeSpaceReuse,
                segment_id: Some(segment_id),
                page_id: None,
                extent_id: Some(extent_id),
                slot: None,
                root_reference: None,
                allocation_class: Some(allocation_class),
                generation,
            },
        }
    }

    pub(crate) const fn for_root_publication(
        root_reference: PhysicalRootReference,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            domain: PhysicalCellReuseDomain::RootPublication,
            segment_id: None,
            page_id: None,
            extent_id: None,
            slot: None,
            root_reference: Some(root_reference),
            allocation_class: None,
            generation,
        }
    }

    pub(crate) const fn for_page(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            domain: PhysicalCellReuseDomain::Page,
            segment_id: Some(segment_id),
            page_id: Some(page_id),
            extent_id: None,
            slot: None,
            root_reference: None,
            allocation_class: None,
            generation,
        }
    }

    pub(crate) const fn for_segment(
        segment_id: PhysicalSegmentId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            domain: PhysicalCellReuseDomain::Segment,
            segment_id: Some(segment_id),
            page_id: None,
            extent_id: None,
            slot: None,
            root_reference: None,
            allocation_class: None,
            generation,
        }
    }

    pub const fn domain(self) -> PhysicalCellReuseDomain {
        self.domain
    }

    pub const fn segment_id(self) -> Option<PhysicalSegmentId> {
        self.segment_id
    }

    pub const fn page_id(self) -> Option<PhysicalPageId> {
        self.page_id
    }

    pub const fn extent_id(self) -> Option<PhysicalExtentId> {
        self.extent_id
    }

    pub const fn slot(self) -> Option<PhysicalRecordSlot> {
        self.slot
    }

    pub const fn root_reference(self) -> Option<PhysicalRootReference> {
        self.root_reference
    }

    pub const fn allocation_class(self) -> Option<AllocationClassKind> {
        self.allocation_class
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }
}
