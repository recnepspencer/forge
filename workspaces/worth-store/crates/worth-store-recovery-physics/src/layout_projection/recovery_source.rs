use crate::{
    AdmittedRecoverySource, CheckpointId, RecoverySourceDecisionKind, RecoverySourceDecisionRow,
    WalLsnRange,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

pub fn reject_decision_row(
    _row: &RecoverySourceDecisionRow,
) -> Result<(), RecoveryLayoutAccessDenial> {
    Err(RecoveryLayoutAccessDenial::new(
        RecoveryLayoutAccessDenialKind::RecoverySourceRowCannotStandInForRecoveryAuthority,
    ))
}

pub(crate) fn project_recovery_source_layout(
    source: &AdmittedRecoverySource,
) -> RecoverySourceLayoutReport {
    RecoverySourceLayoutReport::from_source(source)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourceLayoutReport {
    decision_kind: RecoverySourceDecisionKind,
    candidate_count: usize,
    residue_rejection_count: usize,
    replay_basis: RecoverySourceReplayBasis,
}

impl RecoverySourceLayoutReport {
    pub fn from_source(source: &AdmittedRecoverySource) -> Self {
        let trace = source.trace();
        Self {
            decision_kind: trace.kind(),
            candidate_count: trace.candidate_count(),
            residue_rejection_count: trace.residue_rejections().len(),
            replay_basis: RecoverySourceReplayBasis {
                checkpoint_id: trace.replay_basis().checkpoint_id().cloned(),
                replay_frontier: trace.replay_basis().replay_frontier(),
            },
        }
    }

    pub const fn decision_kind(&self) -> RecoverySourceDecisionKind {
        self.decision_kind
    }

    pub const fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub const fn residue_rejection_count(&self) -> usize {
        self.residue_rejection_count
    }

    pub fn selected_checkpoint_id(&self) -> Option<&CheckpointId> {
        self.replay_basis.checkpoint_id.as_ref()
    }

    pub const fn selected_wal_range(&self) -> Option<WalLsnRange> {
        self.replay_basis.replay_frontier
    }

    pub const fn replay_basis(&self) -> &RecoverySourceReplayBasis {
        &self.replay_basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourceReplayBasis {
    checkpoint_id: Option<CheckpointId>,
    replay_frontier: Option<WalLsnRange>,
}

impl RecoverySourceReplayBasis {
    pub fn checkpoint_id(&self) -> Option<&CheckpointId> {
        self.checkpoint_id.as_ref()
    }

    pub const fn replay_frontier(&self) -> Option<WalLsnRange> {
        self.replay_frontier
    }
}
