use crate::data::telemetry::InvalidationPerformedCounter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalityOptionalObservationInventory {
    pub(crate) lineage_records: usize,
    pub(crate) replay_events: usize,
    pub(crate) explanation_facts: usize,
    pub(crate) provenance_facts: usize,
    pub(crate) frontier_summary: bool,
    pub(crate) flow_summary: bool,
    pub(crate) performed_counter_total: u64,
    pub(crate) performed_work_records: usize,
    pub(crate) lineage_sequences: Vec<u64>,
}

impl LocalityOptionalObservationInventory {
    pub(crate) fn is_idle_zero(&self) -> bool {
        *self
            == Self {
                lineage_records: 0,
                replay_events: 0,
                explanation_facts: 0,
                provenance_facts: 0,
                frontier_summary: false,
                flow_summary: false,
                performed_counter_total: 0,
                performed_work_records: 0,
                lineage_sequences: Vec::new(),
            }
    }
}

impl super::super::CompiledFinancialWorld {
    pub(crate) fn locality_optional_observation_inventory(
        &self,
    ) -> LocalityOptionalObservationInventory {
        self.locality().optional_observation_inventory()
    }
}

impl super::CompiledFinancialLocalityWorld {
    pub(crate) fn optional_observation_inventory(&self) -> LocalityOptionalObservationInventory {
        let observer = self.runtime.graph().observe();
        let graph = self.runtime.graph();
        LocalityOptionalObservationInventory {
            lineage_records: observer.lineage_records().len(),
            replay_events: observer.replay_events().len(),
            explanation_facts: self
                .handles
                .values()
                .filter(|node| observer.explanation_fact(**node).is_some())
                .count(),
            provenance_facts: self
                .handles
                .values()
                .filter(|node| observer.provenance_fact(**node).is_some())
                .count(),
            frontier_summary: observer.latest_frontier_execution_summary().is_some(),
            flow_summary: observer.latest_flow_diagnostics().is_some(),
            performed_counter_total: InvalidationPerformedCounter::ALL
                .into_iter()
                .map(|counter| graph.invalidation_performed_counters().value(counter))
                .sum(),
            performed_work_records: graph.invalidation_performed_work().len(),
            lineage_sequences: observer
                .lineage_records()
                .iter()
                .map(|record| record.sequence)
                .collect(),
        }
    }
}
