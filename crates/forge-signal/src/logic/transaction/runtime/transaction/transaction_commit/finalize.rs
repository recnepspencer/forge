use crate::diagnostics::recorder::record_transaction_semantic_event;

use super::super::transaction_types::{
    SignalTransaction, TransactionOutcome, TransactionResult, TransactionTiming,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn finalize_semantic_delta(
        &mut self,
        restore_baseline: bool,
        outcome: TransactionOutcome,
        touched_nodes: u32,
        commit_nanos: u128,
    ) -> TransactionResult {
        let rollback = self.scratch.semantic_delta.rollback.take();
        let failure_summary = self.scratch.semantic_delta.failure_summary.take();
        let replay_events = std::mem::take(&mut self.scratch.semantic_delta.replay_events);
        let event_epochs = std::mem::take(&mut self.scratch.semantic_delta.event_epochs);
        let execution_report = self.execution_state.latest_report.take();
        let timing = TransactionTiming {
            total_nanos: self.started_at.elapsed().as_nanos(),
            evaluation_nanos: self.execution_state.evaluation_nanos,
            event_flush_nanos: self.scratch.staged_event_flush_nanos,
            commit_nanos,
        };
        let evaluation_summary = std::mem::take(&mut self.execution_state.summary);
        let checkpoint_record =
            crate::logic::transaction::runtime::state::CheckpointRecord::from_checkpoint_telemetry(
                crate::data::telemetry::CheckpointTelemetry {
                    event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                    event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                    checkpoint_flushes: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                },
            );
        let reconstructability =
            crate::logic::transaction::runtime::state::ReconstructabilityRecord::from_transaction_boundary(
                self.graph.current_branch().id,
                self.graph.current_branch().head_snapshot_id,
                self.graph.diagnostics_state().latest_replay_cursor(),
                checkpoint_record,
                &replay_events,
            );
        let mut result = TransactionResult::from_boundary_state(
            outcome,
            execution_report,
            timing,
            touched_nodes,
            evaluation_summary,
            &replay_events,
            reconstructability,
            event_epochs.clone(),
            rollback.clone(),
            failure_summary.clone(),
            *self.telemetry,
        );
        self.telemetry.transaction.decision_log_event_count +=
            result.decision_log.records.len() as u64;
        self.telemetry.checkpoint.checkpoint_size +=
            result.event_epochs.len() as u64
                + u64::from(result.integrity_markers.execution_report_attached)
                + u64::from(result.integrity_markers.rollback_attached)
                + u64::from(result.integrity_markers.failure_attached);
        self.telemetry.checkpoint.journal_replay_span += result
            .reconstructability
            .journal
            .as_ref()
            .map(|journal| journal.replay_event_count as u64)
            .unwrap_or(0);
        result.reconstructability.checkpoint =
            crate::logic::transaction::runtime::state::CheckpointRecord::from_checkpoint_telemetry(
                crate::data::telemetry::CheckpointTelemetry {
                    event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                    event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                    checkpoint_flushes: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                },
            );
        result.performance_accounting = *self.telemetry;
        if restore_baseline {
            *self.config = self.baseline_config.clone();
            *self.graph.diagnostics_state_mut() = self.baseline_diagnostics_state.clone();
        }
        if let Some(rollback) = rollback {
            self.graph.diagnostics_state_mut().record_rollback(rollback);
        }
        if let Some(failure) = failure_summary {
            self.graph.diagnostics_state_mut().record_failure(failure);
        }
        self.graph
            .diagnostics_state_mut()
            .attach_event_epochs_to_latest_flow(event_epochs);
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
