use crate::data::error::SignalError;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::transaction::runtime::transaction::transaction_types::{
    SignalTransaction, TransactionReplayEntry,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) fn record_failure_from_error(
        &mut self,
        phase: ExecutionFailurePhase,
        err: &SignalError,
        plan_summary: Option<crate::logic::planner::PlanSummary>,
    ) {
        if self.graph.captures_failure_diagnostics() {
            let summary = self
                .graph
                .observe()
                .latest_failure_diagnostics()
                .cloned()
                .unwrap_or_else(|| {
                    ExecutionFailureContext::from_error(phase, err, plan_summary)
                        .summarize(None, self.graph.diagnostics_profile())
                });
            self.scratch.semantic_delta.failure_summary = Some(summary);
        }
        if self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::ReplayDetail,
        ) {
            self.scratch
                .semantic_delta
                .replay_events
                .push(TransactionReplayEntry {
                    kind: ReplayEventKind::FailureRecorded,
                    detail: err.to_string(),
                    execution_record_id: None,
                    semantic_segment_id: None,
                });
        }
    }
}
