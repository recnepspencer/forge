use worth_store_physical_format::{
    AllocationClassKind, PhysicalCellReuseDomain, PhysicalGenerationOwner,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BackupProtectedPhysicalOwner {
    pub(super) domain: PhysicalCellReuseDomain,
    pub(super) segment: Option<u64>,
    pub(super) page: Option<u64>,
    pub(super) extent: Option<u64>,
    pub(super) slot: Option<u64>,
    pub(super) root: Option<u64>,
    pub(super) allocation: Option<AllocationClassKind>,
    pub(super) generation: u64,
}

impl BackupProtectedPhysicalOwner {
    pub(crate) fn from_owner(owner: PhysicalGenerationOwner) -> Self {
        Self {
            domain: owner.domain(),
            segment: owner.segment_id().map(|value| value.get()),
            page: owner.page_id().map(|value| value.get()),
            extent: owner.extent_id().map(|value| value.get()),
            slot: owner.slot().map(|value| u64::from(value.get())),
            root: owner.root_reference().map(|value| value.get()),
            allocation: owner.allocation_class(),
            generation: owner.generation().get(),
        }
    }

    pub(super) fn is_valid(self) -> bool {
        if self.generation == 0 {
            return false;
        }
        match self.domain {
            PhysicalCellReuseDomain::SlotAllocation => {
                self.segment.is_some()
                    && self.page.is_some()
                    && self.slot.is_some()
                    && self.extent.is_none()
                    && self.root.is_none()
                    && self.allocation.is_none()
            }
            PhysicalCellReuseDomain::ExtentAllocation => {
                self.segment.is_some()
                    && self.extent.is_some()
                    && self.page.is_none()
                    && self.slot.is_none()
                    && self.root.is_none()
                    && self.allocation.is_none()
            }
            PhysicalCellReuseDomain::FreeSpaceReuse => {
                self.segment.is_some()
                    && self.root.is_none()
                    && self.allocation.is_some()
                    && ((self.page.is_some() && self.slot.is_some() && self.extent.is_none())
                        || (self.extent.is_some() && self.page.is_none() && self.slot.is_none()))
            }
            PhysicalCellReuseDomain::RootPublication => {
                self.root.is_some()
                    && self.segment.is_none()
                    && self.page.is_none()
                    && self.extent.is_none()
                    && self.slot.is_none()
                    && self.allocation.is_none()
            }
            PhysicalCellReuseDomain::Page => {
                self.segment.is_some()
                    && self.page.is_some()
                    && self.extent.is_none()
                    && self.slot.is_none()
                    && self.root.is_none()
                    && self.allocation.is_none()
            }
            PhysicalCellReuseDomain::Segment => {
                self.segment.is_some()
                    && self.page.is_none()
                    && self.extent.is_none()
                    && self.slot.is_none()
                    && self.root.is_none()
                    && self.allocation.is_none()
            }
        }
    }
}

pub(super) const fn domain_tag(domain: PhysicalCellReuseDomain) -> u8 {
    match domain {
        PhysicalCellReuseDomain::SlotAllocation => 1,
        PhysicalCellReuseDomain::ExtentAllocation => 2,
        PhysicalCellReuseDomain::FreeSpaceReuse => 3,
        PhysicalCellReuseDomain::RootPublication => 4,
        PhysicalCellReuseDomain::Page => 5,
        PhysicalCellReuseDomain::Segment => 6,
    }
}

pub(super) fn domain_from_tag(tag: u8) -> Option<PhysicalCellReuseDomain> {
    match tag {
        1 => Some(PhysicalCellReuseDomain::SlotAllocation),
        2 => Some(PhysicalCellReuseDomain::ExtentAllocation),
        3 => Some(PhysicalCellReuseDomain::FreeSpaceReuse),
        4 => Some(PhysicalCellReuseDomain::RootPublication),
        5 => Some(PhysicalCellReuseDomain::Page),
        6 => Some(PhysicalCellReuseDomain::Segment),
        _ => None,
    }
}

pub(super) const fn allocation_tag(allocation: Option<AllocationClassKind>) -> u8 {
    match allocation {
        None => 0,
        Some(AllocationClassKind::OrdinaryRecordPage) => 1,
        Some(AllocationClassKind::LargeRecordExtent) => 2,
        Some(AllocationClassKind::RootManifest) => 3,
        Some(AllocationClassKind::SegmentManifest) => 4,
        Some(AllocationClassKind::ExtentManifest) => 5,
        Some(AllocationClassKind::FreeSpaceMap) => 6,
    }
}

pub(super) fn allocation_from_tag(tag: u8) -> Option<Option<AllocationClassKind>> {
    match tag {
        0 => Some(None),
        1 => Some(Some(AllocationClassKind::OrdinaryRecordPage)),
        2 => Some(Some(AllocationClassKind::LargeRecordExtent)),
        3 => Some(Some(AllocationClassKind::RootManifest)),
        4 => Some(Some(AllocationClassKind::SegmentManifest)),
        5 => Some(Some(AllocationClassKind::ExtentManifest)),
        6 => Some(Some(AllocationClassKind::FreeSpaceMap)),
        _ => None,
    }
}
