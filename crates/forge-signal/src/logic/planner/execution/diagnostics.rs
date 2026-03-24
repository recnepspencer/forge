use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::execution_flow::record_semantic_execution;
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::summary::EvaluationPlanSummary;

use super::super::types::{EvaluationPlan, ExecutionReport, SessionScratch};
pub(crate) fn summarize_recorded_plan(
    plan: &EvaluationPlan,
    profile: DiagnosticsTier,
) -> (EvaluationPlanSummary, Option<NodeId>) {
    (
        EvaluationPlanSummary::from_plan(plan, profile),
        plan.targets.first().copied(),
    )
}

pub(crate) fn summarize_recorded_session(
    session: &SessionScratch<'_>,
    profile: DiagnosticsTier,
) -> (EvaluationPlanSummary, Option<NodeId>) {
    (
        EvaluationPlanSummary::from_session(session, profile),
        session.targets.first().copied(),
    )
}

pub(crate) fn record_successful_execution(
    graph: &mut SignalGraph,
    plan_summary: EvaluationPlanSummary,
    first_target: Option<NodeId>,
    report: &ExecutionReport,
) {
    record_semantic_execution(graph, plan_summary, first_target, report);
}
