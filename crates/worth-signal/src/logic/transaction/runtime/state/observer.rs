use crate::data::telemetry::CheckpointTelemetry;

use super::runtime_state::SignalRuntime;

mod diagnostics;
mod materialized;
mod metrics;
mod ordinary;

pub struct RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a SignalRuntime<D, I, E, Ctx, T>,
}

pub struct RuntimeMaterializer<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a SignalRuntime<D, I, E, Ctx, T>,
}

impl<'a, D, I, E, Ctx, T> RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self { runtime }
    }

    pub fn materialize(&self) -> RuntimeMaterializer<'a, D, I, E, Ctx, T> {
        RuntimeMaterializer {
            runtime: self.runtime,
        }
    }

    fn composed_checkpoint_telemetry(&self) -> CheckpointTelemetry {
        CheckpointTelemetry {
            event_flushes: self.runtime.event_bus.telemetry().checkpoint.event_flushes,
            event_flush_nanos: self
                .runtime
                .event_bus
                .telemetry()
                .checkpoint
                .event_flush_nanos,
            checkpoint_flushes: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flushes,
            checkpoint_flush_nanos: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flush_nanos,
            rollback_count: self.runtime.event_bus.telemetry().checkpoint.rollback_count,
            snapshot_restore_count: self.runtime.telemetry.checkpoint.snapshot_restore_count,
            snapshot_restore_apply_active_policy_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_apply_active_policy_count,
            snapshot_restore_shared_delta_node_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_shared_delta_node_count,
            snapshot_restore_coarse_reason_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_coarse_reason_count,
            checkpoint_size: self.runtime.telemetry.checkpoint.checkpoint_size,
            journal_replay_span: self.runtime.telemetry.checkpoint.journal_replay_span,
            restore_authority_breadth: self.runtime.telemetry.checkpoint.restore_authority_breadth,
            restore_required_derived_breadth: self
                .runtime
                .telemetry
                .checkpoint
                .restore_required_derived_breadth,
            restore_diagnostic_richness_breadth: self
                .runtime
                .telemetry
                .checkpoint
                .restore_diagnostic_richness_breadth,
        }
    }
}
