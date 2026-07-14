use crate::courtroom::recovery::harness::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane,
    RecoveryPhysicsMutant, RecoveryPhysicsOracleKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsMutationFailureEvidence {
    Oracle(RecoveryPhysicsOracleKind),
    Counter(RecoveryPhysicsCounterKind),
    CompileFailBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsMutationSuiteLaneEvidence {
    mutant: RecoveryPhysicsMutant,
    lane: RecoveryPhysicsCrashLane,
    failure_evidence: RecoveryPhysicsMutationFailureEvidence,
    counter: RecoveryPhysicsCounterExpectation,
}

impl RecoveryPhysicsMutationSuiteLaneEvidence {
    pub(crate) const fn from_suite_lane(
        mutant: RecoveryPhysicsMutant,
        lane: RecoveryPhysicsCrashLane,
        failure_evidence: RecoveryPhysicsMutationFailureEvidence,
        counter: RecoveryPhysicsCounterExpectation,
    ) -> Self {
        Self {
            mutant,
            lane,
            failure_evidence,
            counter,
        }
    }

    pub const fn with_lane(mut self, lane: RecoveryPhysicsCrashLane) -> Self {
        self.lane = lane;
        self
    }

    pub const fn with_failure_evidence(
        mut self,
        failure_evidence: RecoveryPhysicsMutationFailureEvidence,
    ) -> Self {
        self.failure_evidence = failure_evidence;
        self
    }

    pub const fn with_counter(mut self, counter: RecoveryPhysicsCounterExpectation) -> Self {
        self.counter = counter;
        self
    }

    pub const fn mutant(&self) -> RecoveryPhysicsMutant {
        self.mutant
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.lane
    }

    pub const fn failure_evidence(&self) -> RecoveryPhysicsMutationFailureEvidence {
        self.failure_evidence
    }

    pub const fn counter(&self) -> RecoveryPhysicsCounterExpectation {
        self.counter
    }
}
