#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DurabilityRecoveryAction {
    WalAppendProposed,
    WalAppendCompletedInMemory,
    WalFenceRequested,
    WalFenceCompleted,
    WalAcknowledgmentLegal,
    PageFlushRequested,
    PageFlushCompleted,
    PageFlushDurabilityUncertain,
    CheckpointBegun,
    CheckpointDurable,
    DirectorySyncCompleted,
    DirectorySyncFailed,
    CheckpointPublished,
    CheckpointSelected,
    RecoveryReplayRequired,
    RecoveryReplayApplied,
    RecoveryReplayRejectedGenerationMismatch,
    RecoveryReplaySkippedIdempotent,
    RecoveredRootPublicationPending,
    RecoveredRootPublicationCompleted,
    Crash,
    Reopen,
}

impl DurabilityRecoveryAction {
    pub const fn all() -> [Self; 22] {
        [
            Self::WalAppendProposed,
            Self::WalAppendCompletedInMemory,
            Self::WalFenceRequested,
            Self::WalFenceCompleted,
            Self::WalAcknowledgmentLegal,
            Self::PageFlushRequested,
            Self::PageFlushCompleted,
            Self::PageFlushDurabilityUncertain,
            Self::CheckpointBegun,
            Self::CheckpointDurable,
            Self::DirectorySyncCompleted,
            Self::DirectorySyncFailed,
            Self::CheckpointPublished,
            Self::CheckpointSelected,
            Self::RecoveryReplayRequired,
            Self::RecoveryReplayApplied,
            Self::RecoveryReplayRejectedGenerationMismatch,
            Self::RecoveryReplaySkippedIdempotent,
            Self::RecoveredRootPublicationPending,
            Self::RecoveredRootPublicationCompleted,
            Self::Crash,
            Self::Reopen,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityRecoveryDenial {
    AmbiguousWalDurability,
    PageFlushAheadOfWal,
    CheckpointFrontierNotDurable,
    DirectorySyncNotDurable,
    RecoveryBasisNotSelected,
    ReplayNotResolved,
    RedoGenerationMismatch,
    IllegalTransition,
}
