use crate::diagnostics::recorder::record_transaction_semantic_event;
use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};

use super::super::transaction_types::{
    SignalTransaction, TransactionOutcome, TransactionResult, TransactionTiming,
};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn apply_rollback_packets(&mut self) -> Result<(), crate::data::error::SignalError> {
        let rollback_packets = self.rollback_packets.drain_ordered();
        self.telemetry.transaction.rollback_packet_breadth += rollback_packets.len() as u64;
        for packet in rollback_packets {
            match packet {
                crate::logic::transaction::runtime::transaction::TransactionRollbackPacket::Config(
                    delta,
                ) => {
                    self.telemetry.transaction.rollback_packet_config_count += 1;
                    *self.config = delta.baseline;
                }
                crate::logic::transaction::runtime::transaction::TransactionRollbackPacket::DiagnosticsRequired(
                    delta,
                ) => {
                    self.telemetry.transaction.rollback_packet_diagnostics_count += 1;
                    *self.graph.diagnostics_state_mut() = delta.baseline;
                }
                crate::logic::transaction::runtime::transaction::TransactionRollbackPacket::GraphPatches(
                    delta,
                ) => {
                    self.telemetry.transaction.rollback_packet_graph_patch_count += 1;
                    delta.patches.rollback_from_packet(self.graph)?;
                }
                crate::logic::transaction::runtime::transaction::TransactionRollbackPacket::CreatedNodes(
                    delta,
                ) => {
                    self.telemetry.transaction.rollback_packet_created_node_count += 1;
                    self.graph.rollback_created_nodes(&delta.created_nodes);
                }
                crate::logic::transaction::runtime::transaction::TransactionRollbackPacket::SubscriberRepair(
                    delta,
                ) => {
                    self.telemetry.transaction.rollback_packet_subscriber_repair_count += 1;
                    self.graph
                        .reconcile_subscriber_membership_for_sources(&delta.sources)?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn finalize_semantic_delta(
        &mut self,
        restore_baseline: bool,
        outcome: TransactionOutcome,
        touched_nodes: u32,
        commit_nanos: u128,
    ) -> TransactionResult {
        let mut outcome = outcome;
        if restore_baseline {
            if let Err(error) = self.apply_rollback_packets() {
                self.telemetry.transaction.transaction_poison_count += 1;
                outcome = TransactionOutcome::Poisoned;
                self.scratch.semantic_delta.failure_summary = Some(
                    ExecutionFailureContext::from_error(
                        ExecutionFailurePhase::Rollback,
                        &error,
                        None,
                    )
                    .summarize(
                        self.scratch.semantic_delta.rollback.as_ref(),
                        self.graph.diagnostics_profile(),
                    ),
                );
                self.scratch.semantic_delta.replay_events.push(
                    crate::logic::transaction::runtime::transaction::TransactionReplayEntry {
                        kind: crate::diagnostics::replay::ReplayEventKind::FailureRecorded,
                        detail: error.to_string(),
                        execution_record_id: None,
                        semantic_segment_id: None,
                    },
                );
            }
        }
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
                    checkpoint_flushes: self.checkpoint.telemetry().checkpoint.checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    snapshot_restore_count: self.telemetry.checkpoint.snapshot_restore_count,
                    snapshot_restore_apply_active_policy_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_apply_active_policy_count,
                    snapshot_restore_shared_delta_node_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_shared_delta_node_count,
                    snapshot_restore_coarse_reason_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_coarse_reason_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                    restore_authority_breadth: self.telemetry.checkpoint.restore_authority_breadth,
                    restore_required_derived_breadth: self
                        .telemetry
                        .checkpoint
                        .restore_required_derived_breadth,
                    restore_diagnostic_richness_breadth: self
                        .telemetry
                        .checkpoint
                        .restore_diagnostic_richness_breadth,
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
        self.telemetry.checkpoint.checkpoint_size += result.event_epochs.len() as u64
            + u64::from(result.integrity_markers.execution_report_attached)
            + u64::from(result.integrity_markers.rollback_attached)
            + u64::from(result.integrity_markers.failure_attached);
        self.telemetry.checkpoint.journal_replay_span +=
            result.reconstructability.journal.replay_event_count as u64;
        result.reconstructability.checkpoint =
            crate::logic::transaction::runtime::state::CheckpointRecord::from_checkpoint_telemetry(
                crate::data::telemetry::CheckpointTelemetry {
                    event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                    event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                    checkpoint_flushes: self.checkpoint.telemetry().checkpoint.checkpoint_flushes,
                    checkpoint_flush_nanos: self
                        .checkpoint
                        .telemetry()
                        .checkpoint
                        .checkpoint_flush_nanos,
                    rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                    snapshot_restore_count: self.telemetry.checkpoint.snapshot_restore_count,
                    snapshot_restore_apply_active_policy_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_apply_active_policy_count,
                    snapshot_restore_shared_delta_node_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_shared_delta_node_count,
                    snapshot_restore_coarse_reason_count: self
                        .telemetry
                        .checkpoint
                        .snapshot_restore_coarse_reason_count,
                    checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                    journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                    restore_authority_breadth: self.telemetry.checkpoint.restore_authority_breadth,
                    restore_required_derived_breadth: self
                        .telemetry
                        .checkpoint
                        .restore_required_derived_breadth,
                    restore_diagnostic_richness_breadth: self
                        .telemetry
                        .checkpoint
                        .restore_diagnostic_richness_breadth,
                },
            );
        result.performance_accounting = *self.telemetry;
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
