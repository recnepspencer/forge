use forge_store_physical_isolation::ChunkMigrationReadInterlockPlan;

use crate::{
    AuthenticatedFrameDigest, BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId,
    BlobPlacementClass, ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};

use super::basis::BlobPlacementMovementBasis;
use crate::placement::movement::counters::BlobPlacementMovementCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementExecution {
    pub(crate) _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementPublication {
    _private: (),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreOwnedPlacementMovementExecutionReceipt {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) source_class: BlobPlacementClass,
    pub(crate) target_class: BlobPlacementClass,
    pub(crate) movement_interlock: ChunkMigrationReadInterlockPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedBlobPlacementMovementReceipt {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) source_class: BlobPlacementClass,
    pub(crate) target_class: BlobPlacementClass,
    pub(crate) counters: BlobPlacementMovementCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBlobPlacementObservation {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) placement_class: BlobPlacementClass,
    pub(crate) counters: BlobPlacementMovementCounterSnapshot,
}

impl StoreOwnedPlacementMovementPublication {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }
}

impl ExecutedBlobPlacementMovementReceipt {
    pub const fn object_id(&self) -> &BlobObjectId {
        self.basis.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.basis.generation()
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        self.basis.chunk_tree_root()
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        self.basis.logical_content_digest()
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.basis.stored_digest()
    }

    pub const fn authenticated_frame_digest(&self) -> &AuthenticatedFrameDigest {
        self.basis.authenticated_frame_digest()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.basis.security_metadata()
    }

    pub const fn source_class(&self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(&self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }
}

impl PublishedBlobPlacementObservation {
    pub const fn object_id(&self) -> &BlobObjectId {
        self.basis.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.basis.generation()
    }

    pub const fn placement_class(&self) -> BlobPlacementClass {
        self.placement_class
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.basis.stored_digest()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.basis.security_metadata()
    }

    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }
}