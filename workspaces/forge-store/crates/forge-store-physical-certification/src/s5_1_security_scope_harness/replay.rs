use super::physical_replay::require_replay_physical_binding;
use super::{
    S51SecurityScopeHarnessEvidence, S51SecurityScopeHarnessOutcomeKind,
    S51SecurityScopeHarnessSchedule, S51SecurityScopePhysicalReplayDenial,
    S51SecurityScopePhysicalReplayEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S51SecurityScopeReplayMutationKind {
    ChangedTenantScope,
    ChangedKeyVersionPosture,
    ChangedAuthenticityRequirement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessReplayCounterSnapshot {
    baseline_admissions: u64,
    replay_attempts: u64,
    replay_denials_before_logical_decode: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessReplayTranscript {
    schedule: S51SecurityScopeHarnessSchedule,
    mutation: S51SecurityScopeReplayMutationKind,
    baseline_physical_replay: S51SecurityScopePhysicalReplayEvidence,
    replay_physical_replay: S51SecurityScopePhysicalReplayEvidence,
    baseline_evidence: S51SecurityScopeHarnessEvidence,
    replay_evidence: S51SecurityScopeHarnessEvidence,
    counters: S51SecurityScopeHarnessReplayCounterSnapshot,
}

impl S51SecurityScopeHarnessReplayTranscript {
    pub fn from_physical_replay(
        mutation: S51SecurityScopeReplayMutationKind,
        baseline_physical_replay: S51SecurityScopePhysicalReplayEvidence,
        replay_physical_replay: S51SecurityScopePhysicalReplayEvidence,
        baseline_evidence: S51SecurityScopeHarnessEvidence,
        replay_evidence: S51SecurityScopeHarnessEvidence,
    ) -> Result<Self, S51SecurityScopePhysicalReplayDenial> {
        require_replay_physical_binding(
            &baseline_physical_replay,
            &replay_physical_replay,
            baseline_evidence,
            replay_evidence,
        )?;
        Ok(Self {
            schedule: baseline_evidence.scenario().schedule(),
            mutation,
            baseline_physical_replay,
            replay_physical_replay,
            baseline_evidence,
            replay_evidence,
            counters: S51SecurityScopeHarnessReplayCounterSnapshot::from_evidence_pair(
                baseline_evidence,
                replay_evidence,
            ),
        })
    }

    pub const fn schedule(&self) -> S51SecurityScopeHarnessSchedule {
        self.schedule
    }

    pub const fn mutation(&self) -> S51SecurityScopeReplayMutationKind {
        self.mutation
    }

    pub const fn baseline_evidence(&self) -> S51SecurityScopeHarnessEvidence {
        self.baseline_evidence
    }

    pub const fn replay_evidence(&self) -> S51SecurityScopeHarnessEvidence {
        self.replay_evidence
    }

    pub const fn baseline_physical_replay(&self) -> &S51SecurityScopePhysicalReplayEvidence {
        &self.baseline_physical_replay
    }

    pub const fn replay_physical_replay(&self) -> &S51SecurityScopePhysicalReplayEvidence {
        &self.replay_physical_replay
    }

    pub const fn counters(&self) -> S51SecurityScopeHarnessReplayCounterSnapshot {
        self.counters
    }

    pub fn replays_same_physical_schedule(&self) -> bool {
        self.baseline_physical_replay
            .same_physical_schedule_identity_as(&self.replay_physical_replay)
    }

    pub fn replay_rejected_before_logical_decode(&self) -> bool {
        self.replay_evidence
            .counters()
            .denied_before_logical_decode()
            == 1
            && self.replay_evidence.observation().outcome()
                != S51SecurityScopeHarnessOutcomeKind::Admitted
    }
}

impl S51SecurityScopeHarnessReplayCounterSnapshot {
    fn from_evidence_pair(
        baseline_evidence: S51SecurityScopeHarnessEvidence,
        replay_evidence: S51SecurityScopeHarnessEvidence,
    ) -> Self {
        Self {
            baseline_admissions: baseline_evidence.counters().readiness_acceptances(),
            replay_attempts: replay_evidence.counters().scope_admission_attempts(),
            replay_denials_before_logical_decode: replay_evidence
                .counters()
                .denied_before_logical_decode(),
        }
    }

    pub const fn baseline_admissions(self) -> u64 {
        self.baseline_admissions
    }

    pub const fn replay_attempts(self) -> u64 {
        self.replay_attempts
    }

    pub const fn replay_denials_before_logical_decode(self) -> u64 {
        self.replay_denials_before_logical_decode
    }
}
