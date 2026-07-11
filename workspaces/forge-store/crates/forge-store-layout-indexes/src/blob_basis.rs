use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BlobGenerationBasis {
    sequence: u64,
}

impl S8BlobGenerationBasis {
    pub const fn from_sequence(sequence: u64) -> Self {
        Self { sequence }
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8BlobIdentityKeyBasis {
    object_digest: StableDigest,
    generation: S8BlobGenerationBasis,
}

impl S8BlobIdentityKeyBasis {
    pub const fn new(object_digest: StableDigest, generation: S8BlobGenerationBasis) -> Self {
        Self {
            object_digest,
            generation,
        }
    }

    pub const fn object_digest(&self) -> &StableDigest {
        &self.object_digest
    }

    pub const fn generation(&self) -> S8BlobGenerationBasis {
        self.generation
    }
}
