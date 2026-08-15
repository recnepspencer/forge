use std::collections::BTreeMap;

use crate::data::bitset::DenseBitset;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::BatchedDirtySet;
use crate::data::temporal::TemporalExecutionSummary;
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::failure::FailureSummary;
use crate::logic::evaluation::EvaluationVerdict;
use crate::logic::planner::ExecutionReport;

use super::super::super::super::key_registry::RuntimeStringId;
use super::super::super::super::patch_buffer::SparsePatchBuffer;
use super::super::transaction_observation::{
    ObservationBoundarySummary, TransactionObservationScratch,
};

use super::evidence::TransactionTemporalScratch;
use super::outcome::{EvaluationSummary, TransactionReplayEntry};

#[derive(Debug, Clone, Default)]
pub(in crate::logic::transaction::runtime) struct TransactionExecutionState {
    pub latest_report: Option<ExecutionReport>,
    pub summary: EvaluationSummary,
    pub temporal_summary: TemporalExecutionSummary,
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
        self.temporal_summary.absorb(report.temporal_summary);
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
    pub observation: ObservationBoundarySummary,
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
    pub observations: TransactionObservationScratch,
    pub temporal: TransactionTemporalScratch,
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
            observations: TransactionObservationScratch::default(),
            temporal: TransactionTemporalScratch::default(),
            staged_dirty: BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_event_flush_nanos: 0,
            staged_event_operations: Vec::new(),
            staged_memo_writes: BTreeMap::new(),
            graph_patches: SparsePatchBuffer::new(),
            created_nodes: Vec::new(),
            semantic_delta: TransactionSemanticDelta::default(),
            mark_dirty_staged: DenseBitset::new(),
            evaluate_seen: DenseBitset::new(),
            dirty_targets: DenseBitset::new(),
            staged_patch_count: 0,
        }
    }
}
