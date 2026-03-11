use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::execution_flow::record_semantic_execution;
use crate::diagnostics::summary::EvaluationPlanSummary;

use super::types::{EvaluationPlan, EvaluationSession, ExecutionReport};

pub(crate) struct RecordedPlan {
    pub(crate) summary: EvaluationPlanSummary,
    pub(crate) first_target: Option<NodeId>,
}

impl RecordedPlan {
    pub(crate) fn from_plan(
        plan: &EvaluationPlan,
        profile: crate::diagnostics::profile::DiagnosticsProfile,
    ) -> Self {
        Self {
            summary: EvaluationPlanSummary::from_plan(plan, profile),
            first_target: plan.targets.first().copied(),
        }
    }

    pub(crate) fn from_session(
        session: &EvaluationSession<'_>,
        profile: crate::diagnostics::profile::DiagnosticsProfile,
    ) -> Self {
        Self {
            summary: EvaluationPlanSummary::from_session(session, profile),
            first_target: session.targets.first().copied(),
        }
    }
}

pub(crate) fn record_successful_execution(
    graph: &mut SignalGraph,
    plan: RecordedPlan,
    report: &ExecutionReport,
) {
    record_semantic_execution(graph, plan.summary, plan.first_target, report);
}
