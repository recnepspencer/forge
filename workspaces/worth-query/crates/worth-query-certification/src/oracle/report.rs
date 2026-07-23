use crate::evidence::WorthQueryCertificationCounters;
use crate::scenario::{
    WorthQueryCertificationJourneyCheckpoint, WorthQueryCertificationScenarioKind,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCertificationScenarioReport {
    scenario_identity: String,
    kind: WorthQueryCertificationScenarioKind,
    journey_checkpoints: BTreeSet<WorthQueryCertificationJourneyCheckpoint>,
    counters: WorthQueryCertificationCounters,
}

impl WorthQueryCertificationScenarioReport {
    pub(crate) fn new(
        scenario_identity: String,
        kind: WorthQueryCertificationScenarioKind,
        journey_checkpoints: BTreeSet<WorthQueryCertificationJourneyCheckpoint>,
        counters: WorthQueryCertificationCounters,
    ) -> Self {
        Self {
            scenario_identity,
            kind,
            journey_checkpoints,
            counters,
        }
    }

    pub fn scenario_identity(&self) -> &str {
        &self.scenario_identity
    }

    pub fn kind(&self) -> WorthQueryCertificationScenarioKind {
        self.kind
    }

    pub fn journey_checkpoints(&self) -> &BTreeSet<WorthQueryCertificationJourneyCheckpoint> {
        &self.journey_checkpoints
    }

    pub fn counters(&self) -> &WorthQueryCertificationCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCertificationReport {
    provider_identities: [String; 2],
    scenarios: Vec<WorthQueryCertificationScenarioReport>,
}

impl WorthQueryCertificationReport {
    pub(crate) fn new(
        provider_identities: [String; 2],
        scenarios: Vec<WorthQueryCertificationScenarioReport>,
    ) -> Self {
        Self {
            provider_identities,
            scenarios,
        }
    }

    pub fn provider_identities(&self) -> &[String; 2] {
        &self.provider_identities
    }

    pub fn scenarios(&self) -> &[WorthQueryCertificationScenarioReport] {
        &self.scenarios
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryHostileCertificationReport {
    provider_identity: String,
    hostile_case_count: usize,
}

impl WorthQueryHostileCertificationReport {
    pub(crate) fn new(provider_identity: String, hostile_case_count: usize) -> Self {
        Self {
            provider_identity,
            hostile_case_count,
        }
    }

    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    pub fn hostile_case_count(&self) -> usize {
        self.hostile_case_count
    }
}
