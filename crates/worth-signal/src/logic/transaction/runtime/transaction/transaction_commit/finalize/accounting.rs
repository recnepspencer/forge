use super::super::super::transaction_types::{SignalTransaction, TransactionResult};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn finalize_result_accounting(&mut self, result: &mut TransactionResult) {
        self.with_telemetry(|telemetry| {
            telemetry.transaction.decision_log_event_count +=
                result.decision_log.records.len() as u64;
            telemetry.checkpoint.checkpoint_size += result.event_epochs.len() as u64
                + u64::from(result.integrity_markers.execution_report_attached)
                + u64::from(result.integrity_markers.rollback_attached)
                + u64::from(result.integrity_markers.failure_attached);
            telemetry.checkpoint.journal_replay_span +=
                result.reconstructability.journal.replay_event_count as u64;
        });
        result.reconstructability.checkpoint = self.checkpoint_record();
        result.performance_accounting = self.telemetry_snapshot();
    }
}
