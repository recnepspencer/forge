use crate::{
    AllocationClassKind, FreeSpaceReuseAddress, PhysicalExtentId, PhysicalGeneration,
    PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalCellReuseDomain {
    SlotAllocation,
    ExtentAllocation,
    FreeSpaceReuse,
    RootPublication,
    Page,
    Segment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn stable_fingerprint(self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-physical-generation-owner-v1");
        digest.update([
            domain_tag(self.domain),
            allocation_tag(self.allocation_class),
        ]);
        digest.update(
            self.segment_id
                .map_or(0, PhysicalSegmentId::get)
                .to_be_bytes(),
        );
        digest.update(self.page_id.map_or(0, PhysicalPageId::get).to_be_bytes());
        digest.update(
            self.extent_id
                .map_or(0, PhysicalExtentId::get)
                .to_be_bytes(),
        );
        digest.update(self.slot.map_or(0, PhysicalRecordSlot::get).to_be_bytes());
        digest.update(
            self.root_reference
                .map_or(0, PhysicalRootReference::get)
                .to_be_bytes(),
        );
        digest.update(self.generation.get().to_be_bytes());
        digest.finalize().into()
    }
}

const fn domain_tag(domain: PhysicalCellReuseDomain) -> u8 {
    match domain {
        PhysicalCellReuseDomain::SlotAllocation => 1,
        PhysicalCellReuseDomain::ExtentAllocation => 2,
        PhysicalCellReuseDomain::FreeSpaceReuse => 3,
        PhysicalCellReuseDomain::RootPublication => 4,
        PhysicalCellReuseDomain::Page => 5,
        PhysicalCellReuseDomain::Segment => 6,
    }
}

const fn allocation_tag(class: Option<AllocationClassKind>) -> u8 {
    match class {
        None => 0,
        Some(AllocationClassKind::OrdinaryRecordPage) => 1,
        Some(AllocationClassKind::LargeRecordExtent) => 2,
        Some(AllocationClassKind::RootManifest) => 3,
        Some(AllocationClassKind::SegmentManifest) => 4,
        Some(AllocationClassKind::ExtentManifest) => 5,
        Some(AllocationClassKind::FreeSpaceMap) => 6,
    }
}
