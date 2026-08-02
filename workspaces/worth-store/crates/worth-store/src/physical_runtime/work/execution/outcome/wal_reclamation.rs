use worth_store_physical_backend::{
    ArtifactTreeFailure, MediaOperationIdentity, MediaOperationRole,
};
use worth_store_physical_format::PhysicalCheckpointIdentity;
use worth_store_wal::{WalLsnRange, WalSegmentArtifactIdentity};

pub struct CompletedPhysicalWalReclamationAction {
    checkpoint: PhysicalCheckpointIdentity,
    segment: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
    operation: MediaOperationIdentity,
}

pub(in crate::physical_runtime) struct IndeterminatePhysicalWalReclamationAction {
    checkpoint: PhysicalCheckpointIdentity,
    segment: WalSegmentArtifactIdentity,
    operation: MediaOperationIdentity,
    failure: ArtifactTreeFailure,
}

impl CompletedPhysicalWalReclamationAction {
    pub(in crate::physical_runtime) const fn new(
        checkpoint: PhysicalCheckpointIdentity,
        segment: WalSegmentArtifactIdentity,
        lsn_range: WalLsnRange,
        byte_count: u64,
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            checkpoint,
            segment,
            lsn_range,
            byte_count,
            operation,
        }
    }

    pub const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub const fn segment(&self) -> WalSegmentArtifactIdentity {
        self.segment
    }

    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn byte_count(&self) -> u64 {
        self.byte_count
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn role(&self) -> MediaOperationRole {
        MediaOperationRole::Delete
    }
}

impl IndeterminatePhysicalWalReclamationAction {
    pub(in crate::physical_runtime) const fn new(
        checkpoint: PhysicalCheckpointIdentity,
        segment: WalSegmentArtifactIdentity,
        operation: MediaOperationIdentity,
        failure: ArtifactTreeFailure,
    ) -> Self {
        Self {
            checkpoint,
            segment,
            operation,
            failure,
        }
    }

    pub(in crate::physical_runtime) const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub(in crate::physical_runtime) const fn segment(&self) -> WalSegmentArtifactIdentity {
        self.segment
    }

    pub(in crate::physical_runtime) const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub(in crate::physical_runtime) const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
}
