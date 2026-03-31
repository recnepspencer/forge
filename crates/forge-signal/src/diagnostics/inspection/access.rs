use std::collections::VecDeque;

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::diagnostics::compare::{
    explanations_semantically_equivalent, graphs_semantically_equivalent,
    plans_semantically_equivalent, repeat_run_summaries_equal, reports_semantically_equivalent,
    serial_parallel_reports_equivalent,
};
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::history::{
    inspect_execution, inspect_flow, inspect_graph, inspect_plan, inspect_report,
    ExecutionInspector, FlowInspector, GraphInspector, PlanInspector, ReportInspector,
};
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::explain::NodeExplanation;
use crate::logic::planner::{EvaluationPlan, ExecutionReport};
use crate::logic::transaction::SignalRuntime;

/// Public diagnostics facade over one committed signal graph.
pub struct GraphDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphComparisonDiagnostics;

pub struct GraphHealthDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphInspectDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphForensicDiagnostics<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphDiagnostics<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn forensic(&self) -> GraphForensicDiagnostics<'a> {
        GraphForensicDiagnostics { graph: self.graph }
    }

    pub fn compare(&self) -> GraphComparisonDiagnostics {
        GraphComparisonDiagnostics
    }

    /// Group health-oriented reads in one place.
    pub fn health_view(&self) -> GraphHealthDiagnostics<'a> {
        GraphHealthDiagnostics { graph: self.graph }
    }

    /// Group inspector-style reads in one place.
    pub fn inspect(&self) -> GraphInspectDiagnostics<'a> {
        GraphInspectDiagnostics { graph: self.graph }
    }

    pub fn explain(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, crate::data::error::SignalError> {
        self.graph.observe().explain(node)
    }

    pub fn why(&self, node: NodeId) -> Result<NodeExplanation, crate::data::error::SignalError> {
        self.explain(node)
    }

    pub fn summary(&self, profile: DiagnosticsTier) -> GraphSummary {
        self.graph.observe().diagnostics_summary(profile)
    }

    pub fn summary_now(&self) -> GraphSummary {
        self.summary(self.graph.runtime_policy().tier)
    }

    pub fn history(&self, profile: DiagnosticsTier) -> ExecutionHistorySummary {
        self.graph.observe().execution_history_summary(profile)
    }

    pub fn history_now(&self) -> ExecutionHistorySummary {
        self.history(self.graph.runtime_policy().tier)
    }

    pub fn health(&self, profile: DiagnosticsTier) -> GraphSummary {
        self.summary(profile)
    }

    pub fn health_now(&self) -> GraphSummary {
        self.summary_now()
    }

    pub fn latest_flow(&self) -> Option<&'a FlowSummary> {
        self.graph.observe().latest_flow_diagnostics()
    }

    pub fn latest_failure(&self) -> Option<&'a FailureSummary> {
        self.graph.observe().latest_failure_diagnostics()
    }

    pub fn latest_rollback(&self) -> Option<&'a RollbackDiagnostic> {
        self.graph.observe().latest_rollback_diagnostics()
    }

    pub fn latest_frontier_execution(&self) -> Option<&'a FrontierExecutionSummary> {
        self.graph.observe().latest_frontier_execution_summary()
    }

    pub fn latest_invalidation_trace_records(&self) -> &'a [InvalidationTraceRecord] {
        self.graph.observe().latest_invalidation_trace_records()
    }

    pub fn recent_history(&self) -> &'a VecDeque<ExecutionHistorySummary> {
        self.graph.observe().recent_execution_history_diagnostics()
    }

    pub fn inspect_graph(&self) -> GraphInspector<'a> {
        inspect_graph(self.graph)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'a> {
        inspect_execution(self.graph)
    }

    pub fn inspect_plan(&self, plan: &'a EvaluationPlan) -> PlanInspector<'a> {
        inspect_plan(plan)
    }

    pub fn inspect_report(&self, report: &'a ExecutionReport) -> ReportInspector<'a> {
        inspect_report(report)
    }

    pub fn inspect_flow(&self, flow: &'a FlowSummary) -> FlowInspector<'a> {
        inspect_flow(flow)
    }

    pub fn graphs_semantically_equivalent(
        &self,
        left: &GraphSummary,
        right: &GraphSummary,
    ) -> bool {
        graphs_semantically_equivalent(left, right)
    }

    pub fn plans_semantically_equivalent(
        &self,
        left: &crate::diagnostics::summary::EvaluationPlanSummary,
        right: &crate::diagnostics::summary::EvaluationPlanSummary,
    ) -> bool {
        plans_semantically_equivalent(left, right)
    }

    pub fn reports_semantically_equivalent(
        &self,
        left: &crate::diagnostics::summary::ExecutionReportSummary,
        right: &crate::diagnostics::summary::ExecutionReportSummary,
    ) -> bool {
        reports_semantically_equivalent(left, right)
    }

    pub fn explanations_semantically_equivalent(
        &self,
        left: &crate::diagnostics::summary::ExplanationSummary,
        right: &crate::diagnostics::summary::ExplanationSummary,
    ) -> bool {
        explanations_semantically_equivalent(left, right)
    }

    pub fn serial_parallel_reports_equivalent(
        &self,
        left: &crate::diagnostics::summary::ExecutionReportSummary,
        right: &crate::diagnostics::summary::ExecutionReportSummary,
    ) -> bool {
        serial_parallel_reports_equivalent(left, right)
    }

    pub fn repeat_run_summaries_equal<T>(&self, summaries: &[T]) -> bool
    where
        T: PartialEq,
    {
        repeat_run_summaries_equal(summaries)
    }
}

impl<'a> GraphHealthDiagnostics<'a> {
    pub fn summary(&self, profile: DiagnosticsTier) -> GraphSummary {
        self.graph.observe().diagnostics_summary(profile)
    }

    pub fn summary_now(&self) -> GraphSummary {
        self.summary(self.graph.runtime_policy().tier)
    }

    pub fn current(&self, profile: DiagnosticsTier) -> GraphSummary {
        self.summary(profile)
    }

    pub fn current_now(&self) -> GraphSummary {
        self.summary_now()
    }

    pub fn latest_flow(&self) -> Option<&'a FlowSummary> {
        self.graph.observe().latest_flow_diagnostics()
    }

    pub fn latest_failure(&self) -> Option<&'a FailureSummary> {
        self.graph.observe().latest_failure_diagnostics()
    }

    pub fn latest_rollback(&self) -> Option<&'a RollbackDiagnostic> {
        self.graph.observe().latest_rollback_diagnostics()
    }

    pub fn latest_frontier_execution(&self) -> Option<&'a FrontierExecutionSummary> {
        self.graph.observe().latest_frontier_execution_summary()
    }

    pub fn latest_invalidation_trace_records(&self) -> &'a [InvalidationTraceRecord] {
        self.graph.observe().latest_invalidation_trace_records()
    }

    pub fn recent_history(&self) -> &'a VecDeque<ExecutionHistorySummary> {
        self.graph.observe().recent_execution_history_diagnostics()
    }
}

