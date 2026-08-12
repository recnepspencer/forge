use worth_store_physical_backend::{
    ArtifactTreeFailure, BackendQueueExecutionCompletion, MediaOperationIdentity,
};
use worth_store_wal::{CheckpointCoveredWalCleanupDenial, WalSegmentArtifactIdentity};

use super::{
    RecoveryCleanupArtifactRevalidationDenial, RecoveryCleanupArtifactRevalidationProgress,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedRecoveryCleanupPhysicalRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    operation: MediaOperationIdentity,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedRecoveryCleanupPhysicalRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    cause: RecoveryCleanupRemovalDenialCause,
    queue: Option<BackendQueueExecutionCompletion>,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateRecoveryCleanupPhysicalRemoval {
    artifact: WalSegmentArtifactIdentity,
    admission: [u8; 32],
    operation: MediaOperationIdentity,
    failure: ArtifactTreeFailure,
    queue: BackendQueueExecutionCompletion,
    revalidation: RecoveryCleanupArtifactRevalidationProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCleanupRemovalDenialCause {
    Admission,
    TerminalCoverage(CheckpointCoveredWalCleanupDenial),
    Preparation(ArtifactTreeFailure),
    Revalidation(RecoveryCleanupArtifactRevalidationDenial),
    Removal(ArtifactTreeFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryCleanupRemovalOutcome {
    Completed(Box<CompletedRecoveryCleanupPhysicalRemoval>),
    DeniedBeforeEffect(Box<DeniedRecoveryCleanupPhysicalRemoval>),
    Indeterminate(Box<IndeterminateRecoveryCleanupPhysicalRemoval>),
}

impl CompletedRecoveryCleanupPhysicalRemoval {
    pub(in crate::physical_runtime) const fn new(
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        operation: MediaOperationIdentity,
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> Self {
        Self {
            artifact,
            admission,
            operation,
            queue,
            revalidation,
        }
    }

    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
    }
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
    }
    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl DeniedRecoveryCleanupPhysicalRemoval {
    pub(in crate::physical_runtime) const fn new(
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        cause: RecoveryCleanupRemovalDenialCause,
        queue: Option<BackendQueueExecutionCompletion>,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> Self {
        Self {
            artifact,
            admission,
            cause,
            queue,
            revalidation,
        }
    }

    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
    }
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
    }
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.cause.failure()
    }
    pub const fn cause(&self) -> RecoveryCleanupRemovalDenialCause {
        self.cause
    }
    pub const fn queue(&self) -> Option<BackendQueueExecutionCompletion> {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl IndeterminateRecoveryCleanupPhysicalRemoval {
    pub(in crate::physical_runtime) const fn new(
        artifact: WalSegmentArtifactIdentity,
        admission: [u8; 32],
        operation: MediaOperationIdentity,
        failure: ArtifactTreeFailure,
        queue: BackendQueueExecutionCompletion,
        revalidation: RecoveryCleanupArtifactRevalidationProgress,
    ) -> Self {
        Self {
            artifact,
            admission,
            operation,
            failure,
            queue,
            revalidation,
        }
    }

    pub const fn artifact(&self) -> WalSegmentArtifactIdentity {
        self.artifact
    }
    pub const fn admission(&self) -> [u8; 32] {
        self.admission
    }
    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
    pub const fn revalidation(&self) -> RecoveryCleanupArtifactRevalidationProgress {
        self.revalidation
    }
}

impl RecoveryCleanupRemovalDenialCause {
    pub const fn failure(self) -> ArtifactTreeFailure {
        match self {
            Self::Preparation(failure) | Self::Removal(failure) => failure,
            Self::Admission | Self::TerminalCoverage(_) | Self::Revalidation(_) => {
                ArtifactTreeFailure::recovery_denial()
            }
        }
    }
}
