use crate::data::telemetry::CheckpointTelemetry;
use crate::logic::transaction::runtime::state::{CheckpointRecord, ReconstructabilityRecord};

use super::super::super::transaction_types::{SignalTransaction, TransactionReplayEntry};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn checkpoint_record(&self) -> CheckpointRecord {
        CheckpointRecord::from_checkpoint_telemetry(self.checkpoint_telemetry())
    }

    pub(super) fn boundary_reconstructability(
        &self,
        replay_events: &[TransactionReplayEntry],
    ) -> ReconstructabilityRecord {
        let temporal_evidence = self
            .scratch
            .temporal
            .boundary_evidence(self.temporal.clock_basis());
        let temporal = crate::logic::transaction::runtime::state::TemporalReconstructabilityArtifact::from_evidence(
            self.temporal.wake_summary(),
            &temporal_evidence,
        );
        ReconstructabilityRecord::from_transaction_boundary(
            self.graph.current_branch().id,
            self.graph.current_branch().head_snapshot_id,
            self.graph.diagnostics_state().latest_replay_cursor(),
            self.checkpoint_record(),
            replay_events,
            temporal,
        )
    }

    fn checkpoint_telemetry(&self) -> CheckpointTelemetry {
        let telemetry = self.telemetry_snapshot();
        CheckpointTelemetry {
            event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
            event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
            checkpoint_flushes: self.checkpoint.telemetry().checkpoint.checkpoint_flushes,
            checkpoint_flush_nanos: self
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flush_nanos,
            rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
            snapshot_restore_count: telemetry.checkpoint.snapshot_restore_count,
            snapshot_restore_apply_active_policy_count: telemetry
                .checkpoint
                .snapshot_restore_apply_active_policy_count,
            snapshot_restore_shared_delta_node_count: telemetry
                .checkpoint
                .snapshot_restore_shared_delta_node_count,
            snapshot_restore_coarse_reason_count: telemetry
                .checkpoint
                .snapshot_restore_coarse_reason_count,
            checkpoint_size: telemetry.checkpoint.checkpoint_size,
            journal_replay_span: telemetry.checkpoint.journal_replay_span,
            restore_authority_breadth: telemetry.checkpoint.restore_authority_breadth,
            restore_required_derived_breadth: telemetry.checkpoint.restore_required_derived_breadth,
            restore_diagnostic_richness_breadth: telemetry
                .checkpoint
                .restore_diagnostic_richness_breadth,
        }
    }
}
