#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsOracleKind {
    DeterministicFreshRecovery,
    RejectSyntheticShortcut,
    CounterExpectationsMatch,
    TranscriptEvidenceComplete,
    MutationFailsIntendedLane,
}

impl RecoveryPhysicsOracleKind {
    pub const REQUIRED_SCENARIO_ORACLES: [Self; 4] = [
        Self::DeterministicFreshRecovery,
        Self::RejectSyntheticShortcut,
        Self::CounterExpectationsMatch,
        Self::TranscriptEvidenceComplete,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsOracleJudgment {
    kind: RecoveryPhysicsOracleKind,
    passed: bool,
}

impl RecoveryPhysicsOracleJudgment {
    pub const fn passed(kind: RecoveryPhysicsOracleKind) -> Self {
        Self { kind, passed: true }
    }

    pub const fn failed(kind: RecoveryPhysicsOracleKind) -> Self {
        Self {
            kind,
            passed: false,
        }
    }

    pub const fn kind(&self) -> RecoveryPhysicsOracleKind {
        self.kind
    }

    pub const fn passed_certification(&self) -> bool {
        self.passed
    }
}
