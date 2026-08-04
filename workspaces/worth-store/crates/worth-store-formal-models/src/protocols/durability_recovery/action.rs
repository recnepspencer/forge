#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DurabilityRecoveryAction {
    WalAppendProposed,
    WalAppendCompletedInMemory,
    WalFenceRequested,
    WalFenceCompleted,
    PhysicalMutationAcknowledged,
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
    /// Every action in the independent durability policy model.
    pub const fn all() -> [Self; 22] {
        [
            Self::WalAppendProposed,
            Self::WalAppendCompletedInMemory,
            Self::WalFenceRequested,
            Self::WalFenceCompleted,
            Self::PhysicalMutationAcknowledged,
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

    /// Actions emitted by current typed production-owner mappings.
    pub const fn production_owned() -> [Self; 19] {
        [
            Self::WalAppendProposed,
            Self::WalAppendCompletedInMemory,
            Self::WalFenceRequested,
            Self::WalFenceCompleted,
            Self::PhysicalMutationAcknowledged,
            Self::PageFlushRequested,
            Self::PageFlushCompleted,
            Self::CheckpointBegun,
            Self::CheckpointDurable,
            Self::DirectorySyncCompleted,
            Self::CheckpointPublished,
            Self::CheckpointSelected,
            Self::RecoveryReplayRequired,
            Self::RecoveryReplayApplied,
            Self::RecoveryReplayRejectedGenerationMismatch,
            Self::RecoveryReplaySkippedIdempotent,
            Self::RecoveredRootPublicationPending,
            Self::RecoveredRootPublicationCompleted,
            Self::Reopen,
        ]
    }

    /// Retained policy states with no current production receipt owner.
    pub const fn policy_only() -> [Self; 3] {
        [
            Self::PageFlushDurabilityUncertain,
            Self::DirectorySyncFailed,
            Self::Crash,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityRecoveryDenial {
    IncompletePhysicalDurability,
    PhysicalMutationAlreadyAcknowledged,
    PageFlushAheadOfWal,
    CheckpointFrontierNotDurable,
    DirectorySyncNotDurable,
    RecoveryBasisNotSelected,
    ReplayNotResolved,
    RedoGenerationMismatch,
    IllegalTransition,
}
