use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary, RollbackDiagnostic};
use crate::diagnostics::flow::{
    ApplySummary, ChangeInputSummary, FlowSummary, InvalidationSummary, PlanningSummary,
    PrecomputeSummary,
};
use crate::diagnostics::policy::DiagnosticsPolicy;
use crate::diagnostics::state::DiagnosticsState;
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

    pub fn note_change_input(
        &mut self,
        node: crate::data::handle::NodeId,
        aspect: crate::data::aspect::Aspect,
        changed_regions: &[crate::data::output::ChangedRegion],
    ) {
        let causality_kind = self
            .graph
            .get_entry(node)
            .ok()
            .and_then(|entry| entry.get_causality())
            .map(|causality| causality.kind.clone());
        self.graph.diagnostics_state_mut().note_change_input(
            node,
            aspect,
            changed_regions,
            causality_kind,
        );
    }

    pub fn record_invalidation_result(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) {
        self.graph
            .diagnostics_state_mut()
            .record_invalidation_result(
                invalidated_direct_subscribers,
                maybe_stale_direct_subscribers,
                partition_scoped_checks,
            );
    }

    pub fn record_execution_completed(&mut self, plan: &EvaluationPlan, report: &ExecutionReport) {
        let policy = self.policy();
        let (change, invalidation) = self
            .graph
            .diagnostics_state()
            .pending_change_summary()
            .unwrap_or_else(|| {
                (
                    ChangeInputSummary::new(Vec::new(), Vec::new(), 0, None),
                    InvalidationSummary::new(0, 0, 0),
                )
            });
        let explanation = if policy.retain_flow_explanation {
            plan.targets
                .first()
                .and_then(|target| self.graph.explain(*target).ok())
                .map(|explanation| {
                    ExplanationSummary::from_explanation(&explanation, policy.profile)
                })
        } else {
            None
        };
        let flow = FlowSummary::new(
            policy.profile,
            change,
            invalidation,
            PlanningSummary::from_plan(plan, policy.profile),
            PrecomputeSummary::from_report(report, policy.profile),
            ApplySummary::from_report(report, policy.profile),
            None,
            explanation,
        );
        let history = if execution_history_unchanged(report) {
            self.graph
                .diagnostics_state()
                .recent_history()
                .back()
                .cloned()
                .unwrap_or_else(|| ExecutionHistorySummary::from_graph(self.graph, policy.profile))
        } else {
            ExecutionHistorySummary::from_graph(self.graph, policy.profile)
        };
        self.graph
            .diagnostics_state_mut()
            .complete_flow(flow, history);
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

    pub fn record_rollback(&mut self, rollback: RollbackDiagnostic) {
        self.graph.diagnostics_state_mut().record_rollback(rollback);
    }

    pub fn restore_snapshot(&mut self, snapshot: DiagnosticsState) {
        *self.graph.diagnostics_state_mut() = snapshot;
    }
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
