use super::{
    BackendResidueRejection, RecoveryCandidateDiscoveryTrace, RecoverySourceApplicationRole,
};

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
}

impl RecoverySourceDecisionTrace {
    pub(crate) fn new(
        kind: RecoverySourceDecisionKind,
        profile: String,
        candidate_count: usize,
        roles: Vec<RecoverySourceApplicationRole>,
        residue_rejections: Vec<BackendResidueRejection>,
        decision_rows: Vec<RecoverySourceDecisionRow>,
    ) -> Self {
        Self {
            kind,
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
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

    pub fn canonical_replay_digest(&self) -> String {
        format!("{:?}:{}:{}", self.kind, self.profile, self.candidate_count)
    }
}
