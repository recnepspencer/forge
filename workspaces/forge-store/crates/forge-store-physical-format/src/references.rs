use crate::{
    AllocationClassKind, ExtentGenerationCell, FreeSpaceReuseAddress, FreeSpaceReuseCell,
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationOwner, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId, RootPublicationCell,
    SlotGenerationCell,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReferenceKind {
    PageSlot,
    ExtentBacked,
    FreeSpaceReuse,
    RootPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReference {
    segment_id: Option<PhysicalSegmentId>,
    page_id: Option<PhysicalPageId>,
    extent_id: Option<PhysicalExtentId>,
    slot: Option<PhysicalRecordSlot>,
    root_reference: Option<PhysicalRootReference>,
    allocation_class: Option<AllocationClassKind>,
    generation: PhysicalGeneration,
    kind: PhysicalReferenceKind,
}

impl PhysicalReference {
    pub(crate) const fn from_slot_cell(cell: SlotGenerationCell) -> Self {
        Self {
            segment_id: Some(cell.segment_id()),
            page_id: Some(cell.page_id()),
            extent_id: None,
            slot: Some(cell.slot()),
            root_reference: None,
            allocation_class: None,
            generation: cell.generation(),
            kind: PhysicalReferenceKind::PageSlot,
        }
    }

    pub(crate) const fn from_extent_cell(cell: ExtentGenerationCell) -> Self {
        Self {
            segment_id: Some(cell.segment_id()),
            page_id: None,
            extent_id: Some(cell.extent_id()),
            slot: None,
            root_reference: None,
            allocation_class: None,
            generation: cell.generation(),
            kind: PhysicalReferenceKind::ExtentBacked,
        }
    }

    pub(crate) const fn from_free_space_cell(cell: FreeSpaceReuseCell) -> Self {
        match cell.address() {
            FreeSpaceReuseAddress::PageSlot {
                segment_id,
                page_id,
                slot,
            } => Self {
                segment_id: Some(segment_id),
                page_id: Some(page_id),
                extent_id: None,
                slot: Some(slot),
                root_reference: None,
                allocation_class: Some(cell.allocation_class()),
                generation: cell.generation(),
                kind: PhysicalReferenceKind::FreeSpaceReuse,
            },
            FreeSpaceReuseAddress::Extent {
                segment_id,
                extent_id,
            } => Self {
                segment_id: Some(segment_id),
                page_id: None,
                extent_id: Some(extent_id),
                slot: None,
                root_reference: None,
                allocation_class: Some(cell.allocation_class()),
                generation: cell.generation(),
                kind: PhysicalReferenceKind::FreeSpaceReuse,
            },
        }
    }

    pub(crate) const fn from_root_publication_cell(cell: RootPublicationCell) -> Self {
        Self {
            segment_id: None,
            page_id: None,
            extent_id: None,
            slot: None,
            root_reference: Some(cell.root_reference()),
            allocation_class: None,
            generation: cell.generation(),
            kind: PhysicalReferenceKind::RootPublication,
        }
    }

    pub const fn kind(&self) -> PhysicalReferenceKind {
        self.kind
    }

    pub const fn segment_id(&self) -> Option<PhysicalSegmentId> {
        self.segment_id
    }

    pub const fn page_id(&self) -> Option<PhysicalPageId> {
        self.page_id
    }

    pub const fn extent_id(&self) -> Option<PhysicalExtentId> {
        self.extent_id
    }

    pub const fn slot(&self) -> Option<PhysicalRecordSlot> {
        self.slot
    }

    pub const fn root_reference(&self) -> Option<PhysicalRootReference> {
        self.root_reference
    }

    pub const fn allocation_class(&self) -> Option<AllocationClassKind> {
        self.allocation_class
    }

    pub const fn generation(&self) -> PhysicalGeneration {
        self.generation
    }

    pub fn generation_owner(&self) -> PhysicalGenerationOwner {
        match self.kind {
            PhysicalReferenceKind::PageSlot => PhysicalGenerationOwner::for_slot(
                self.segment_id.expect("sealed reference has segment"),
                self.page_id.expect("sealed page-slot reference has page"),
                self.slot.expect("sealed page-slot reference has slot"),
                self.generation,
            ),
            PhysicalReferenceKind::ExtentBacked => PhysicalGenerationOwner::for_extent(
                self.segment_id.expect("sealed reference has segment"),
                self.extent_id.expect("sealed extent reference has extent"),
                self.generation,
            ),
            PhysicalReferenceKind::FreeSpaceReuse => PhysicalGenerationOwner::for_free_space(
                self.free_space_address(),
                self.allocation_class
                    .expect("sealed free-space reference has allocation class"),
                self.generation,
            ),
            PhysicalReferenceKind::RootPublication => {
                PhysicalGenerationOwner::for_root_publication(
                    self.root_reference
                        .expect("sealed root publication reference has root reference"),
                    self.generation,
                )
            }
        }
    }

    fn free_space_address(&self) -> FreeSpaceReuseAddress {
        if let Some(page_id) = self.page_id {
            return FreeSpaceReuseAddress::PageSlot {
                segment_id: self.segment_id.expect("sealed reference has segment"),
                page_id,
                slot: self
                    .slot
                    .expect("sealed free-space slot reference has slot"),
            };
        }
        FreeSpaceReuseAddress::Extent {
            segment_id: self.segment_id.expect("sealed reference has segment"),
            extent_id: self
                .extent_id
                .expect("sealed free-space extent reference has extent"),
        }
    }
}
