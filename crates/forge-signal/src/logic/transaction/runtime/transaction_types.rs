use std::collections::BTreeMap;

use crate::data::bitset::DenseBitset;
use crate::data::dirty_set::BatchedDirtySet;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::state::DiagnosticsState;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;

use super::super::key_registry::RuntimeStringId;
use super::super::patch_buffer::SparsePatchBuffer;
use super::config::SignalRuntimeConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionOutcome {
    Committed,
    RolledBack,
    Poisoned,
}

#[derive(Debug, Clone, Default)]
pub(super) struct TransactionSemanticDelta {
    pub failure_summary: Option<FailureSummary>,
    pub rollback: Option<crate::diagnostics::failure::RollbackDiagnostic>,
    pub replay_events: Vec<(ReplayEventKind, String, Option<u64>, Option<u64>)>,
}

pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) config: &'a mut SignalRuntimeConfig<T>,
    pub(super) graph: &'a mut crate::data::graph::SignalGraph,
    pub(super) checkpoint: &'a mut CheckpointRuntime<D, I>,
    pub(super) event_bus: &'a mut EventBus<E, D, Ctx>,
    pub(super) telemetry: &'a mut RuntimeTelemetry,
    pub(super) staged_dirty: BatchedDirtySet<D, I>,
    pub(super) staged_checkpoint_flushes: u64,
    pub(super) staged_checkpoint_flush_nanos: u128,
    pub(super) staged_events: Vec<E>,
    pub(super) staged_event_flushes: Vec<crate::data::checkpoint::CheckpointBarrier>,
    pub(super) staged_memo_writes: BTreeMap<
        (RuntimeStringId, RuntimeStringId, RuntimeStringId),
        crate::data::output::NodeEvaluationResult,
    >,
    pub(super) graph_patches: SparsePatchBuffer,
    pub(super) created_nodes: Vec<crate::data::handle::NodeId>,
    pub(super) baseline_config: SignalRuntimeConfig<T>,
    pub(super) baseline_diagnostics_state: DiagnosticsState,
    pub(super) semantic_delta: TransactionSemanticDelta,
    pub(super) mark_dirty_seen: DenseBitset,
    pub(super) evaluate_seen: DenseBitset,
    pub(super) dirty_targets: DenseBitset,
    pub(super) poisoned: bool,
    pub(super) finished: bool,
    pub(super) staged_patch_count: u64,
}
