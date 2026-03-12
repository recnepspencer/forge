use std::collections::BTreeMap;
use std::time::Instant;

use crate::data::bitset::DenseBitset;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::BatchedDirtySet;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::state::DiagnosticsState;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::logic::planner::{ExecutionRecordId, ExecutionReport, SemanticSegmentId};
use crate::logic::evaluation::EvaluationVerdict;

use super::super::super::key_registry::RuntimeStringId;
use super::super::super::patch_buffer::SparsePatchBuffer;
use super::super::config::SignalRuntimeConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionOutcome {
    Committed,
    RolledBack,
    Poisoned,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionTiming {
    pub total_nanos: u128,
    pub evaluation_nanos: u128,
    pub event_flush_nanos: u128,
    pub commit_nanos: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EvaluationSummary {
    pub nodes_evaluated: u32,
    pub nodes_recomputed: u32,
    pub nodes_suppressed: u32,
    pub plans_built: u32,
    pub stages_executed: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionResult {
    pub outcome: TransactionOutcome,
    pub execution_report: Option<ExecutionReport>,
    pub timing: TransactionTiming,
    pub touched_nodes: u32,
    pub evaluation_summary: EvaluationSummary,
    pub event_epochs: Vec<EventEpochSummary>,
    pub rollback: Option<crate::diagnostics::failure::RollbackDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionReplayEntry {
    pub kind: ReplayEventKind,
    pub detail: String,
    pub execution_record_id: Option<ExecutionRecordId>,
    pub semantic_segment_id: Option<SemanticSegmentId>,
}

#[derive(Debug, Clone, Default)]
pub(in crate::logic::transaction::runtime) struct TransactionExecutionState {
    pub latest_report: Option<ExecutionReport>,
    pub summary: EvaluationSummary,
    pub evaluation_nanos: u128,
}

impl TransactionExecutionState {
    pub fn record_report(&mut self, report: &ExecutionReport, duration_nanos: u128) {
        self.summary.nodes_evaluated += report.tasks_executed;
        self.summary.nodes_recomputed += report
            .stages
            .iter()
            .flat_map(|stage| &stage.task_records)
            .filter(|record| matches!(record.verdict, Some(EvaluationVerdict::Recomputed)))
            .count() as u32;
        self.summary.nodes_suppressed += report
            .stages
            .iter()
            .flat_map(|stage| &stage.task_records)
            .filter(|record| {
                matches!(
                    record.verdict,
                    Some(EvaluationVerdict::Suppressed { .. })
                )
            })
            .count() as u32;
        self.summary.plans_built += 1;
        self.summary.stages_executed += report.stage_count;
        self.evaluation_nanos += duration_nanos;
        self.latest_report = Some(report.clone());
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::logic::transaction::runtime) struct TransactionSemanticDelta {
    pub failure_summary: Option<FailureSummary>,
    pub rollback: Option<crate::diagnostics::failure::RollbackDiagnostic>,
    pub replay_events: Vec<TransactionReplayEntry>,
    pub event_epochs: Vec<EventEpochSummary>,
}

pub(in crate::logic::transaction::runtime) enum StagedEventOperation<E> {
    Emit(E),
    Flush(CheckpointBarrier),
}

pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) config: &'a mut SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: &'a mut crate::data::graph::SignalGraph,
    pub(in crate::logic::transaction::runtime) checkpoint: &'a mut CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: &'a mut EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) telemetry: &'a mut RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) staged_dirty: BatchedDirtySet<D, I>,
    pub(in crate::logic::transaction::runtime) staged_checkpoint_flushes: u64,
    pub(in crate::logic::transaction::runtime) staged_checkpoint_flush_nanos: u128,
    pub(in crate::logic::transaction::runtime) staged_event_flush_nanos: u128,
    pub(in crate::logic::transaction::runtime) staged_event_operations: Vec<StagedEventOperation<E>>,
    pub(in crate::logic::transaction::runtime) staged_memo_writes: BTreeMap<
        (RuntimeStringId, RuntimeStringId, RuntimeStringId),
        crate::data::output::NodeEvaluationResult,
    >,
    pub(in crate::logic::transaction::runtime) graph_patches: SparsePatchBuffer,
    pub(in crate::logic::transaction::runtime) created_nodes: Vec<crate::data::handle::NodeId>,
    pub(in crate::logic::transaction::runtime) baseline_config: SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) baseline_diagnostics_state: DiagnosticsState,
    pub(in crate::logic::transaction::runtime) semantic_delta: TransactionSemanticDelta,
    pub(in crate::logic::transaction::runtime) mark_dirty_seen: DenseBitset,
    pub(in crate::logic::transaction::runtime) mark_dirty_staged: DenseBitset,
    pub(in crate::logic::transaction::runtime) evaluate_seen: DenseBitset,
    pub(in crate::logic::transaction::runtime) dirty_targets: DenseBitset,
    pub(in crate::logic::transaction::runtime) poisoned: bool,
    pub(in crate::logic::transaction::runtime) finished: bool,
    pub(in crate::logic::transaction::runtime) staged_patch_count: u64,
    pub(in crate::logic::transaction::runtime) execution_state: TransactionExecutionState,
    pub(in crate::logic::transaction::runtime) started_at: Instant,
}
