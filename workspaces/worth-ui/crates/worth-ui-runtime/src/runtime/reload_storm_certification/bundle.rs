use crate::runtime::{
    WorthUiFoundationalCounterEvidence, WorthUiReloadLatencyCounters,
    WorthUiReloadStormIterationOutcome,
};
use worth_foundational::performance_api::lower_lane::basis::{
    compare_performance_bundles, FoundationalPerformanceComparison,
};

#[derive(Debug, PartialEq)]
pub struct WorthUiReloadCertificationBundle {
    iteration_outcomes: Vec<WorthUiReloadStormIterationOutcome>,
    foundational_evidence: Vec<WorthUiFoundationalCounterEvidence>,
    counters: WorthUiReloadLatencyCounters,
}

impl WorthUiReloadCertificationBundle {
    pub(crate) fn new(
        iteration_outcomes: Vec<WorthUiReloadStormIterationOutcome>,
        foundational_evidence: Vec<WorthUiFoundationalCounterEvidence>,
        counters: WorthUiReloadLatencyCounters,
    ) -> Self {
        Self {
            iteration_outcomes,
            foundational_evidence,
            counters,
        }
    }

    pub fn iteration_outcomes(&self) -> &[WorthUiReloadStormIterationOutcome] {
        &self.iteration_outcomes
    }

    pub fn foundational_evidence(&self) -> &[WorthUiFoundationalCounterEvidence] {
        &self.foundational_evidence
    }

    pub fn counters(&self) -> WorthUiReloadLatencyCounters {
        self.counters
    }

    pub fn foundational_receipt_count(&self) -> usize {
        self.foundational_evidence.len()
    }

    pub fn foundational_meaning_digest(&self) -> u64 {
        let mut entries = Vec::new();
        for evidence in &self.foundational_evidence {
            entries.push(format!("basis:{}", evidence.canonical_basis_entry_count()));
            entries.push(format!("replay:{}", evidence.worth_ui_replay_digest()));
            for spec in evidence.counter_specs() {
                entries.push(format!("spec:{spec:?}"));
            }
            for row in evidence.counter_rows() {
                entries.push(format!("row:{row:?}"));
            }
        }
        super::digest::fold_texts(entries)
    }

    pub fn compare_foundational_bundle_meaning(
        &self,
        other: &Self,
    ) -> Option<FoundationalPerformanceComparison> {
        let left = self.foundational_evidence.first()?;
        let right = other.foundational_evidence.first()?;
        Some(compare_performance_bundles(
            left.counter_backed_receipt().bundle(),
            right.counter_backed_receipt().bundle(),
        ))
    }
}
