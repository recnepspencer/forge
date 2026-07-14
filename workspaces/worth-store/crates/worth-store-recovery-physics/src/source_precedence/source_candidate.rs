use super::{
    BackendResidueRejection, CheckpointBaseAdmission, CompactionCutoverRecoveryPosture,
    PageLsnSkipApplyDecision, RecoveryCandidateDiscoveryTrace, WalTailRedoSource,
};
use crate::RecoveryBlockedByIntegrityDamage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoverySourceCandidate {
    CheckpointBase {
        admission: CheckpointBaseAdmission,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    OrphanedCheckpointManifest {
        rejection: BackendResidueRejection,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    WalTail {
        source: WalTailRedoSource,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    PageImage {
        decision: PageLsnSkipApplyDecision,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    BackendResidue {
        rejection: BackendResidueRejection,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    CompactionProduct {
        posture: CompactionCutoverRecoveryPosture,
        trace: RecoveryCandidateDiscoveryTrace,
    },
    RecoveryBlocked {
        damage: RecoveryBlockedByIntegrityDamage,
        trace: RecoveryCandidateDiscoveryTrace,
    },
}

impl RecoverySourceCandidate {
    pub fn checkpoint_base(admission: CheckpointBaseAdmission) -> Self {
        let trace = admission.trace().clone();
        Self::CheckpointBase { admission, trace }
    }

    pub fn orphaned_checkpoint_manifest(rejection: BackendResidueRejection) -> Self {
        let trace = rejection.trace().clone();
        Self::OrphanedCheckpointManifest { rejection, trace }
    }

    pub fn wal_tail(source: WalTailRedoSource) -> Self {
        let trace = source.trace().clone();
        Self::WalTail { source, trace }
    }

    pub fn page_image(
        decision: PageLsnSkipApplyDecision,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::PageImage { decision, trace }
    }

    pub fn backend_residue(rejection: BackendResidueRejection) -> Self {
        let trace = rejection.trace().clone();
        Self::BackendResidue { rejection, trace }
    }

    pub fn compaction_product(
        posture: CompactionCutoverRecoveryPosture,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::CompactionProduct { posture, trace }
    }

    pub fn recovery_blocked(
        damage: RecoveryBlockedByIntegrityDamage,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Self {
        Self::RecoveryBlocked { damage, trace }
    }

    pub fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        match self {
            Self::CheckpointBase { trace, .. }
            | Self::OrphanedCheckpointManifest { trace, .. }
            | Self::WalTail { trace, .. }
            | Self::PageImage { trace, .. }
            | Self::BackendResidue { trace, .. }
            | Self::CompactionProduct { trace, .. }
            | Self::RecoveryBlocked { trace, .. } => trace,
        }
    }
}
