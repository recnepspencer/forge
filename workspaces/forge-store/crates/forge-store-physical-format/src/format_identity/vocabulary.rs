use crate::{
    AllocationClassKind, FreeSpaceMapVocabulary, PhysicalEpoch, PhysicalExtentId, PhysicalFrameId,
    PhysicalGeneration, PhysicalPageId, PhysicalRecordSlot, PhysicalRootReference,
    PhysicalSegmentId, PhysicalVocabularyError,
};
use forge_store_contracts::{PhysicalAuthorityScope, StorePhysicalAuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalVocabularyTerm {
    Segment,
    Page,
    Extent,
    Frame,
    Slot,
    Generation,
    Epoch,
    RootReference,
    AllocationClass,
    FreeSpaceMap,
    Manifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalFormatVocabulary {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    extent_id: PhysicalExtentId,
    frame_id: PhysicalFrameId,
    slot: PhysicalRecordSlot,
    generation: PhysicalGeneration,
    epoch: PhysicalEpoch,
    root_reference: PhysicalRootReference,
    allocation_class: AllocationClassKind,
    free_space_map: FreeSpaceMapVocabulary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFormatVocabularyDeclaration {
    segment_id: Option<PhysicalSegmentId>,
    page_id: Option<PhysicalPageId>,
    extent_id: Option<PhysicalExtentId>,
    frame_id: Option<PhysicalFrameId>,
    slot: Option<PhysicalRecordSlot>,
    generation: Option<PhysicalGeneration>,
    epoch: Option<PhysicalEpoch>,
    root_reference: Option<PhysicalRootReference>,
    allocation_class: Option<AllocationClassKind>,
    free_space_map: Option<FreeSpaceMapVocabulary>,
}

impl PhysicalFormatVocabulary {
    pub const fn declare() -> PhysicalFormatVocabularyDeclaration {
        PhysicalFormatVocabularyDeclaration {
            segment_id: None,
            page_id: None,
            extent_id: None,
            frame_id: None,
            slot: None,
            generation: None,
            epoch: None,
            root_reference: None,
            allocation_class: None,
            free_space_map: None,
        }
    }

    pub const fn segment_id(&self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_id(&self) -> PhysicalPageId {
        self.page_id
    }

    pub const fn extent_id(&self) -> PhysicalExtentId {
        self.extent_id
    }

    pub const fn frame_id(&self) -> PhysicalFrameId {
        self.frame_id
    }

    pub const fn slot(&self) -> PhysicalRecordSlot {
        self.slot
    }

    pub const fn generation(&self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn epoch(&self) -> PhysicalEpoch {
        self.epoch
    }

    pub const fn root_reference(&self) -> PhysicalRootReference {
        self.root_reference
    }

    pub const fn allocation_class(&self) -> AllocationClassKind {
        self.allocation_class
    }

    pub const fn free_space_map(&self) -> FreeSpaceMapVocabulary {
        self.free_space_map
    }
}

impl PhysicalFormatVocabularyDeclaration {
    pub const fn with_segment_id(mut self, segment_id: PhysicalSegmentId) -> Self {
        self.segment_id = Some(segment_id);
        self
    }

    pub const fn with_page_id(mut self, page_id: PhysicalPageId) -> Self {
        self.page_id = Some(page_id);
        self
    }

    pub const fn with_extent_id(mut self, extent_id: PhysicalExtentId) -> Self {
        self.extent_id = Some(extent_id);
        self
    }

    pub const fn with_frame_id(mut self, frame_id: PhysicalFrameId) -> Self {
        self.frame_id = Some(frame_id);
        self
    }

    pub const fn with_slot(mut self, slot: PhysicalRecordSlot) -> Self {
        self.slot = Some(slot);
        self
    }

    pub const fn with_generation(mut self, generation: PhysicalGeneration) -> Self {
        self.generation = Some(generation);
        self
    }

    pub const fn with_epoch(mut self, epoch: PhysicalEpoch) -> Self {
        self.epoch = Some(epoch);
        self
    }

    pub const fn with_root_reference(mut self, root_reference: PhysicalRootReference) -> Self {
        self.root_reference = Some(root_reference);
        self
    }

    pub const fn with_allocation_class(mut self, allocation_class: AllocationClassKind) -> Self {
        self.allocation_class = Some(allocation_class);
        self
    }

    pub const fn with_free_space_map(mut self, free_space_map: FreeSpaceMapVocabulary) -> Self {
        self.free_space_map = Some(free_space_map);
        self
    }

    pub fn admit(
        self,
        authority: StorePhysicalAuthorityWitness,
    ) -> Result<PhysicalFormatVocabulary, PhysicalVocabularyError> {
        if authority.authority_scope() != PhysicalAuthorityScope::PhysicalFoundationVocabulary {
            return Err(PhysicalVocabularyError::WrongAuthorityScope);
        }

        Ok(PhysicalFormatVocabulary {
            segment_id: Self::required(self.segment_id)?,
            page_id: Self::required(self.page_id)?,
            extent_id: Self::required(self.extent_id)?,
            frame_id: Self::required(self.frame_id)?,
            slot: Self::required(self.slot)?,
            generation: Self::required(self.generation)?,
            epoch: Self::required(self.epoch)?,
            root_reference: Self::required(self.root_reference)?,
            allocation_class: Self::required(self.allocation_class)?,
            free_space_map: Self::required(self.free_space_map)?,
        })
    }

    fn required<T>(value: Option<T>) -> Result<T, PhysicalVocabularyError> {
        value.ok_or(PhysicalVocabularyError::MissingVocabularyTerm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_contracts::StorePhysicalAuthorityWitness;

    #[test]
    fn physical_vocabulary_admits_all_required_terms() {
        let authority = StorePhysicalAuthorityWitness::for_physical_format_vocabulary(
            forge_store_contracts::ROADMAP_2_S1_SCOPE,
        )
        .expect("S.1 physical vocabulary authority");
        let vocabulary = PhysicalFormatVocabulary::declare()
            .with_segment_id(PhysicalSegmentId::from_raw(1).expect("segment id"))
            .with_page_id(PhysicalPageId::from_raw(2).expect("page id"))
            .with_extent_id(PhysicalExtentId::from_raw(3).expect("extent id"))
            .with_frame_id(PhysicalFrameId::from_raw(4).expect("frame id"))
            .with_slot(PhysicalRecordSlot::from_raw(8).expect("slot"))
            .with_generation(PhysicalGeneration::from_raw(5).expect("generation"))
            .with_epoch(PhysicalEpoch::from_raw(6).expect("epoch"))
            .with_root_reference(PhysicalRootReference::from_raw(7).expect("root reference"))
            .with_allocation_class(AllocationClassKind::OrdinaryRecordPage)
            .with_free_space_map(FreeSpaceMapVocabulary::for_free_space_map())
            .admit(authority)
            .expect("complete vocabulary is accepted");

        assert_eq!(vocabulary.page_id().get(), 2);
        assert_eq!(
            vocabulary.free_space_map().allocation_class(),
            AllocationClassKind::FreeSpaceMap
        );
    }

    #[test]
    fn zero_raw_identifier_is_rejected() {
        let denial = PhysicalPageId::from_raw(0).expect_err("zero ids are rejected");

        assert_eq!(denial, PhysicalVocabularyError::ZeroPhysicalIdentifier);
    }

    #[test]
    fn missing_free_space_map_vocabulary_is_rejected() {
        let authority = StorePhysicalAuthorityWitness::for_physical_format_vocabulary(
            forge_store_contracts::ROADMAP_2_S1_SCOPE,
        )
        .expect("S.1 physical vocabulary authority");
        let denial = PhysicalFormatVocabulary::declare()
            .with_segment_id(PhysicalSegmentId::from_raw(1).expect("segment id"))
            .with_page_id(PhysicalPageId::from_raw(2).expect("page id"))
            .with_extent_id(PhysicalExtentId::from_raw(3).expect("extent id"))
            .with_frame_id(PhysicalFrameId::from_raw(4).expect("frame id"))
            .with_slot(PhysicalRecordSlot::from_raw(8).expect("slot"))
            .with_generation(PhysicalGeneration::from_raw(5).expect("generation"))
            .with_epoch(PhysicalEpoch::from_raw(6).expect("epoch"))
            .with_root_reference(PhysicalRootReference::from_raw(7).expect("root reference"))
            .with_allocation_class(AllocationClassKind::OrdinaryRecordPage)
            .admit(authority)
            .expect_err("free-space vocabulary is mandatory");

        assert_eq!(denial, PhysicalVocabularyError::MissingVocabularyTerm);
    }

    #[test]
    fn free_space_map_vocabulary_names_only_free_space_map_allocation() {
        assert_eq!(
            FreeSpaceMapVocabulary::for_free_space_map().allocation_class(),
            AllocationClassKind::FreeSpaceMap
        );
    }
}
