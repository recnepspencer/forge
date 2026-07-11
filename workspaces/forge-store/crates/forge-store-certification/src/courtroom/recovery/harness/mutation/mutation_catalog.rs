use crate::courtroom::recovery::harness::{
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsMutationFailureEvidence,
    RecoveryPhysicsOracleKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsMutant {
    WalAppendAcknowledgedBeforeDurable,
    PageFlushWithoutWalBeforeData,
    CheckpointWriteAcceptedWithoutManifest,
    CheckpointCutoverWithoutDurableManifest,
    CompactionCutoverFromBackendResidue,
    LiveAckMemoryReused,
    SameRunSelfComparisonAccepted,
    DirectPrivateMutationAccepted,
}

impl RecoveryPhysicsMutant {
    pub const REQUIRED_S4_MUTANTS: [Self; 8] = [
        Self::WalAppendAcknowledgedBeforeDurable,
        Self::PageFlushWithoutWalBeforeData,
        Self::CheckpointWriteAcceptedWithoutManifest,
        Self::CheckpointCutoverWithoutDurableManifest,
        Self::CompactionCutoverFromBackendResidue,
        Self::LiveAckMemoryReused,
        Self::SameRunSelfComparisonAccepted,
        Self::DirectPrivateMutationAccepted,
    ];

    pub const fn intended_lane(&self) -> RecoveryPhysicsCrashLane {
        match self {
            Self::WalAppendAcknowledgedBeforeDurable => RecoveryPhysicsCrashLane::WalAppend,
            Self::PageFlushWithoutWalBeforeData => RecoveryPhysicsCrashLane::PageFlush,
            Self::CheckpointWriteAcceptedWithoutManifest => {
                RecoveryPhysicsCrashLane::CheckpointWrite
            }
            Self::CheckpointCutoverWithoutDurableManifest => {
                RecoveryPhysicsCrashLane::CheckpointCutover
            }
            Self::CompactionCutoverFromBackendResidue => {
                RecoveryPhysicsCrashLane::CompactionCutover
            }
            Self::LiveAckMemoryReused => RecoveryPhysicsCrashLane::Acknowledgment,
            Self::SameRunSelfComparisonAccepted => RecoveryPhysicsCrashLane::DirectorySync,
            Self::DirectPrivateMutationAccepted => RecoveryPhysicsCrashLane::RenameDurability,
        }
    }

    pub const fn failure_evidence(&self) -> RecoveryPhysicsMutationFailureEvidence {
        match self {
            Self::WalAppendAcknowledgedBeforeDurable
            | Self::PageFlushWithoutWalBeforeData
            | Self::CheckpointWriteAcceptedWithoutManifest
            | Self::CheckpointCutoverWithoutDurableManifest => {
                RecoveryPhysicsMutationFailureEvidence::Oracle(
                    RecoveryPhysicsOracleKind::DeterministicFreshRecovery,
                )
            }
            Self::CompactionCutoverFromBackendResidue | Self::LiveAckMemoryReused => {
                RecoveryPhysicsMutationFailureEvidence::Counter(
                    RecoveryPhysicsCounterKind::ShortcutDenials,
                )
            }
            Self::SameRunSelfComparisonAccepted | Self::DirectPrivateMutationAccepted => {
                RecoveryPhysicsMutationFailureEvidence::CompileFailBoundary
            }
        }
    }
}
