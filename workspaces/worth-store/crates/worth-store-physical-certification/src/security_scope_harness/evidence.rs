use worth_store_security::StoreSecurityScopeAdmissionCounterSnapshot;

use super::{
    SecurityScopeHarnessCounterSnapshot, SecurityScopeHarnessObservation,
    SecurityScopeHarnessScenario, SecurityScopeOracleVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityScopeHarnessEvidence {
    scenario: SecurityScopeHarnessScenario,
    observation: SecurityScopeHarnessObservation,
    oracle: SecurityScopeOracleVerdict,
    counters: SecurityScopeHarnessCounterSnapshot,
    lower_store_admission_counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

impl SecurityScopeHarnessEvidence {
    pub fn from_observation_and_store_counters(
        observation: SecurityScopeHarnessObservation,
        lower_store_admission_counters: StoreSecurityScopeAdmissionCounterSnapshot,
    ) -> Self {
        let scenario = observation.scenario();
        let oracle = SecurityScopeOracleVerdict::from_observation(observation);
        let counters = SecurityScopeHarnessCounterSnapshot::start_scenario(scenario)
            .record_outcome(observation.outcome());
        Self {
            scenario,
            observation,
            oracle,
            counters,
            lower_store_admission_counters,
        }
    }

    pub const fn scenario(self) -> SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn observation(self) -> SecurityScopeHarnessObservation {
        self.observation
    }

    pub const fn oracle(self) -> SecurityScopeOracleVerdict {
        self.oracle
    }

    pub const fn counters(self) -> SecurityScopeHarnessCounterSnapshot {
        self.counters
    }

    pub const fn lower_store_admission_counters(
        self,
    ) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.lower_store_admission_counters
    }
}
