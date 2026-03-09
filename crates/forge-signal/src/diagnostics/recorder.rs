use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary};
use crate::diagnostics::flow::{
    ApplySummary, ChangeInputSummary, FlowSummary, InvalidationSummary, PlanningSummary,
    PrecomputeSummary,
};
use crate::diagnostics::policy::DiagnosticsPolicy;
use crate::diagnostics::replay::{ReplayEvent, ReplayEventKind};
use crate::diagnostics::summary::{ExecutionHistorySummary, ExplanationSummary};
use crate::logic::planner::{EvaluationPlan, ExecutionReport, TaskExecutionOutcome};

pub struct DiagnosticsRecorder<'a> {
    graph: &'a mut SignalGraph,
}

impl<'a> DiagnosticsRecorder<'a> {
    pub fn new(graph: &'a mut SignalGraph) -> Self {
        Self { graph }
    }

    fn policy(&self) -> DiagnosticsPolicy {
        DiagnosticsPolicy::from_profile(self.graph.diagnostics_profile())
    }

    pub fn record_failure(&mut self, context: ExecutionFailureContext) -> FailureSummary {
        let policy = self.policy();
        let summary = context.summarize(self.graph.latest_rollback_diagnostics(), policy.profile);
        self.record_failure_summary(summary.clone());
        self.graph.clear_pending_diagnostics_input();
        summary
    }

    pub fn record_failure_summary(&mut self, summary: FailureSummary) {
        self.graph.diagnostics_state_mut().record_failure(summary);
    }
}

pub fn record_semantic_execution(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    report: &ExecutionReport,
) {
    let profile = DiagnosticsPolicy::from_profile(graph.diagnostics_profile()).profile;
    let (change, invalidation) = graph
        .diagnostics_state()
        .pending_change_summary()
        .unwrap_or_else(|| {
            (
                ChangeInputSummary::new(Vec::new(), Vec::new(), 0, None),
                InvalidationSummary::new(0, 0, 0),
            )
        });
    let explanation = if DiagnosticsPolicy::from_profile(profile).retain_flow_explanation {
        plan.targets
            .first()
            .and_then(|target| graph.explain(*target).ok())
            .map(|explanation| ExplanationSummary::from_explanation(&explanation, profile))
    } else {
        None
    };
    let flow = FlowSummary::new(
        profile,
        change,
        invalidation,
        PlanningSummary::from_plan(plan, profile),
        PrecomputeSummary::from_report(report, profile),
        ApplySummary::from_report(report, profile),
        None,
        explanation,
    );
    let history = if execution_history_unchanged(report) {
        graph
            .diagnostics_state()
            .recent_history()
            .back()
            .cloned()
            .unwrap_or_else(|| ExecutionHistorySummary::from_graph(graph, profile))
    } else {
        ExecutionHistorySummary::from_graph(graph, profile)
    };
    graph.diagnostics_state_mut().complete_flow(flow, history);
    for task in report
        .stages
        .iter()
        .flat_map(|stage| stage.task_records.iter())
    {
        let sequence = graph.diagnostics_state_mut().allocate_replay_sequence();
        graph
            .diagnostics_state_mut()
            .record_replay_event(ReplayEvent::new(
                sequence,
                ReplayEventKind::TaskApplied,
                Some(task.node),
                Some(task.id.0),
                Some(task.semantic_segment_id.0),
                Some(format!("{:?}", task.outcome)),
            ));
    }
}

pub fn record_transaction_semantic_event(
    graph: &mut SignalGraph,
    kind: ReplayEventKind,
    detail: impl Into<String>,
    execution_record_id: Option<u64>,
    semantic_segment_id: Option<u64>,
) {
    let sequence = graph.diagnostics_state_mut().allocate_replay_sequence();
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            sequence,
            kind,
            None,
            execution_record_id,
            semantic_segment_id,
            Some(detail.into()),
        ));
}

fn execution_history_unchanged(report: &ExecutionReport) -> bool {
    report
        .stages
        .iter()
        .flat_map(|stage| &stage.task_records)
        .all(|task| {
            matches!(
                task.outcome,
                TaskExecutionOutcome::ValidatedClean
                    | TaskExecutionOutcome::ConditionDeferred
                    | TaskExecutionOutcome::ConditionRevertedClean
                    | TaskExecutionOutcome::Pruned
            )
        })
}
