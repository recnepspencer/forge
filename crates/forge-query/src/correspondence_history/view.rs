use crate::correspondence::{
    CorrespondenceCostPosture, CorrespondenceCounterSnapshot, CorrespondenceEvidenceResolved,
};
use crate::execution::{ExecutionCounters, ExecutionResultEnvelope};
use crate::historical::{
    HistoricalCounterSnapshot, HistoricalMaterializationPathMetadata, HistoricalPathCostPosture,
    HistoricalPathResolved,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataPreservingHistoricalResultView<'a> {
    rows: &'a [String],
    execution_counters: &'a ExecutionCounters,
    correspondence_family_name: &'static str,
    correspondence_cost_posture: &'a CorrespondenceCostPosture,
    correspondence_counters: &'a CorrespondenceCounterSnapshot,
    materialization_metadata: &'a HistoricalMaterializationPathMetadata,
    historical_cost_posture: &'a HistoricalPathCostPosture,
    historical_counters: &'a HistoricalCounterSnapshot,
}

impl<'a> MetadataPreservingHistoricalResultView<'a> {
    pub fn rows(&self) -> &[String] {
        self.rows
    }

    pub fn execution_counters(&self) -> &ExecutionCounters {
        self.execution_counters
    }

    pub fn correspondence_family_name(&self) -> &'static str {
        self.correspondence_family_name
    }

    pub fn correspondence_cost_posture(&self) -> &CorrespondenceCostPosture {
        self.correspondence_cost_posture
    }

    pub fn correspondence_counters(&self) -> &CorrespondenceCounterSnapshot {
        self.correspondence_counters
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        self.materialization_metadata
    }

    pub fn historical_cost_posture(&self) -> &HistoricalPathCostPosture {
        self.historical_cost_posture
    }

    pub fn historical_counters(&self) -> &HistoricalCounterSnapshot {
        self.historical_counters
    }
}

pub(crate) fn build_result_view<'a>(
    execution: &'a ExecutionResultEnvelope,
    correspondence: &'a CorrespondenceEvidenceResolved,
    materialization_metadata: &'a HistoricalMaterializationPathMetadata,
    historical: &'a HistoricalPathResolved,
) -> MetadataPreservingHistoricalResultView<'a> {
    MetadataPreservingHistoricalResultView {
        rows: execution.rows(),
        execution_counters: execution.counters(),
        correspondence_family_name: correspondence.outcome().family_name(),
        correspondence_cost_posture: correspondence.cost_posture(),
        correspondence_counters: correspondence.counters(),
        materialization_metadata,
        historical_cost_posture: historical.cost_posture(),
        historical_counters: historical.counters(),
    }
}
