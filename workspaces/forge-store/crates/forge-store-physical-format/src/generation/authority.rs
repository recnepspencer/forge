use crate::{
    AllocationClassKind, ExtentGenerationCellBuilder, FreeSpaceReuseAddress,
    FreeSpaceReuseCellBuilder, PageGenerationCellBuilder, PhysicalExtentId, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId, PhysicalVocabularyError,
    RootPublicationCellBuilder, SegmentGenerationCellBuilder, SlotGenerationCellBuilder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalGenerationAuthority {
    scope: PhysicalGenerationAuthorityScope,
}

impl PhysicalGenerationAuthority {
    pub const fn s1() -> Self {
        Self {
            scope: PhysicalGenerationAuthorityScope::StorageFoundationS1,
        }
    }

    pub const fn scope(self) -> PhysicalGenerationAuthorityScope {
        self.scope
    }

    pub const fn slot_cell(
        self,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
    ) -> SlotGenerationCellBuilder {
        SlotGenerationCellBuilder::new(segment_id, page_id, slot)
    }

    pub const fn extent_cell(
        self,
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
    ) -> ExtentGenerationCellBuilder {
        ExtentGenerationCellBuilder::new(segment_id, extent_id)
    }

    pub fn free_space_slot_cell(
        self,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot: PhysicalRecordSlot,
        allocation_class: AllocationClassKind,
    ) -> Result<FreeSpaceReuseCellBuilder, PhysicalVocabularyError> {
        if !matches!(allocation_class, AllocationClassKind::OrdinaryRecordPage) {
            return Err(PhysicalVocabularyError::InvalidFreeSpaceReuseAllocationClass);
        }
        Ok(FreeSpaceReuseCellBuilder::new(
            FreeSpaceReuseAddress::PageSlot {
                segment_id,
                page_id,
                slot,
            },
            allocation_class,
        ))
    }

    pub fn free_space_extent_cell(
        self,
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
        allocation_class: AllocationClassKind,
    ) -> Result<FreeSpaceReuseCellBuilder, PhysicalVocabularyError> {
        if !matches!(allocation_class, AllocationClassKind::LargeRecordExtent) {
            return Err(PhysicalVocabularyError::InvalidFreeSpaceReuseAllocationClass);
        }
        Ok(FreeSpaceReuseCellBuilder::new(
            FreeSpaceReuseAddress::Extent {
                segment_id,
                extent_id,
            },
            allocation_class,
        ))
    }

    pub const fn root_publication_cell(
        self,
        root_reference: PhysicalRootReference,
    ) -> RootPublicationCellBuilder {
        RootPublicationCellBuilder::new(root_reference)
    }

    pub const fn page_cell(
        self,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    ) -> PageGenerationCellBuilder {
        PageGenerationCellBuilder::new(segment_id, page_id)
    }

    pub const fn segment_cell(self, segment_id: PhysicalSegmentId) -> SegmentGenerationCellBuilder {
        SegmentGenerationCellBuilder::new(segment_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalGenerationAuthorityScope {
    StorageFoundationS1,
}
