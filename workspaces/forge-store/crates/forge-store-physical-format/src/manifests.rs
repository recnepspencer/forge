use crate::{AllocationClassKind, PhysicalExtentId, PhysicalRootReference, PhysicalSegmentId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ManifestVocabularyKind {
    PhysicalRoot,
    Segment,
    Extent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRootManifestVocabulary {
    root_reference: PhysicalRootReference,
}

impl PhysicalRootManifestVocabulary {
    pub const fn new(root_reference: PhysicalRootReference) -> Self {
        Self { root_reference }
    }

    pub const fn root_reference(&self) -> PhysicalRootReference {
        self.root_reference
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentManifestVocabulary {
    segment_id: PhysicalSegmentId,
    allocation_class: AllocationClassKind,
}

impl SegmentManifestVocabulary {
    pub const fn new(segment_id: PhysicalSegmentId, allocation_class: AllocationClassKind) -> Self {
        Self {
            segment_id,
            allocation_class,
        }
    }

    pub const fn segment_id(&self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn allocation_class(&self) -> AllocationClassKind {
        self.allocation_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentManifestVocabulary {
    extent_id: PhysicalExtentId,
    allocation_class: AllocationClassKind,
}

impl ExtentManifestVocabulary {
    pub const fn new(extent_id: PhysicalExtentId, allocation_class: AllocationClassKind) -> Self {
        Self {
            extent_id,
            allocation_class,
        }
    }

    pub const fn extent_id(&self) -> PhysicalExtentId {
        self.extent_id
    }

    pub const fn allocation_class(&self) -> AllocationClassKind {
        self.allocation_class
    }
}
