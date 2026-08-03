use crate::{PhysicalExtentId, PhysicalGeneration, PhysicalGenerationOwner};

/// Generation authority for an extent that is a top-level record artifact.
///
/// Older physical extents may be segment-owned. C.5 record extents are not, so
/// their generation cell must not invent a segment coordinate merely to reuse
/// the older cell shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordExtentGenerationCell {
    extent_id: PhysicalExtentId,
    generation: PhysicalGeneration,
}

impl RecordExtentGenerationCell {
    pub(crate) const fn new(extent_id: PhysicalExtentId, generation: PhysicalGeneration) -> Self {
        Self {
            extent_id,
            generation,
        }
    }

    pub const fn extent_id(self) -> PhysicalExtentId {
        self.extent_id
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        PhysicalGenerationOwner::for_record_extent(self.extent_id, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordExtentGenerationCellBuilder {
    extent_id: PhysicalExtentId,
}

impl RecordExtentGenerationCellBuilder {
    pub(crate) const fn new(extent_id: PhysicalExtentId) -> Self {
        Self { extent_id }
    }

    pub const fn with_extent_generation(
        self,
        generation: PhysicalGeneration,
    ) -> RecordExtentGenerationCell {
        RecordExtentGenerationCell::new(self.extent_id, generation)
    }
}
