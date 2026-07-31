use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::clock::RuntimeInstant;
use crate::data::bitset::DenseBitset;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::dirty_set::BatchedDirtySet;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::DirtyBatchEntry;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::temporal::{
    IntervalWakeRegeneration, LoweredTemporalEligibility, ReadyTemporalWake, RetiredTemporalWake,
    RuntimeClockBasis, ScheduledTemporalWake, TemporalExecutionSummary,
    TemporalPreviousValueReference, TemporalWakeReschedule, TemporalWakeReuse,
};
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::replay::ReplayEventKind;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::EvaluationVerdict;
use crate::logic::events::EventBus;
use crate::logic::planner::{ExecutionRecordId, ExecutionReport, SemanticSegmentId};

use super::super::super::key_registry::RuntimeStringId;
use super::super::super::patch_buffer::SparsePatchBuffer;
use super::super::config::SignalRuntimeConfig;
use super::super::state::{
    BranchManager, ReconstructabilityRecord, ResourceRuntimeState, RuntimeObservationRegistry,
    TemporalRuntimeState,
};
use super::transaction_observation::{ObservationBoundarySummary, TransactionObservationScratch};

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
    pub temporal_summary: TemporalExecutionSummary,
    pub temporal_evidence: TemporalTransactionEvidence,
    pub reconstructability: ReconstructabilityRecord,
    pub event_epochs: Vec<EventEpochSummary>,
    pub rollback: Option<crate::diagnostics::failure::RollbackDiagnostic>,
    pub warnings: Vec<super::envelope::AdvisoryRecord>,
    pub observation: ObservationBoundarySummary,
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
            mark_dirty_seen: DenseBitset::new(),
            mark_dirty_staged: DenseBitset::new(),
            evaluate_seen: DenseBitset::new(),
            dirty_targets: DenseBitset::new(),
            staged_patch_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalEligibilityFact {
    pub node: NodeId,
    pub eligibility: LoweredTemporalEligibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemporalTransactionEvidence {
    pub clock_basis: RuntimeClockBasis,
    pub eligibility_facts: Vec<TemporalEligibilityFact>,
    pub scheduled_wakes: Vec<ScheduledTemporalWake>,
    pub ready_wakes: Vec<ReadyTemporalWake>,
    pub retired_wakes: Vec<RetiredTemporalWake>,
    pub rescheduled_wakes: Vec<TemporalWakeReschedule>,
    pub reused_wakes: Vec<TemporalWakeReuse>,
    pub interval_regenerations: Vec<IntervalWakeRegeneration>,
    pub previous_value_references: Vec<TemporalPreviousValueReference>,
}

impl TemporalTransactionEvidence {
    pub fn has_temporal_facts(&self) -> bool {
        !self.eligibility_facts.is_empty()
            || !self.scheduled_wakes.is_empty()
            || !self.ready_wakes.is_empty()
            || !self.retired_wakes.is_empty()
            || !self.rescheduled_wakes.is_empty()
            || !self.reused_wakes.is_empty()
            || !self.interval_regenerations.is_empty()
            || !self.previous_value_references.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::logic::transaction::runtime) struct TransactionTemporalScratch {
    pub summary: TemporalExecutionSummary,
    pub eligibility_facts: Vec<TemporalEligibilityFact>,
    pub scheduled_wakes: Vec<ScheduledTemporalWake>,
    pub ready_wakes: Vec<ReadyTemporalWake>,
    pub retired_wakes: Vec<RetiredTemporalWake>,
    pub rescheduled_wakes: Vec<TemporalWakeReschedule>,
    pub reused_wakes: Vec<TemporalWakeReuse>,
    pub interval_regenerations: Vec<IntervalWakeRegeneration>,
    pub previous_value_references: Vec<TemporalPreviousValueReference>,
}

impl TransactionTemporalScratch {
    pub fn absorb_report(&mut self, report: &ExecutionReport) {
        self.summary.absorb(report.temporal_summary);
        for stage in &report.stages {
            for task in &stage.task_records {
                if let Some(eligibility) = task.temporal_eligibility.clone() {
                    self.eligibility_facts.push(TemporalEligibilityFact {
                        node: task.node,
                        eligibility,
                    });
                }
            }
        }
    }

    pub fn record_scheduled_wake(&mut self, wake: ScheduledTemporalWake) {
        self.scheduled_wakes.push(wake);
    }

    pub fn record_ready_wake(&mut self, wake: ReadyTemporalWake) {
        self.ready_wakes.push(wake);
    }

    pub fn record_retired_wake(&mut self, wake: RetiredTemporalWake) {
        self.retired_wakes.push(wake);
    }

    pub fn record_rescheduled_wake(&mut self, reschedule: TemporalWakeReschedule) {
        self.rescheduled_wakes.push(reschedule);
    }

    pub fn record_reused_wake(&mut self, reuse: TemporalWakeReuse) {
        self.reused_wakes.push(reuse);
    }

    pub fn record_interval_regeneration(&mut self, regeneration: IntervalWakeRegeneration) {
        self.interval_regenerations.push(regeneration);
    }

    pub fn record_previous_value_reference(&mut self, reference: TemporalPreviousValueReference) {
        self.previous_value_references.push(reference);
    }

    pub fn boundary_evidence(&self, clock_basis: RuntimeClockBasis) -> TemporalTransactionEvidence {
        TemporalTransactionEvidence {
            clock_basis,
            eligibility_facts: self.eligibility_facts.clone(),
            scheduled_wakes: self.scheduled_wakes.clone(),
            ready_wakes: self.ready_wakes.clone(),
            retired_wakes: self.retired_wakes.clone(),
            rescheduled_wakes: self.rescheduled_wakes.clone(),
            reused_wakes: self.reused_wakes.clone(),
            interval_regenerations: self.interval_regenerations.clone(),
            previous_value_references: self.previous_value_references.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct ConfigRollbackDelta<T: Copy + Ord> {
    pub baseline: SignalRuntimeConfig<T>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct DiagnosticsRollbackDelta {
    pub baseline: crate::diagnostics::state::DiagnosticsState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct GraphPatchRollbackDelta {
    pub patches: SparsePatchBuffer,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct CreatedNodeRollbackDelta {
    pub created_nodes: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SubscriberRepairRollbackDelta {
    pub sources: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct ResourceRollbackDelta {
    pub baseline: ResourceRuntimeState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct TemporalRollbackDelta {
    pub baseline: TemporalRuntimeState,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) enum TransactionRollbackPacket<T: Copy + Ord> {
    Config(ConfigRollbackDelta<T>),
    DiagnosticsRequired(DiagnosticsRollbackDelta),
    GraphPatches(GraphPatchRollbackDelta),
    CreatedNodes(CreatedNodeRollbackDelta),
    SubscriberRepair(SubscriberRepairRollbackDelta),
    Resource(ResourceRollbackDelta),
    Temporal(TemporalRollbackDelta),
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct TransactionRollbackPacketSet<T: Copy + Ord> {
    config: Option<ConfigRollbackDelta<T>>,
    diagnostics: Option<DiagnosticsRollbackDelta>,
    graph_patches: Option<GraphPatchRollbackDelta>,
    created_nodes: Option<CreatedNodeRollbackDelta>,
    subscriber_repair: Option<SubscriberRepairRollbackDelta>,
    resource: Option<ResourceRollbackDelta>,
    temporal: Option<TemporalRollbackDelta>,
}

impl<T: Copy + Ord> Default for TransactionRollbackPacketSet<T> {
    fn default() -> Self {
        Self {
            config: None,
            diagnostics: None,
            graph_patches: None,
            created_nodes: None,
            subscriber_repair: None,
            resource: None,
            temporal: None,
        }
    }
}

impl<T: Copy + Ord> TransactionRollbackPacketSet<T> {
    pub fn capture_runtime_baseline_if_needed(
        &mut self,
        config: &SignalRuntimeConfig<T>,
        diagnostics_state: &crate::diagnostics::state::DiagnosticsState,
    ) {
        if self.config.is_none() {
            self.config = Some(ConfigRollbackDelta {
                baseline: config.clone(),
            });
        }
        if self.diagnostics.is_none() {
            self.diagnostics = Some(DiagnosticsRollbackDelta {
                baseline: diagnostics_state.clone(),
            });
        }
    }

    pub fn stage_graph_patches(
        &mut self,
        delta: GraphPatchRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.graph_patches.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "graph patch rollback packet was staged more than once",
            ));
        }
        self.graph_patches = Some(delta);
        Ok(())
    }

    pub fn stage_created_nodes(
        &mut self,
        delta: CreatedNodeRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.created_nodes.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "created-node rollback packet was staged more than once",
            ));
        }
        self.created_nodes = Some(delta);
        Ok(())
    }

    pub fn stage_subscriber_repair(
        &mut self,
        delta: SubscriberRepairRollbackDelta,
    ) -> Result<(), crate::data::error::SignalError> {
        if self.subscriber_repair.is_some() {
            return Err(crate::data::error::SignalError::internal(
                "subscriber-repair rollback packet was staged more than once",
            ));
        }
        self.subscriber_repair = Some(delta);
        Ok(())
    }

    pub fn capture_resource_baseline_if_needed(&mut self, resource: &ResourceRuntimeState) {
        if self.resource.is_none() {
            self.resource = Some(ResourceRollbackDelta {
                baseline: resource.clone(),
            });
        }
    }

    pub fn capture_temporal_baseline_if_needed(&mut self, temporal: &TemporalRuntimeState) {
        if self.temporal.is_none() {
            self.temporal = Some(TemporalRollbackDelta {
                baseline: temporal.clone(),
            });
        }
    }

    pub fn drain_ordered(&mut self) -> Vec<TransactionRollbackPacket<T>> {
        let mut packets = Vec::with_capacity(7);
        if let Some(delta) = self.graph_patches.take() {
            packets.push(TransactionRollbackPacket::GraphPatches(delta));
        }
        if let Some(delta) = self.created_nodes.take() {
            packets.push(TransactionRollbackPacket::CreatedNodes(delta));
        }
        if let Some(delta) = self.subscriber_repair.take() {
            packets.push(TransactionRollbackPacket::SubscriberRepair(delta));
        }
        if let Some(delta) = self.resource.take() {
            packets.push(TransactionRollbackPacket::Resource(delta));
        }
        if let Some(delta) = self.temporal.take() {
            packets.push(TransactionRollbackPacket::Temporal(delta));
        }
        if let Some(delta) = self.config.take() {
            packets.push(TransactionRollbackPacket::Config(delta));
        }
        if let Some(delta) = self.diagnostics.take() {
            packets.push(TransactionRollbackPacket::DiagnosticsRequired(delta));
        }
        packets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) enum TransactionCommitPosture {
    Visible,
    BranchLocal,
}

pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) runtime_ctx: &'a mut Ctx,
    pub(in crate::logic::transaction::runtime) observations:
        &'a RuntimeObservationRegistry<D, I, E, Ctx, T>,
    pub(in crate::logic::transaction::runtime) config: &'a mut SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: &'a mut crate::data::graph::SignalGraph,
    pub(in crate::logic::transaction::runtime) checkpoint: &'a mut CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: &'a mut EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) resource: &'a mut ResourceRuntimeState,
    pub(in crate::logic::transaction::runtime) temporal: &'a mut TemporalRuntimeState,
    pub(in crate::logic::transaction::runtime) telemetry: &'a mut RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) branches: &'a mut BranchManager<D, I, T>,
    pub(in crate::logic::transaction::runtime) scratch: TransactionScratch<D, I, E>,
    pub(in crate::logic::transaction::runtime) rollback_packets: TransactionRollbackPacketSet<T>,
    pub(in crate::logic::transaction::runtime) poisoned: bool,
    pub(in crate::logic::transaction::runtime) finished: bool,
    pub(in crate::logic::transaction::runtime) execution_state: TransactionExecutionState,
    pub(in crate::logic::transaction::runtime) started_at: RuntimeInstant,
    pub(in crate::logic::transaction::runtime) commit_posture: TransactionCommitPosture,
}

pub struct BatchChangeSession<'tx, 'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    pub(in crate::logic::transaction::runtime) entries: Vec<DirtyBatchEntry>,
    pub(in crate::logic::transaction::runtime) applied: bool,
}

impl<'tx, 'a, D, I, E, Ctx, T> BatchChangeSession<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) fn new(
        tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    ) -> Self {
        Self {
            tx,
            entries: Vec::new(),
            applied: false,
        }
    }

    pub fn mark(mut self, source: NodeId, changed_aspect: crate::data::aspect::Aspect) -> Self {
        self.entries
            .push(DirtyBatchEntry::without_regions(source, changed_aspect));
        self
    }

    pub fn mark_regions(
        mut self,
        source: NodeId,
        changed_aspect: crate::data::aspect::Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Self {
        self.entries.push(DirtyBatchEntry::new(
            source,
            changed_aspect,
            changed_regions.to_vec(),
        ));
        self
    }
}
