use super::{
    CompletedMediaTransfer, MediaCapability, MediaFailureContext, MediaOperationIdentity,
    PartialMediaTransfer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletedMediaEffect {
    ExistingHandleOpened { handle: super::MediaHandleIdentity },
    NewFileCreated { handle: super::MediaHandleIdentity },
    PositionedReadCompleted(CompletedMediaTransfer),
    PositionedWriteCompleted(CompletedMediaTransfer),
    AppendCompleted(CompletedMediaTransfer),
    LogicalLengthChanged,
    AllocationCompleted,
    MetadataObserved,
    DirectoryBatchObserved,
    FileDataSynchronized,
    FileStateSynchronized,
    DirectoryPublicationSynchronized,
    AtomicReplacementCompleted,
    NamespaceEntryDeleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaAttemptedEffect {
    ExistingHandleAcquisition,
    NewFileCreation,
    PositionedRead { requested_bytes: u64 },
    PositionedWrite { requested_bytes: u64 },
    Append { requested_bytes: u64 },
    LogicalLengthChange,
    Allocation,
    MetadataObservation,
    DirectoryObservation,
    FileDataSynchronization,
    FileStateSynchronization,
    DirectoryPublicationSynchronization,
    AtomicReplacement,
    NamespaceEntryDeletion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaEstablishedBoundary {
    None,
    HandleAcquired,
    NamespaceEntryCreationIssued,
    BytePrefix { completed_bytes: u64 },
    LogicalLengthChangeIssued,
    AllocationIssued,
    FileDataSynchronizationIssued,
    FileStateSynchronizationIssued,
    DirectoryPublicationSynchronizationIssued,
    AtomicReplacementIssued,
    NamespaceEntryDeletionIssued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaEffectStatus {
    DeniedBeforeEffect,
    PartialTransfer,
    CompletedEffect,
    IndeterminateEffect,
    UnsupportedCapability,
    StaleHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaRetryPosture {
    SafeFromStart,
    SafeFromContinuationPosition(u64),
    InspectionRequired,
    CapabilityUnavailable,
    RebindHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaOperationFailureKind {
    DeniedBeforeEffect,
    PartialTransfer(PartialMediaTransfer),
    IndeterminateEffect {
        attempted: MediaAttemptedEffect,
        last_established: MediaEstablishedBoundary,
    },
    UnsupportedCapability(MediaCapability),
    StaleHandle,
}

/// Sealed failure observation minted by the media owner after an attempted operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOperationFailure {
    operation: MediaOperationIdentity,
    kind: MediaOperationFailureKind,
    context: MediaFailureContext,
}

impl MediaOperationFailure {
    pub const fn kind(self) -> MediaOperationFailureKind {
        self.kind
    }

    pub const fn effect_status(self) -> MediaEffectStatus {
        match self.kind {
            MediaOperationFailureKind::DeniedBeforeEffect => MediaEffectStatus::DeniedBeforeEffect,
            MediaOperationFailureKind::PartialTransfer(_) => MediaEffectStatus::PartialTransfer,
            MediaOperationFailureKind::IndeterminateEffect { .. } => {
                MediaEffectStatus::IndeterminateEffect
            }
            MediaOperationFailureKind::UnsupportedCapability(_) => {
                MediaEffectStatus::UnsupportedCapability
            }
            MediaOperationFailureKind::StaleHandle => MediaEffectStatus::StaleHandle,
        }
    }

    pub const fn retry_posture(self) -> MediaRetryPosture {
        match self.kind {
            MediaOperationFailureKind::DeniedBeforeEffect => MediaRetryPosture::SafeFromStart,
            MediaOperationFailureKind::PartialTransfer(transfer) => {
                match (
                    self.context.operation().contract().retry(),
                    transfer.continuation_position(),
                ) {
                    (super::MediaRetryRule::ContinueFromEstablishedPosition, Some(position)) => {
                        MediaRetryPosture::SafeFromContinuationPosition(position)
                    }
                    _ => MediaRetryPosture::InspectionRequired,
                }
            }
            MediaOperationFailureKind::IndeterminateEffect { .. } => {
                MediaRetryPosture::InspectionRequired
            }
            MediaOperationFailureKind::UnsupportedCapability(_) => {
                MediaRetryPosture::CapabilityUnavailable
            }
            MediaOperationFailureKind::StaleHandle => MediaRetryPosture::RebindHandle,
        }
    }

    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn context(self) -> MediaFailureContext {
        self.context
    }

    #[cfg(test)]
    pub(super) const fn for_test(
        operation: MediaOperationIdentity,
        kind: MediaOperationFailureKind,
        context: MediaFailureContext,
    ) -> Self {
        Self {
            operation,
            kind,
            context,
        }
    }

    pub(super) const fn new(
        operation: MediaOperationIdentity,
        kind: MediaOperationFailureKind,
        context: MediaFailureContext,
    ) -> Self {
        Self {
            operation,
            kind,
            context,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaOperationResult {
    Completed(CompletedMediaEffect),
    Failed(MediaOperationFailure),
}

/// Sealed correlation between an owner-issued identity and its result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOperationOutcome {
    operation: MediaOperationIdentity,
    result: MediaOperationResult,
}

impl MediaOperationOutcome {
    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn result(self) -> MediaOperationResult {
        self.result
    }

    pub const fn effect_status(self) -> MediaEffectStatus {
        match self.result {
            MediaOperationResult::Completed(_) => MediaEffectStatus::CompletedEffect,
            MediaOperationResult::Failed(failure) => failure.effect_status(),
        }
    }

    #[cfg(test)]
    pub(super) const fn completed_for_test(
        operation: MediaOperationIdentity,
        effect: CompletedMediaEffect,
    ) -> Self {
        Self {
            operation,
            result: MediaOperationResult::Completed(effect),
        }
    }

    pub(super) const fn completed(
        operation: MediaOperationIdentity,
        effect: CompletedMediaEffect,
    ) -> Self {
        Self {
            operation,
            result: MediaOperationResult::Completed(effect),
        }
    }

    pub(super) const fn failed(
        operation: MediaOperationIdentity,
        failure: MediaOperationFailure,
    ) -> Self {
        Self {
            operation,
            result: MediaOperationResult::Failed(failure),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionedReadResult {
    Transferred(CompletedMediaTransfer),
    EndOfFile { requested_offset: u64 },
    Failed(MediaOperationFailure),
}

/// Sealed read observation correlated with one owner-issued operation identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionedReadOutcome {
    operation: MediaOperationIdentity,
    result: PositionedReadResult,
}

impl PositionedReadOutcome {
    pub const fn operation(self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn result(self) -> PositionedReadResult {
        self.result
    }

    #[cfg(test)]
    pub(super) const fn for_test(
        operation: MediaOperationIdentity,
        result: PositionedReadResult,
    ) -> Self {
        Self { operation, result }
    }

    pub(super) const fn new(
        operation: MediaOperationIdentity,
        result: PositionedReadResult,
    ) -> Self {
        Self { operation, result }
    }
}
