use forge_store_physical_isolation::S5IsolationEvidenceProfile;

use crate::PhysicalSimulationScenarioFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5ExecutedIsolationOutcome {
    Satisfied,
    DeniedMutation,
    NonClaimStabilityOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5ExecutedIsolationSourceBasis {
    family: &'static str,
    plan_digest: [u8; 32],
    schedule_digest: [u8; 32],
    transcript_digest: [u8; 32],
    replay_basis_digest: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5ExecutedIsolationRequiredCounters {
    outcome_count: u64,
    retry_count: u64,
    latch_count: u64,
    reclaim_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5EvidenceProfileCounterSet {
    outcome_count: u64,
    retry_count: u64,
    latch_count: u64,
    reclaim_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5ExecutedIsolationFinding {
    basis: S5ExecutedIsolationSourceBasis,
    outcome: S5ExecutedIsolationOutcome,
    counters: S5ExecutedIsolationRequiredCounters,
    profile: S5IsolationEvidenceProfile,
}

impl S5ExecutedIsolationSourceBasis {
    pub const fn new(
        family: &'static str,
        plan_digest: [u8; 32],
        schedule_digest: [u8; 32],
        transcript_digest: [u8; 32],
        replay_basis_digest: [u8; 32],
    ) -> Self {
        Self {
            family,
            plan_digest,
            schedule_digest,
            transcript_digest,
            replay_basis_digest,
        }
    }

    pub const fn family(&self) -> &'static str {
        self.family
    }

    pub const fn plan_digest(&self) -> &[u8; 32] {
        &self.plan_digest
    }

    pub const fn schedule_digest(&self) -> &[u8; 32] {
        &self.schedule_digest
    }

    pub const fn transcript_digest(&self) -> &[u8; 32] {
        &self.transcript_digest
    }

    pub const fn replay_basis_digest(&self) -> &[u8; 32] {
        &self.replay_basis_digest
    }
}

impl S5ExecutedIsolationRequiredCounters {
    pub const fn new(
        outcome_count: u64,
        retry_count: u64,
        latch_count: u64,
        reclaim_count: u64,
    ) -> Self {
        Self {
            outcome_count,
            retry_count,
            latch_count,
            reclaim_count,
        }
    }

    pub const fn outcome_count(self) -> u64 {
        self.outcome_count
    }

    pub const fn retry_count(self) -> u64 {
        self.retry_count
    }

    pub const fn latch_count(self) -> u64 {
        self.latch_count
    }

    pub const fn reclaim_count(self) -> u64 {
        self.reclaim_count
    }

    pub const fn profile_counter_set(self) -> S5EvidenceProfileCounterSet {
        S5EvidenceProfileCounterSet {
            outcome_count: self.outcome_count,
            retry_count: self.retry_count,
            latch_count: self.latch_count,
            reclaim_count: self.reclaim_count,
        }
    }
}

impl S5EvidenceProfileCounterSet {
    pub const fn outcome_count(self) -> u64 {
        self.outcome_count
    }

    pub const fn retry_count(self) -> u64 {
        self.retry_count
    }

    pub const fn latch_count(self) -> u64 {
        self.latch_count
    }

    pub const fn reclaim_count(self) -> u64 {
        self.reclaim_count
    }

    pub const fn matches_required_counters(
        self,
        required: S5ExecutedIsolationRequiredCounters,
    ) -> bool {
        self.outcome_count == required.outcome_count()
            && self.retry_count == required.retry_count()
            && self.latch_count == required.latch_count()
            && self.reclaim_count == required.reclaim_count()
    }
}

impl S5ExecutedIsolationFinding {
    pub fn from_admitted_executed_source(
        family: PhysicalSimulationScenarioFamily,
        basis: S5ExecutedIsolationSourceBasis,
        counters: S5ExecutedIsolationRequiredCounters,
        profile: S5IsolationEvidenceProfile,
    ) -> Self {
        Self {
            basis,
            outcome: outcome_for_family(family),
            counters,
            profile,
        }
    }

    pub const fn basis(&self) -> &S5ExecutedIsolationSourceBasis {
        &self.basis
    }

    pub const fn outcome(&self) -> S5ExecutedIsolationOutcome {
        self.outcome
    }

    pub const fn counters(&self) -> S5ExecutedIsolationRequiredCounters {
        self.counters
    }

    pub const fn profile(&self) -> S5IsolationEvidenceProfile {
        self.profile
    }
}

fn outcome_for_family(family: PhysicalSimulationScenarioFamily) -> S5ExecutedIsolationOutcome {
    match family {
        PhysicalSimulationScenarioFamily::S5TierMovementStability
        | PhysicalSimulationScenarioFamily::S5FutureChunkStability => {
            S5ExecutedIsolationOutcome::NonClaimStabilityOnly
        }
        PhysicalSimulationScenarioFamily::S5CompactionInterlock
        | PhysicalSimulationScenarioFamily::S5ReclaimReachability => {
            S5ExecutedIsolationOutcome::DeniedMutation
        }
        _ => S5ExecutedIsolationOutcome::Satisfied,
    }
}
