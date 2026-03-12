use std::collections::VecDeque;

use crate::data::graph::SignalGraph;
use crate::diagnostics::compare::{
    explanations_semantically_equivalent, graphs_semantically_equivalent,
    plans_semantically_equivalent, repeat_run_summaries_equal, reports_semantically_equivalent,
    serial_parallel_reports_equivalent,
};
use crate::diagnostics::failure::{FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::history::{
    inspect_execution, inspect_flow, inspect_graph, inspect_plan, inspect_report,
    ExecutionInspector, FlowInspector, GraphInspector, PlanInspector, ReportInspector,
};
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::planner::{EvaluationPlan, ExecutionReport};
use crate::logic::transaction::SignalRuntime;

/// Public diagnostics facade over one committed signal graph.
pub struct GraphDiagnostics<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphDiagnostics<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        self.graph.observe().diagnostics_summary(profile)
    }

    pub fn history(&self, profile: DiagnosticsProfile) -> ExecutionHistorySummary {
        self.graph.observe().execution_history_summary(profile)
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
