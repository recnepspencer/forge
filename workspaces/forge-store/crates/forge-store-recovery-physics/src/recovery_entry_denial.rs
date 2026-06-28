use crate::RecoveryBlockedByIntegrityDamage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryEntryAdmissionDecision {
    Admitted(crate::RecoveryEntryAdmission),
    Blocked(RecoveryEntryBlockedByIntegrityDamage),
    Denied(RecoveryEntryAdmissionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEntryBlockedByIntegrityDamage {
    blockers: Vec<RecoveryBlockedByIntegrityDamage>,
    replay_planning_started: bool,
    source_precedence_chosen: bool,
}

impl RecoveryEntryBlockedByIntegrityDamage {
    pub(crate) fn before_replay_planning(blockers: Vec<RecoveryBlockedByIntegrityDamage>) -> Self {
        Self {
            blockers,
            replay_planning_started: false,
            source_precedence_chosen: false,
        }
    }

    pub fn blockers(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.blockers
    }

    pub fn blocker_count(&self) -> u64 {
        self.blockers.len() as u64
    }

    pub const fn replay_planning_started(&self) -> bool {
        self.replay_planning_started
    }

    pub const fn source_precedence_chosen(&self) -> bool {
        self.source_precedence_chosen
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryEntryAdmissionDenial {
    kind: RecoveryEntryAdmissionDenialKind,
}

impl RecoveryEntryAdmissionDenial {
    pub const fn new(kind: RecoveryEntryAdmissionDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> RecoveryEntryAdmissionDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEntryAdmissionDenialKind {
    IntegrityReadinessClaimsRecovery,
    RawBytesCrossedIntegrityBoundary,
    RecoveryMemoryEnvelopeClaimsWalRecovery,
    RecoveryMemoryEnvelopeClaimsCheckpointSafety,
    RecoveryMemoryEnvelopeClaimsRepairBehavior,
    MissingPhysicalAuthorityRecap,
}
