use crate::CurrentGenerationPhysicalReference;
use worth_store::physical_runtime::{PhysicalRecordChunkBasis, PhysicalRecordChunkView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalByteGuardScope {
    reference: CurrentGenerationPhysicalReference,
    chunk_basis: PhysicalRecordChunkBasis,
}

impl PhysicalByteGuardScope {
    pub fn for_record_chunk(chunk: &PhysicalRecordChunkView<'_>) -> Self {
        Self {
            reference: CurrentGenerationPhysicalReference::for_record_chunk(chunk),
            chunk_basis: chunk.basis(),
        }
    }

    pub const fn reference(self) -> CurrentGenerationPhysicalReference {
        self.reference
    }

    pub const fn chunk_basis(self) -> PhysicalRecordChunkBasis {
        self.chunk_basis
    }
}
