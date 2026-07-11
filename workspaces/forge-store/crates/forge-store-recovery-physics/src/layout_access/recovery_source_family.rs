use crate::{
    AdmittedRecoverySource, CheckpointId, RecoverySourceDecisionKind, RecoverySourceDecisionRow,
    WalLsnRange,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRecoverySourceLayoutRule {
    _private: (),
}

impl AdmittedRecoverySourceLayoutRule {
    pub(crate) const fn internal_phase22() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase22-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase22() -> Self {
        Self::internal_phase22()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoverySourceLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoverySourceLayoutAdmission {
    _private: (),
}

impl RecoverySourceLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedRecoverySourceLayoutRule,
    ) -> Result<RecoverySourceLayoutAdmission, RecoveryLayoutAccessDenial> {
        Ok(RecoverySourceLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRecoverySourceLayoutFamily {
    _admission: RecoverySourceLayoutAdmission,
}

impl AdmittedRecoverySourceLayoutFamily {
    pub(crate) const fn new(admission: RecoverySourceLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn source_report(&self, source: &AdmittedRecoverySource) -> RecoverySourceLayoutReport {
        RecoverySourceLayoutReport::from_source(source)
    }

    pub fn reject_decision_row(
        &self,
        _row: &RecoverySourceDecisionRow,
    ) -> Result<(), RecoveryLayoutAccessDenial> {
        Err(RecoveryLayoutAccessDenial::new(
            RecoveryLayoutAccessDenialKind::RecoverySourceRowCannotStandInForRecoveryAuthority,
        ))
    }
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
    fn from_source(source: &AdmittedRecoverySource) -> Self {
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
