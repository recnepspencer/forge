use sha2::{Digest, Sha256};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::ArtifactTreeFailure;
use crate::{
    filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity},
    BackendQueueExecutionCompletion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRangeWriteDurability {
    BufferedWriteCompleted,
    FileDataSynchronized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRangeWriteDurabilityRequirement {
    BufferedWrite,
    FileDataSynchronization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedArtifactRangeWrite {
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    completed_bytes: u64,
    operation: MediaOperationIdentity,
    durability: ArtifactRangeWriteDurability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateArtifactRangeWrite {
    failure: ArtifactTreeFailure,
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    completed_bytes: u64,
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactRangeWriteOutcome {
    Completed(CompletedArtifactRangeWrite),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactRangeWrite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledArtifactRangeWrite {
    pub(super) physical: CompletedArtifactRangeWrite,
    pub(super) queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactRangeWriteOutcome {
    Completed(Box<CompletedScheduledArtifactRangeWrite>),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactRangeWrite),
}

impl CompletedArtifactRangeWrite {
    pub(super) fn buffered(
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            owner,
            store,
            coordinate,
            payload_digest: Sha256::digest(bytes).into(),
            completed_bytes: bytes.len() as u64,
            operation,
            durability: ArtifactRangeWriteDurability::BufferedWriteCompleted,
        }
    }

    pub(super) fn set_durability(&mut self, durability: ArtifactRangeWriteDurability) {
        self.durability = durability;
    }

    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn durability(&self) -> ArtifactRangeWriteDurability {
        self.durability
    }
}

impl IndeterminateArtifactRangeWrite {
    pub(super) fn new(
        failure: ArtifactTreeFailure,
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        coordinate: RecordFrameCoordinate,
        bytes: &[u8],
        completed_bytes: u64,
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            failure,
            owner,
            store,
            coordinate,
            payload_digest: Sha256::digest(bytes).into(),
            completed_bytes,
            operation,
        }
    }

    pub const fn failure(self) -> ArtifactTreeFailure {
        self.failure
    }

    pub const fn owner(self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn payload_digest(self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn completed_bytes(self) -> u64 {
        self.completed_bytes
    }

    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }
}

impl CompletedScheduledArtifactRangeWrite {
    pub const fn physical(&self) -> &CompletedArtifactRangeWrite {
        &self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}