impl<'a> GraphInspectDiagnostics<'a> {
    pub fn graph(&self) -> GraphInspector<'a> {
        inspect_graph(self.graph)
    }

    pub fn execution(&self) -> ExecutionInspector<'a> {
        inspect_execution(self.graph)
    }

    pub fn plan(&self, plan: &'a EvaluationPlan) -> PlanInspector<'a> {
        inspect_plan(plan)
    }

    pub fn report(&self, report: &'a ExecutionReport) -> ReportInspector<'a> {
        inspect_report(report)
    }

    pub fn flow(&self, flow: &'a FlowSummary) -> FlowInspector<'a> {
        inspect_flow(flow)
    }
}

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
        left: &crate::diagnostics::EvaluationPlanSummary,
        right: &crate::diagnostics::EvaluationPlanSummary,
    ) -> crate::diagnostics::PlanDiff {
        crate::diagnostics::compare_plans(left, right)
    }

    pub fn reports(
        &self,
        left: &crate::diagnostics::ExecutionReportSummary,
        right: &crate::diagnostics::ExecutionReportSummary,
    ) -> crate::diagnostics::ExecutionReportDiff {
        crate::diagnostics::compare_execution_reports(left, right)
    }

    pub fn explanations(
        &self,
        left: &crate::diagnostics::ExplanationSummary,
        right: &crate::diagnostics::ExplanationSummary,
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

impl<'a> GraphForensicDiagnostics<'a> {
    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph
            .observe()
            .materialize()
            .retained_explanation_artifact(node)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, crate::data::error::SignalError> {
        self.graph
            .observe()
            .materialize()
            .reconstruct_explanation_artifact(node)
    }

    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), crate::data::error::SignalError>
    {
        self.graph
            .observe()
            .materialize()
            .materialize_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph
            .observe()
            .materialize()
            .retained_provenance_artifact(node)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, crate::data::error::SignalError> {
        self.graph
            .observe()
            .materialize()
            .reconstruct_provenance_artifact(node)
    }

    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), crate::data::error::SignalError>
    {
        self.graph
            .observe()
            .materialize()
            .materialize_provenance_artifact(node)
    }
}

pub type RuntimeDiagnostics<'a> = GraphDiagnostics<'a>;

pub fn diagnostics_for_graph(graph: &SignalGraph) -> GraphDiagnostics<'_> {
    GraphDiagnostics::new(graph)
}

pub fn diagnostics_for_runtime<D, I, E, Ctx, T>(
    runtime: &SignalRuntime<D, I, E, Ctx, T>,
) -> RuntimeDiagnostics<'_>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    GraphDiagnostics::new(runtime.graph())
}
