use std::collections::BTreeMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::data::bitset::DenseBitset;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::BatchedDirtySet;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::replay::ReplayEventKind;
use crate::diagnostics::state::DiagnosticsState;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::EvaluationVerdict;
use crate::logic::events::EventBus;
use crate::logic::planner::{ExecutionRecordId, ExecutionReport, SemanticSegmentId};

use super::super::super::key_registry::RuntimeStringId;
use super::super::super::patch_buffer::SparsePatchBuffer;
use super::super::config::SignalRuntimeConfig;
use super::super::state::ReconstructabilityRecord;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionOutcome {
    Committed,
    RolledBack,
    Poisoned,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransactionTiming {
    pub total_nanos: u128,
    pub evaluation_nanos: u128,
    pub event_flush_nanos: u128,
    pub commit_nanos: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
    pub reconstructability: ReconstructabilityRecord,
    pub event_epochs: Vec<EventEpochSummary>,
    pub rollback: Option<crate::diagnostics::failure::RollbackDiagnostic>,
    pub warnings: Vec<super::envelope::AdvisoryRecord>,
    pub decision_summary: super::envelope::DecisionSummary,
    pub decision_log: super::envelope::DecisionLog,
    pub integrity_markers: super::envelope::IntegrityMarkers,
    pub performance_accounting: RuntimeTelemetry,
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
            .filter(|record| matches!(record.verdict, Some(EvaluationVerdict::Suppressed { .. })))
            .count() as u32;
        self.summary.plans_built += 1;
        self.summary.stages_executed += report.stage_count;
        self.evaluation_nanos += duration_nanos;
        // This retained report has a second observer by design:
        // public transaction evaluation APIs return the report immediately,
        // and the finalized transaction boundary may also need to retain the
        // same report for commit/rollback results.
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

pub(in crate::logic::transaction::runtime) struct TransactionScratch<D, I, E>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub staged_dirty: BatchedDirtySet<D, I>,
    pub staged_checkpoint_flushes: u64,
    pub staged_checkpoint_flush_nanos: u128,
    pub staged_event_flush_nanos: u128,
    pub staged_event_operations: Vec<StagedEventOperation<E>>,
    pub staged_memo_writes: BTreeMap<
        (RuntimeStringId, RuntimeStringId, RuntimeStringId),
        crate::data::output::NodeEvaluationResult,
    >,
    pub graph_patches: SparsePatchBuffer,
    pub created_nodes: Vec<crate::data::handle::NodeId>,
    pub semantic_delta: TransactionSemanticDelta,
    pub mark_dirty_seen: DenseBitset,
    pub mark_dirty_staged: DenseBitset,
    pub evaluate_seen: DenseBitset,
    pub dirty_targets: DenseBitset,
    pub staged_patch_count: u64,
}

impl<D, I, E> TransactionScratch<D, I, E>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub fn new() -> Self {
        Self {
            staged_dirty: BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_event_flush_nanos: 0,
            staged_event_operations: Vec::new(),
            staged_memo_writes: BTreeMap::new(),
            graph_patches: SparsePatchBuffer::new(),
            created_nodes: Vec::new(),
            semantic_delta: TransactionSemanticDelta::default(),
            mark_dirty_seen: DenseBitset::new(),
            mark_dirty_staged: DenseBitset::new(),
            evaluate_seen: DenseBitset::new(),
            dirty_targets: DenseBitset::new(),
            staged_patch_count: 0,
        }
    }
}

pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) runtime_ctx: &'a mut Ctx,
    pub(in crate::logic::transaction::runtime) config: &'a mut SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: &'a mut crate::data::graph::SignalGraph,
    pub(in crate::logic::transaction::runtime) checkpoint: &'a mut CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: &'a mut EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) telemetry: &'a mut RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) scratch: TransactionScratch<D, I, E>,
    pub(in crate::logic::transaction::runtime) baseline_config: SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) baseline_diagnostics_state: DiagnosticsState,
    pub(in crate::logic::transaction::runtime) poisoned: bool,
    pub(in crate::logic::transaction::runtime) finished: bool,
    pub(in crate::logic::transaction::runtime) execution_state: TransactionExecutionState,
    pub(in crate::logic::transaction::runtime) started_at: Instant,
}
