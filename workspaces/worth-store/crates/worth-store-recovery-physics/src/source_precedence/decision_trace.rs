use super::{
    BackendResidueRejection, RecoveryCandidateDiscoveryTrace, RecoverySourceApplicationRole,
};
use crate::{CheckpointId, WalLsnRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoverySourceDecisionKind {
    CheckpointPlusWalTail,
    WalOnly,
    NoValidCheckpoint,
    RecoveryBlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoverySourceDecisionOutcome {
    AdmittedCandidate,
    ApplicationRoleOnly,
    DiscoveryOnly,
    RejectedResidue,
    RecoveryBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourceDecisionRow {
    trace: RecoveryCandidateDiscoveryTrace,
    role: RecoverySourceApplicationRole,
    outcome: RecoverySourceDecisionOutcome,
}

impl RecoverySourceDecisionRow {
    pub(crate) fn new(
        trace: RecoveryCandidateDiscoveryTrace,
        role: RecoverySourceApplicationRole,
        outcome: RecoverySourceDecisionOutcome,
    ) -> Self {
        Self {
            trace,
            role,
            outcome,
        }
    }

    pub const fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        &self.trace
    }

    pub const fn role(&self) -> RecoverySourceApplicationRole {
        self.role
    }

    pub const fn outcome(&self) -> RecoverySourceDecisionOutcome {
        self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourceDecisionTrace {
    kind: RecoverySourceDecisionKind,
    profile: String,
    candidate_count: usize,
    roles: Vec<RecoverySourceApplicationRole>,
    residue_rejections: Vec<BackendResidueRejection>,
    decision_rows: Vec<RecoverySourceDecisionRow>,
    replay_basis: RecoverySourceReplayBasis,
}

impl RecoverySourceDecisionTrace {
    pub(crate) fn new(
        kind: RecoverySourceDecisionKind,
        profile: String,
        candidate_count: usize,
        roles: Vec<RecoverySourceApplicationRole>,
        residue_rejections: Vec<BackendResidueRejection>,
        decision_rows: Vec<RecoverySourceDecisionRow>,
        replay_basis: RecoverySourceReplayBasis,
    ) -> Self {
        Self {
            kind,
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
            replay_basis,
        }
    }

    pub const fn kind(&self) -> RecoverySourceDecisionKind {
        self.kind
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub const fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub fn roles(&self) -> &[RecoverySourceApplicationRole] {
        &self.roles
    }

    pub fn residue_rejections(&self) -> &[BackendResidueRejection] {
        &self.residue_rejections
    }

    pub fn decision_rows(&self) -> &[RecoverySourceDecisionRow] {
        &self.decision_rows
    }

    pub(crate) const fn replay_basis(&self) -> &RecoverySourceReplayBasis {
        &self.replay_basis
    }

    pub fn canonical_replay_digest(&self) -> String {
        format!(
            "{:?}:checkpoint={}:frontier={}",
            self.kind,
            self.replay_basis
                .checkpoint_id()
                .map(|checkpoint_id| checkpoint_id.digest().as_str())
                .unwrap_or("none"),
            self.replay_basis
                .replay_frontier()
                .map(|frontier| format!(
                    "{}-{}",
                    frontier.start().get(),
                    frontier.end_exclusive().get()
                ))
                .unwrap_or_else(|| "none".to_owned())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoverySourceReplayBasis {
    checkpoint_id: Option<CheckpointId>,
    replay_frontier: Option<WalLsnRange>,
}

impl RecoverySourceReplayBasis {
    pub(crate) const fn empty() -> Self {
        Self {
            checkpoint_id: None,
            replay_frontier: None,
        }
    }

    pub(crate) const fn wal_only(replay_frontier: WalLsnRange) -> Self {
        Self {
            checkpoint_id: None,
            replay_frontier: Some(replay_frontier),
        }
    }

    pub(crate) fn checkpoint_plus_tail(
        checkpoint_id: CheckpointId,
        replay_frontier: WalLsnRange,
    ) -> Self {
        Self {
            checkpoint_id: Some(checkpoint_id),
            replay_frontier: Some(replay_frontier),
        }
    }

    pub(crate) fn checkpoint_id(&self) -> Option<&CheckpointId> {
        self.checkpoint_id.as_ref()
    }

    pub(crate) const fn replay_frontier(&self) -> Option<WalLsnRange> {
        self.replay_frontier
    }
}
