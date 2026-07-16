use crate::{
    IntegrityDamageMap, RecoveryBlockedByIntegrityDamage, RecoveryCorruptionReadmissionHandoff,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryEntryAdmissionDecision {
    Admitted(Box<crate::RecoveryEntryAdmission>),
    Blocked(RecoveryEntryBlockedByIntegrityDamage),
    Denied(RecoveryEntryAdmissionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEntryBlockedByIntegrityDamage {
    blockers: Vec<RecoveryBlockedByIntegrityDamage>,
    readmission_handoffs: Vec<RecoveryCorruptionReadmissionHandoff>,
    replay_planning_started: bool,
    source_precedence_chosen: bool,
}

impl RecoveryEntryBlockedByIntegrityDamage {
    pub(crate) fn before_replay_planning(damage_map: &IntegrityDamageMap) -> Self {
        Self {
            blockers: damage_map.recovery_blocking_findings().to_vec(),
            readmission_handoffs: damage_map.build_corruption_readmission_handoffs(),
            replay_planning_started: false,
            source_precedence_chosen: false,
        }
    }

    pub fn blockers(&self) -> &[RecoveryBlockedByIntegrityDamage] {
        &self.blockers
    }

    pub fn corruption_readmission_handoffs(&self) -> &[RecoveryCorruptionReadmissionHandoff] {
        &self.readmission_handoffs
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
