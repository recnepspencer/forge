use forge_store_security::StoreSecurityScopeAdmissionCounterSnapshot;

use super::{
    S51SecurityScopeHarnessCounterSnapshot, S51SecurityScopeHarnessObservation,
    S51SecurityScopeHarnessScenario, S51SecurityScopeOracleVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessEvidence {
    scenario: S51SecurityScopeHarnessScenario,
    observation: S51SecurityScopeHarnessObservation,
    oracle: S51SecurityScopeOracleVerdict,
    counters: S51SecurityScopeHarnessCounterSnapshot,
    lower_store_admission_counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

impl S51SecurityScopeHarnessEvidence {
    pub fn from_observation_and_store_counters(
        observation: S51SecurityScopeHarnessObservation,
        lower_store_admission_counters: StoreSecurityScopeAdmissionCounterSnapshot,
    ) -> Self {
        let scenario = observation.scenario();
        let oracle = S51SecurityScopeOracleVerdict::from_observation(observation);
        let counters = S51SecurityScopeHarnessCounterSnapshot::start_scenario(scenario)
            .record_outcome(observation.outcome());
        Self {
            scenario,
            observation,
            oracle,
            counters,
            lower_store_admission_counters,
        }
    }

    pub const fn scenario(self) -> S51SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn observation(self) -> S51SecurityScopeHarnessObservation {
        self.observation
    }

    pub const fn oracle(self) -> S51SecurityScopeOracleVerdict {
        self.oracle
    }

    pub const fn counters(self) -> S51SecurityScopeHarnessCounterSnapshot {
        self.counters
    }

    pub const fn lower_store_admission_counters(
        self,
    ) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.lower_store_admission_counters
    }
}
