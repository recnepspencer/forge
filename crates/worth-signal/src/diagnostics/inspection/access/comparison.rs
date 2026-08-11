use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExecutionReportSummary, ExplanationSummary,
    GraphSummary,
};

use super::GraphComparisonDiagnostics;

impl GraphComparisonDiagnostics {
    pub fn graphs(
        &self,
        left: &GraphSummary,
        right: &GraphSummary,
    ) -> crate::diagnostics::GraphDiff {
        crate::diagnostics::compare_graphs(left, right)
    }

    pub fn plans(
        &self,
        left: &EvaluationPlanSummary,
        right: &EvaluationPlanSummary,
    ) -> crate::diagnostics::PlanDiff {
        crate::diagnostics::compare_plans(left, right)
    }

    pub fn reports(
        &self,
        left: &ExecutionReportSummary,
        right: &ExecutionReportSummary,
    ) -> crate::diagnostics::ExecutionReportDiff {
        crate::diagnostics::compare_execution_reports(left, right)
    }

    pub fn explanations(
        &self,
        left: &ExplanationSummary,
        right: &ExplanationSummary,
    ) -> crate::diagnostics::ExplanationDiff {
        crate::diagnostics::compare_explanations(left, right)
    }

    pub fn histories(
        &self,
        left: &ExecutionHistorySummary,
        right: &ExecutionHistorySummary,
    ) -> crate::diagnostics::HistoryDiff {
        crate::diagnostics::compare_execution_history(left, right)
    }

    pub fn flows(&self, left: &FlowSummary, right: &FlowSummary) -> crate::diagnostics::FlowDiff {
        crate::diagnostics::compare_flows(left, right)
    }

    pub fn failures(
        &self,
        left: &FailureSummary,
        right: &FailureSummary,
    ) -> crate::diagnostics::FailureDiff {
        crate::diagnostics::compare_failures(left, right)
    }

    pub fn replay(
        &self,
        left: &crate::diagnostics::ReplayView,
        right: &crate::diagnostics::ReplayView,
    ) -> crate::diagnostics::ReplayDiff {
        crate::diagnostics::compare_replay_slices(left, right)
    }

    pub fn lineage(
        &self,
        left: &[crate::diagnostics::LineageEvent],
        right: &[crate::diagnostics::LineageEvent],
    ) -> crate::diagnostics::LineageDiff {
        crate::diagnostics::compare_lineage_records(left, right)
    }
}
