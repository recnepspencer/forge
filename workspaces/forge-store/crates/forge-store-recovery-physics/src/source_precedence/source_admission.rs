use super::{CheckpointBaseAdmission, RecoverySourceDecisionTrace, WalTailRedoSource};
use crate::RecoveryBlockedByIntegrityDamage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmittedRecoverySource {
    CheckpointPlusWalTail {
        checkpoint: CheckpointBaseAdmission,
        wal_tail: WalTailRedoSource,
        trace: RecoverySourceDecisionTrace,
    },
    WalOnly {
        wal_tail: WalTailRedoSource,
        trace: RecoverySourceDecisionTrace,
    },
    NoValidCheckpoint {
        trace: RecoverySourceDecisionTrace,
    },
    RecoveryBlocked {
        damage: RecoveryBlockedByIntegrityDamage,
        trace: RecoverySourceDecisionTrace,
    },
}

impl AdmittedRecoverySource {
    pub const fn trace(&self) -> &RecoverySourceDecisionTrace {
        match self {
            Self::CheckpointPlusWalTail { trace, .. }
            | Self::WalOnly { trace, .. }
            | Self::NoValidCheckpoint { trace }
            | Self::RecoveryBlocked { trace, .. } => trace,
        }
    }

    pub const fn selected_checkpoint(&self) -> Option<&CheckpointBaseAdmission> {
        match self {
            Self::CheckpointPlusWalTail { checkpoint, .. } => Some(checkpoint),
            Self::WalOnly { .. }
            | Self::NoValidCheckpoint { .. }
            | Self::RecoveryBlocked { .. } => None,
        }
    }

    pub const fn selected_wal_tail(&self) -> Option<&WalTailRedoSource> {
        match self {
            Self::CheckpointPlusWalTail { wal_tail, .. } | Self::WalOnly { wal_tail, .. } => {
                Some(wal_tail)
            }
            Self::NoValidCheckpoint { .. } | Self::RecoveryBlocked { .. } => None,
        }
    }
}
