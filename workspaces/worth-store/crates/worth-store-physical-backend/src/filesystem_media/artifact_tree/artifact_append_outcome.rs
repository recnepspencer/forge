use sha2::{Digest, Sha256};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{ArtifactTreeFailure, ArtifactTreeFile};
use crate::{
    filesystem_media::{MediaOperationIdentity, MediaOwnerIdentity},
    BackendQueueExecutionCompletion,
};

/// Exact nonempty byte interval expected to begin at the artifact EOF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtifactAppendRange {
    offset: u64,
    byte_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedArtifactAppend {
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    artifact: ArtifactTreeFile,
    range: ArtifactAppendRange,
    payload_digest: [u8; 32],
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateArtifactAppend {
    failure: ArtifactTreeFailure,
    owner: MediaOwnerIdentity,
    store: StableStoreIdentity,
    artifact: ArtifactTreeFile,
    range: ArtifactAppendRange,
    payload_digest: [u8; 32],
    completed_bytes: u64,
    operation: MediaOperationIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactAppendOutcome {
    Completed(CompletedArtifactAppend),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactAppend),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledArtifactAppend {
    pub(super) physical: CompletedArtifactAppend,
    pub(super) queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledArtifactAppendOutcome {
    Completed(Box<CompletedScheduledArtifactAppend>),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate(IndeterminateArtifactAppend),
}

impl ArtifactAppendRange {
    pub const fn new(offset: u64, byte_count: u64) -> Option<Self> {
        if byte_count == 0 || offset.checked_add(byte_count).is_none() {
            return None;
        }
        Some(Self { offset, byte_count })
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }

    pub const fn end_exclusive(self) -> u64 {
        self.offset + self.byte_count
    }
}

impl CompletedArtifactAppend {
    pub(super) fn new(
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        artifact: ArtifactTreeFile,
        range: ArtifactAppendRange,
        bytes: &[u8],
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            owner,
            store,
            artifact,
            range,
            payload_digest: Sha256::digest(bytes).into(),
            operation,
        }
    }

    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
    }

    pub const fn range(&self) -> ArtifactAppendRange {
        self.range
    }

    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }
}

impl IndeterminateArtifactAppend {
    pub(super) fn new(
        failure: ArtifactTreeFailure,
        owner: MediaOwnerIdentity,
        store: StableStoreIdentity,
        artifact: ArtifactTreeFile,
        range: ArtifactAppendRange,
        bytes: &[u8],
        completed_bytes: u64,
        operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            failure,
            owner,
            store,
            artifact,
            range,
            payload_digest: Sha256::digest(bytes).into(),
            completed_bytes,
            operation,
        }
    }

    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }

    pub const fn owner(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn artifact(&self) -> &ArtifactTreeFile {
        &self.artifact
    }

    pub const fn range(&self) -> ArtifactAppendRange {
        self.range
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
}

impl CompletedScheduledArtifactAppend {
    pub const fn physical(&self) -> &CompletedArtifactAppend {
        &self.physical
    }

    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

#[cfg(test)]
mod tests {
    use super::ArtifactAppendRange;

    #[test]
    fn append_ranges_are_nonempty_and_nonoverflowing() {
        assert!(ArtifactAppendRange::new(0, 1).is_some());
        assert!(ArtifactAppendRange::new(0, 0).is_none());
        assert!(ArtifactAppendRange::new(u64::MAX, 1).is_none());
    }
}
