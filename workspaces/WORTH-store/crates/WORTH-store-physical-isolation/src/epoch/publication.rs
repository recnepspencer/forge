use worth_store_physical_format::{
    PhysicalCellReuseDomain, PhysicalGenerationOwner, PhysicalReferenceKind,
};

use crate::{
    CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
    GenerationCountedReferenceDenial,
};

use super::{
    chunk_epoch_from_future_publication, extent_epoch_from_publication,
    page_epoch_from_publication, segment_epoch_from_publication, ChunkEpoch, ExtentEpoch,
    PageEpoch, SegmentEpoch,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentPublicationEpochBasis {
    root_scope_id: u64,
    owner: PhysicalGenerationOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentPublicationEpochBasis {
    root_scope_id: u64,
    owner: PhysicalGenerationOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagePublicationEpochBasis {
    root_scope_id: u64,
    owner: PhysicalGenerationOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureChunkPublicationEpochBasis {
    root_scope_id: u64,
}

impl SegmentPublicationEpochBasis {
    pub(crate) fn admit(
        root_scope_id: u64,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<Self, GenerationCountedReferenceDenial> {
        let owner = reference.owner();
        if owner.domain() != PhysicalCellReuseDomain::Segment {
            return Err(GenerationCountedReferenceDenial::WrongPhysicalReferenceKind);
        }
        Ok(Self {
            root_scope_id,
            owner,
        })
    }

    pub fn epoch(self) -> SegmentEpoch {
        segment_epoch_from_publication(self.root_scope_id, publication_seed(self.owner))
    }
}

impl ExtentPublicationEpochBasis {
    pub(crate) fn admit(
        root_scope_id: u64,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<Self, GenerationCountedReferenceDenial> {
        let owner = reference.owner();
        if owner.extent_id().is_none()
            || matches!(
                reference.generation_counted_reference(),
                GenerationCountedPhysicalReference::Segment { .. }
            )
        {
            return Err(GenerationCountedReferenceDenial::WrongPhysicalReferenceKind);
        }
        Ok(Self {
            root_scope_id,
            owner,
        })
    }

    pub fn epoch(self) -> ExtentEpoch {
        extent_epoch_from_publication(self.root_scope_id, publication_seed(self.owner))
    }
}

impl PagePublicationEpochBasis {
    pub(crate) fn admit(
        root_scope_id: u64,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<Self, GenerationCountedReferenceDenial> {
        let owner = reference.owner();
        if !matches!(
            reference.generation_counted_reference(),
            GenerationCountedPhysicalReference::AdmittedReference(inner)
                if inner.kind() == PhysicalReferenceKind::PageSlot
        ) {
            return Err(GenerationCountedReferenceDenial::WrongPhysicalReferenceKind);
        }
        Ok(Self {
            root_scope_id,
            owner,
        })
    }

    pub fn epoch(self) -> PageEpoch {
        page_epoch_from_publication(self.root_scope_id, publication_seed(self.owner))
    }
}

impl FutureChunkPublicationEpochBasis {
    pub(crate) const fn s7_placeholder(root_scope_id: u64) -> Self {
        Self { root_scope_id }
    }

    pub fn epoch(self) -> ChunkEpoch {
        chunk_epoch_from_future_publication(self.root_scope_id, 1)
    }
}

fn publication_seed(owner: PhysicalGenerationOwner) -> u64 {
    let mut seed = owner.generation().get();
    if let Some(segment) = owner.segment_id() {
        seed = seed.wrapping_mul(0x100000001b3) ^ segment.get();
    }
    if let Some(page) = owner.page_id() {
        seed = seed.wrapping_mul(0x100000001b3) ^ page.get();
    }
    if let Some(extent) = owner.extent_id() {
        seed = seed.wrapping_mul(0x100000001b3) ^ extent.get();
    }
    if let Some(slot) = owner.slot() {
        seed = seed.wrapping_mul(0x100000001b3) ^ slot.get() as u64;
    }
    if let Some(root_reference) = owner.root_reference() {
        seed = seed.wrapping_mul(0x100000001b3) ^ root_reference.get();
    }
    if let Some(allocation_class) = owner.allocation_class() {
        seed = seed.wrapping_mul(0x100000001b3) ^ allocation_class as u64;
    }
    seed = seed.wrapping_mul(0x100000001b3) ^ owner.domain() as u64;
    if seed == 0 {
        1
    } else {
        seed
    }
}
