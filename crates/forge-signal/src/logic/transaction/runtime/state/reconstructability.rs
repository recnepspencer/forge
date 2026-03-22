use serde::{Deserialize, Serialize};

use crate::data::graph::SignalGraph;
use crate::data::telemetry::{CheckpointTelemetry, RuntimeTelemetry};
use crate::diagnostics::replay::{ReplayEvent, ReplayEventKind};
use crate::diagnostics::ReplayCursor;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::logic::transaction::TransactionReplayEntry;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct AuthorityState<T>
where
    T: Copy + Ord,
{
    pub graph: SignalGraph,
    pub config: SignalRuntimeConfig<T>,
}

impl<T> AuthorityState<T>
where
    T: Copy + Ord,
{
    pub fn capture(graph: &SignalGraph, config: &SignalRuntimeConfig<T>) -> Self {
        Self {
            graph: graph.clone_stateful(),
            config: config.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct DerivedState<D, I>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub checkpoint: CheckpointRuntime<D, I>,
    pub telemetry: RuntimeTelemetry,
}

impl<D, I> DerivedState<D, I>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub fn capture(checkpoint: &CheckpointRuntime<D, I>, telemetry: &RuntimeTelemetry) -> Self {
        Self {
            checkpoint: checkpoint.clone(),
            telemetry: telemetry.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CheckpointRecord {
    pub checkpoint_flushes: u64,
    pub checkpoint_flush_nanos: u128,
    pub rollback_count: u64,
    pub checkpoint_size: u64,
    pub journal_replay_span: u64,
}

impl CheckpointRecord {
    pub fn from_checkpoint_telemetry(telemetry: CheckpointTelemetry) -> Self {
        Self {
            checkpoint_flushes: telemetry.checkpoint_flushes,
            checkpoint_flush_nanos: telemetry.checkpoint_flush_nanos,
            rollback_count: telemetry.rollback_count,
            checkpoint_size: telemetry.checkpoint_size,
            journal_replay_span: telemetry.journal_replay_span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct JournalSegment {
    pub replay_event_count: u32,
    pub first_execution_record_id: Option<ExecutionRecordId>,
    pub last_execution_record_id: Option<ExecutionRecordId>,
    pub first_semantic_segment_id: Option<SemanticSegmentId>,
    pub last_semantic_segment_id: Option<SemanticSegmentId>,
    pub contains_rollback: bool,
    pub contains_failure: bool,
}

impl JournalSegment {
    pub fn from_entries(entries: &[TransactionReplayEntry]) -> Self {
        let mut segment = Self {
            replay_event_count: entries.len() as u32,
            ..Self::default()
        };
        for entry in entries {
            segment.first_execution_record_id = segment
                .first_execution_record_id
                .or(entry.execution_record_id);
            segment.last_execution_record_id = entry.execution_record_id.or(segment.last_execution_record_id);
            segment.first_semantic_segment_id = segment
                .first_semantic_segment_id
                .or(entry.semantic_segment_id);
            segment.last_semantic_segment_id =
                entry.semantic_segment_id.or(segment.last_semantic_segment_id);
            segment.contains_rollback |=
                matches!(entry.kind, ReplayEventKind::TransactionRolledBack);
            segment.contains_failure |= matches!(entry.kind, ReplayEventKind::FailureRecorded);
        }
        segment
    }

    pub fn from_replay_events(entries: &[ReplayEvent]) -> Self {
        let mut segment = Self {
            replay_event_count: entries.len() as u32,
            ..Self::default()
        };
        for entry in entries {
            let execution_record_id =
                entry.execution_record_id.map(crate::logic::planner::ExecutionRecordId);
            let semantic_segment_id =
                entry.semantic_segment_id.map(crate::logic::planner::SemanticSegmentId);
            segment.first_execution_record_id =
                segment.first_execution_record_id.or(execution_record_id);
            segment.last_execution_record_id =
                execution_record_id.or(segment.last_execution_record_id);
            segment.first_semantic_segment_id =
                segment.first_semantic_segment_id.or(semantic_segment_id);
            segment.last_semantic_segment_id =
                semantic_segment_id.or(segment.last_semantic_segment_id);
            segment.contains_rollback |=
                matches!(entry.kind, ReplayEventKind::TransactionRolledBack);
            segment.contains_failure |= matches!(entry.kind, ReplayEventKind::FailureRecorded);
        }
        segment
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconstructabilityRecord {
    pub authority_branch_id: SignalBranchId,
    pub authority_snapshot_id: Option<SignalSnapshotId>,
    pub replay_head: Option<ReplayCursor>,
    pub checkpoint: CheckpointRecord,
    pub journal: Option<JournalSegment>,
}

impl ReconstructabilityRecord {
    pub fn from_transaction_boundary(
        authority_branch_id: SignalBranchId,
        authority_snapshot_id: Option<SignalSnapshotId>,
        replay_head: Option<ReplayCursor>,
        checkpoint: CheckpointRecord,
        replay_entries: &[TransactionReplayEntry],
    ) -> Self {
        Self {
            authority_branch_id,
            authority_snapshot_id,
            replay_head,
            checkpoint,
            journal: Some(JournalSegment::from_entries(replay_entries)),
        }
    }

    pub fn from_snapshot_boundary(
        authority_branch_id: SignalBranchId,
        authority_snapshot_id: SignalSnapshotId,
        replay_head: Option<ReplayCursor>,
        mut checkpoint: CheckpointRecord,
        replay_entries: &[ReplayEvent],
    ) -> Self {
        let journal = JournalSegment::from_replay_events(replay_entries);
        checkpoint.journal_replay_span = journal.replay_event_count as u64;
        Self {
            authority_branch_id,
            authority_snapshot_id: Some(authority_snapshot_id),
            replay_head,
            checkpoint,
            journal: Some(journal),
        }
    }
}
