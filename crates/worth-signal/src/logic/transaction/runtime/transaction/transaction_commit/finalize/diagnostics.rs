use crate::diagnostics::recorder::record_transaction_semantic_event;

use super::super::super::transaction_types::{SignalTransaction, TransactionResult};
use super::boundary::CapturedFinalization;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn publish_finalization_diagnostics(
        &mut self,
        captured: CapturedFinalization,
    ) -> TransactionResult {
        let CapturedFinalization {
            result,
            failure,
            replay_events,
        } = captured;
        if let Some(rollback) = result.rollback.clone() {
            self.graph.diagnostics_state_mut().record_rollback(rollback);
        }
        if let Some(failure) = failure {
            self.graph.diagnostics_state_mut().record_failure(failure);
        }
        self.graph
            .diagnostics_state_mut()
            .attach_event_epochs_to_latest_flow(result.event_epochs.clone());
        self.graph
            .diagnostics_state_mut()
            .record_observation(result.observation.clone());
        for entry in replay_events {
            record_transaction_semantic_event(
                self.graph,
                entry.kind,
                entry.detail,
                entry.execution_record_id.map(|id| id.0),
                entry.semantic_segment_id.map(|id| id.0),
            );
        }
        result
    }
}
