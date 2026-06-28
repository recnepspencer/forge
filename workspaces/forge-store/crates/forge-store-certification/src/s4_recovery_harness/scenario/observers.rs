#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsObserverKind {
    CrashLane,
    StorageBoundary,
    FreshRuntime,
    OracleEvidence,
    CounterEvidence,
    EvidenceBundle,
    Transcript,
}

impl RecoveryPhysicsObserverKind {
    pub const REQUIRED: [Self; 7] = [
        Self::CrashLane,
        Self::StorageBoundary,
        Self::FreshRuntime,
        Self::OracleEvidence,
        Self::CounterEvidence,
        Self::EvidenceBundle,
        Self::Transcript,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsCounterKind {
    LoweredScenarioPlans,
    StorageBoundaryFaults,
    FreshRuntimeRecoveries,
    OracleEvidenceRows,
    EvidenceBundles,
    Transcripts,
    ShortcutDenials,
    MutationFailures,
}

impl RecoveryPhysicsCounterKind {
    pub const REQUIRED_SCENARIO_COUNTERS: [Self; 6] = [
        Self::LoweredScenarioPlans,
        Self::StorageBoundaryFaults,
        Self::FreshRuntimeRecoveries,
        Self::OracleEvidenceRows,
        Self::EvidenceBundles,
        Self::Transcripts,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsCounterExpectation {
    kind: RecoveryPhysicsCounterKind,
    expected: u64,
}

impl RecoveryPhysicsCounterExpectation {
    pub const fn exact(kind: RecoveryPhysicsCounterKind, expected: u64) -> Self {
        Self { kind, expected }
    }

    pub const fn kind(&self) -> RecoveryPhysicsCounterKind {
        self.kind
    }

    pub const fn expected(&self) -> u64 {
        self.expected
    }
}
