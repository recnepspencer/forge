use worth_store::physical_runtime::{
    LifecycleGeneration, PhysicalRecordChunkBasis, PhysicalRecordId, RuntimeIdentity,
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryBasis {
    chunk: PhysicalRecordChunkBasis,
    verification_runtime: RuntimeIdentity,
    verification_bytes: u64,
}

impl IntegrityEntryBasis {
    pub(crate) const fn from_store_authority(
        chunk: PhysicalRecordChunkBasis,
        verification_runtime: RuntimeIdentity,
        verification_bytes: u64,
    ) -> Self {
        Self {
            chunk,
            verification_runtime,
            verification_bytes,
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.chunk.store_identity()
    }

    pub const fn store_generation(self) -> LifecycleGeneration {
        self.chunk.store_generation()
    }

    pub const fn record(self) -> PhysicalRecordId {
        self.chunk.record()
    }

    pub const fn frame_coordinate(self) -> RecordFrameCoordinate {
        self.chunk.frame_coordinate()
    }

    pub const fn verification_runtime(self) -> RuntimeIdentity {
        self.verification_runtime
    }

    pub const fn verification_bytes(self) -> u64 {
        self.verification_bytes
    }
}
