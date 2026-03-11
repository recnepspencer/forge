use crate::data::graph::SignalGraph;
use crate::data::output::{MemoizedResultOrigin, OutputChange};
use crate::data::trace::TraceSummary;
use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary};
use crate::diagnostics::lineage::{LineageEvent, LineageRecord};
use crate::diagnostics::policy::{DiagnosticsPolicy, SnapshotRestoreLineageMode};
use crate::diagnostics::replay::{ReplayEvent, ReplayEventKind};
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::state::SignalSnapshotId;

pub struct DiagnosticsRecorder<'a> {
    graph: &'a mut SignalGraph,
}

impl<'a> DiagnosticsRecorder<'a> {
    pub fn new(graph: &'a mut SignalGraph) -> Self {
        Self { graph }
    }

    fn policy(&self) -> DiagnosticsPolicy {
        DiagnosticsPolicy::from_profile(self.graph.diagnostics_profile())
    }

    pub fn record_failure(&mut self, context: ExecutionFailureContext) -> FailureSummary {
        let policy = self.policy();
        let summary = context.summarize(self.graph.latest_rollback_diagnostics(), policy.profile);
        self.record_failure_summary(summary.clone());
        self.graph.clear_pending_diagnostics_input();
        summary
    }

    pub fn record_failure_summary(&mut self, summary: FailureSummary) {
        self.graph.diagnostics_state_mut().record_failure(summary);
    }
}

pub fn record_transaction_semantic_event(
    graph: &mut SignalGraph,
    kind: ReplayEventKind,
    detail: impl Into<String>,
    execution_record_id: Option<u64>,
    semantic_segment_id: Option<u64>,
) {
    let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
    let branch_id = graph.current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            cursor,
            kind,
            branch_id,
            None,
            None,
            execution_record_id,
            semantic_segment_id,
            None,
            Some(detail.into()),
        ));
}

pub fn record_snapshot_event(
    graph: &mut SignalGraph,
    kind: ReplayEventKind,
    snapshot_id: Option<SignalSnapshotId>,
    detail: impl Into<String>,
) {
    let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
    let branch_id = graph.current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            cursor,
            kind,
            branch_id,
            snapshot_id,
            None,
            None,
            None,
            None,
            Some(detail.into()),
        ));
}

pub fn record_branch_lineage_event(
    graph: &mut SignalGraph,
    event: LineageEvent,
    detail: impl Into<String>,
) {
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let branch_id = graph.current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::new(
            sequence,
            branch_id,
            None,
            None,
            None,
            event,
            None,
            None,
            None,
            Some(detail.into()),
        ));
}

pub fn record_snapshot_restore_lineage(graph: &mut SignalGraph, snapshot_id: SignalSnapshotId) {
    match graph.runtime_policy().snapshot_restore_lineage_mode {
        SnapshotRestoreLineageMode::CompactGlobal => {
            let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
            let branch_id = graph.current_branch().id;
            graph
                .diagnostics_state_mut()
                .record_lineage_record(LineageRecord::new(
                    sequence,
                    branch_id,
                    None,
                    None,
                    None,
                    LineageEvent::Restored,
                    None,
                    None,
                    Some(snapshot_id),
                    Some(format!("restored snapshot {}", snapshot_id.0)),
                ));
        }
        SnapshotRestoreLineageMode::PerNode => {
            let branch_id = graph.current_branch().id;
            let restored_nodes = graph
                .live_node_ids()
                .into_iter()
                .filter_map(|node| {
                    graph.get_entry(node).ok().and_then(|entry| {
                        entry.get_trace_summary().and_then(|summary| {
                            summary
                                .lineage_artifact_id
                                .map(|artifact_id| (node, artifact_id))
                        })
                    })
                })
                .collect::<Vec<_>>();
            for (node, artifact_id) in restored_nodes {
                let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
                graph
                    .diagnostics_state_mut()
                    .record_lineage_record(LineageRecord::new(
                        sequence,
                        branch_id,
                        Some(node),
                        Some(artifact_id),
                        Some(artifact_id),
                        LineageEvent::Restored,
                        None,
                        None,
                        Some(snapshot_id),
                        Some(format!("restored from snapshot {}", snapshot_id.0)),
                    ));
            }
        }
    }
    record_snapshot_event(
        graph,
        ReplayEventKind::SnapshotRestored,
        Some(snapshot_id),
        format!("restored snapshot {}", snapshot_id.0),
    );
}

pub fn record_lineage_transition(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    before_trace: Option<&TraceSummary>,
    execution_record_id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
) -> Result<(), crate::data::error::SignalError> {
    let Some(mut after_trace) = graph.get_entry(node)?.get_trace_summary().cloned() else {
        return Ok(());
    };
    let previous_artifact_id = before_trace.and_then(|summary| summary.lineage_artifact_id);
    let (event, artifact_id, detail) = if matches!(
        after_trace.memoized_origin,
        MemoizedResultOrigin::MemoizedFromCache
    ) {
        let artifact_id = previous_artifact_id
            .unwrap_or_else(|| graph.diagnostics_state_mut().allocate_lineage_artifact_id());
        (
            LineageEvent::MemoizedFrom,
            artifact_id,
            Some("memoized result reused".to_string()),
        )
    } else if previous_artifact_id.is_some()
        && matches!(
            after_trace.output_change,
            OutputChange::Refreshed | OutputChange::Unchanged
        )
    {
        (
            LineageEvent::Refreshed,
            previous_artifact_id.expect("checked above"),
            Some(format!(
                "output continuity preserved via {:?}",
                after_trace.output_change
            )),
        )
    } else {
        (
            LineageEvent::Replaced,
            graph.diagnostics_state_mut().allocate_lineage_artifact_id(),
            Some("output artifact replaced".to_string()),
        )
    };
    after_trace.lineage_artifact_id = Some(artifact_id);
    graph
        .get_entry_mut(node)?
        .set_trace_summary(Some(after_trace));
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let branch_id = graph.current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::new(
            sequence,
            branch_id,
            Some(node),
            Some(artifact_id),
            previous_artifact_id,
            event,
            Some(execution_record_id),
            Some(semantic_segment_id),
            None,
            detail,
        ));
    Ok(())
}

pub fn record_invalidation_lineage(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    detail: impl Into<String>,
) {
    let Some(artifact_id) = graph
        .get_entry(node)
        .ok()
        .and_then(|entry| entry.get_trace_summary())
        .and_then(|summary| summary.lineage_artifact_id)
    else {
        return;
    };
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let branch_id = graph.current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::new(
            sequence,
            branch_id,
            Some(node),
            Some(artifact_id),
            Some(artifact_id),
            LineageEvent::InvalidatedWithoutReplacement,
            None,
            None,
            None,
            Some(detail.into()),
        ));
}
