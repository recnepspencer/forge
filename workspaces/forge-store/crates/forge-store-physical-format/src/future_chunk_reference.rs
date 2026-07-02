use crate::{PhysicalGeneration, PhysicalVocabularyError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalFutureChunkId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalFutureChunkReference {
    chunk_id: PhysicalFutureChunkId,
    generation: PhysicalGeneration,
}

impl PhysicalFutureChunkId {
    pub fn from_raw(value: u64) -> Result<Self, PhysicalVocabularyError> {
        if value == 0 {
            return Err(PhysicalVocabularyError::ZeroPhysicalIdentifier);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl PhysicalFutureChunkReference {
    pub const fn stability_placeholder(
        chunk_id: PhysicalFutureChunkId,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            chunk_id,
            generation,
        }
    }

    pub const fn chunk_id(self) -> PhysicalFutureChunkId {
        self.chunk_id
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.generation
    }
}
