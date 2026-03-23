use crate::data::graph::SignalGraph;
use crate::data::output::OutputChange;
use crate::data::reuse::ReuseOrigin;
use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary};
use crate::diagnostics::lineage::{
    ArtifactTransitionKind, InvalidationCause, LineageRecord, SnapshotRestoreKind,
};
use crate::diagnostics::policy::{SignalRuntimePolicy, SnapshotRestoreLineageMode};
use crate::diagnostics::replay::{ReplayEvent, ReplayEventDetail, ReplayEventKind};
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::logic::transaction::BranchMergeExecutionSummary;
use crate::state::SignalSnapshotId;

pub struct DiagnosticsRecorder<'a> {
    graph: &'a mut SignalGraph,
}

impl<'a> DiagnosticsRecorder<'a> {
    pub fn new(graph: &'a mut SignalGraph) -> Self {
        Self { graph }
    }

    fn policy(&self) -> SignalRuntimePolicy {
        SignalRuntimePolicy::for_tier(self.graph.diagnostics_profile())
    }

    pub fn record_failure(&mut self, context: ExecutionFailureContext) -> FailureSummary {
        let policy = self.policy();
        let summary = context.summarize(
            self.graph.observe().latest_rollback_diagnostics(),
            policy.tier,
        );
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
    let branch_id = graph.observe().current_branch().id;
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
            None,
            None,
            None,
            Some(ReplayEventDetail::Message(detail.into())),
        ));
}

pub fn record_snapshot_event(
    graph: &mut SignalGraph,
    kind: ReplayEventKind,
    snapshot_id: Option<SignalSnapshotId>,
    detail: impl Into<String>,
) {
    let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
    let branch_id = graph.observe().current_branch().id;
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
            None,
            None,
            None,
            Some(ReplayEventDetail::Message(detail.into())),
        ));
}

pub fn record_branch_fork_lineage(
    graph: &mut SignalGraph,
    created_branch_id: crate::state::SignalBranchId,
    parent_branch_id: crate::state::SignalBranchId,
    created_branch_display_name: impl Into<String>,
    parent_branch_display_name: impl Into<String>,
) {
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_fork(
            sequence,
            emitted_on_branch_id,
            created_branch_id,
            parent_branch_id,
            created_branch_display_name,
            parent_branch_display_name,
        ));
}

pub fn record_branch_switch_lineage(
    graph: &mut SignalGraph,
    from_branch_id: crate::state::SignalBranchId,
    to_branch_id: crate::state::SignalBranchId,
    from_branch_display_name: impl Into<String>,
    to_branch_display_name: impl Into<String>,
) {
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_switch(
            sequence,
            emitted_on_branch_id,
            from_branch_id,
            to_branch_id,
            from_branch_display_name,
            to_branch_display_name,
        ));
}

pub fn record_branch_merge_summary(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    source_branch_display_name: impl Into<String> + Clone,
    target_branch_display_name: impl Into<String> + Clone,
) {
    let source_branch_display_name = source_branch_display_name.into();
    let target_branch_display_name = target_branch_display_name.into();
    let resolved_requirements = summary
        .resolution_plan
        .as_ref()
        .map(|plan| {
            plan.records
                .iter()
                .flat_map(|record| record.required_resolution.iter().copied())
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let detail = format!(
        "merged branch {} into {} with {:?}/{:?}/{:?}/{:?}, resolved_requirements={:?}",
        summary.source_branch_id.0,
        summary.target_branch_id.0,
        summary.merge_kind,
        summary.divergence,
        summary.merge_strategy,
        summary.reconciliation_policy,
        resolved_requirements
    );
    let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
    let branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            cursor,
            ReplayEventKind::BranchMerged,
            branch_id,
            summary.target_snapshot_id_after,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(ReplayEventDetail::Message(detail)),
        ));

    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_merge(
            sequence,
            branch_id,
            summary.source_branch_id,
            summary.target_branch_id,
            summary.merge_kind,
            summary.divergence,
            summary.merge_strategy,
            summary.reconciliation_policy,
            summary.resolution_plan.clone(),
            summary.target_snapshot_id_after,
            source_branch_display_name.clone(),
            target_branch_display_name.clone(),
        ));

    for record in &summary.records {
        let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
        graph
            .diagnostics_state_mut()
            .record_lineage_record(LineageRecord::artifact_merge(
                sequence,
                branch_id,
                record.source_node,
                record.target_node,
                summary.source_branch_id,
                summary.target_branch_id,
                record.source_artifact_id,
                record.target_artifact_id_before,
                record.target_artifact_id_after,
                record.action,
                record.basis,
                summary.merge_kind,
                summary.divergence,
                summary.merge_strategy,
                summary.reconciliation_policy,
                record.resolved_conflict_kinds.clone(),
            ));
    }
}

pub fn record_branch_merge_failure(
    graph: &mut SignalGraph,
    error: &crate::data::error::SignalError,
    source_branch: Option<crate::state::SignalBranchHandle>,
    target_branch: Option<crate::state::SignalBranchHandle>,
) {
    let detail = match (source_branch.as_ref(), target_branch.as_ref(), error) {
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed {
                kind,
                evidence: Some(evidence),
                ..
            },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}, divergence={:?}, primary={:?}, resolution={:?})",
            source_branch.id.0,
            target_branch.id.0,
            evidence.divergence,
            evidence.summary.primary_conflict_kind,
            evidence.summary.required_resolution
        ),
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed { kind, message, .. },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}): {message}",
            source_branch.id.0,
            target_branch.id.0
        ),
        _ => error.to_string(),
    };

    DiagnosticsRecorder::new(graph).record_failure(ExecutionFailureContext::new(
        crate::diagnostics::ExecutionFailurePhase::Planning,
        None,
        None,
        None,
        None,
        None,
        detail.clone(),
    ));

    let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
    let branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            cursor,
            ReplayEventKind::FailureRecorded,
            branch_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(ReplayEventDetail::Message(detail)),
        ));
}

pub fn record_snapshot_restore_lineage(graph: &mut SignalGraph, snapshot_id: SignalSnapshotId) {
    match graph.runtime_policy().snapshot_restore_lineage_mode {
        SnapshotRestoreLineageMode::CompactGlobal => {
            let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
            let emitted_on_branch_id = graph.observe().current_branch().id;
            graph
                .diagnostics_state_mut()
                .record_lineage_record(LineageRecord::snapshot_restore(
                    sequence,
                    emitted_on_branch_id,
                    snapshot_id,
                    None,
                    None,
                    SnapshotRestoreKind::CompactGlobal,
                ));
        }
        SnapshotRestoreLineageMode::PerNode => {
            let emitted_on_branch_id = graph.observe().current_branch().id;
            let restored_nodes = graph
                .live_node_ids()
                .into_iter()
                .filter_map(|node| {
                    graph.get_entry(node).ok().and_then(|entry| {
                        entry.get_runtime_artifact_state().and_then(|state| {
                            state
                                .lineage_artifact_id
                                .map(|artifact_id| (node, artifact_id))
                        })
                    })
                })
                .collect::<Vec<_>>();
            for (node, artifact_id) in restored_nodes {
                let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
                graph.diagnostics_state_mut().record_lineage_record(
                    LineageRecord::snapshot_restore(
                        sequence,
                        emitted_on_branch_id,
                        snapshot_id,
                        Some(node),
                        Some(artifact_id),
                        SnapshotRestoreKind::PerNodeArtifact,
                    ),
                );
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

#[allow(dead_code)]
pub fn record_lineage_transition(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    before_trace: Option<&crate::data::trace::RuntimeArtifactState>,
    execution_record_id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
) -> Result<(), crate::data::error::SignalError> {
    let Some(mut after_trace) = graph.get_entry(node)?.get_runtime_artifact_state().cloned() else {
        return Ok(());
    };
    let previous_artifact_id = before_trace.and_then(|summary| summary.lineage_artifact_id);
    let (artifact_id, transition) = if matches!(
        after_trace.reuse_origin,
        ReuseOrigin::MemoizedArtifactReuse
            | ReuseOrigin::SnapshotRestore
            | ReuseOrigin::ReconciliationAdoption
            | ReuseOrigin::CrossIdentityPersistentReuse
            | ReuseOrigin::PartialArtifactSplice
    ) {
        let artifact_id = previous_artifact_id
            .unwrap_or_else(|| graph.diagnostics_state_mut().allocate_lineage_artifact_id());
        (
            artifact_id,
            match after_trace.reuse_origin {
                ReuseOrigin::MemoizedArtifactReuse => ArtifactTransitionKind::MemoizedReuse,
                ReuseOrigin::SnapshotRestore => ArtifactTransitionKind::SnapshotRestoreReuse,
                ReuseOrigin::ReconciliationAdoption => {
                    ArtifactTransitionKind::ReconciliationAdoption
                }
                ReuseOrigin::CrossIdentityPersistentReuse => {
                    ArtifactTransitionKind::CrossIdentityPersistentReuse {
                        correspondence_kind: after_trace
                            .reuse_boundary_context
                            .as_ref()
                            .and_then(|ctx| ctx.persistent_correspondence())
                            .map(|evidence| evidence.kind())
                            .unwrap_or(crate::data::reuse::PersistentCorrespondenceKind::Unknown),
                    }
                }
                ReuseOrigin::PartialArtifactSplice => ArtifactTransitionKind::PartialArtifactSplice {
                    composition_region_count: after_trace
                        .reuse_boundary_context
                        .as_ref()
                        .and_then(|ctx| ctx.composition_regions())
                        .map(|regions| regions.as_slice().len() as u32)
                        .unwrap_or(0),
                    recomputed_region_count: after_trace.changed_partition_count,
                },
                ReuseOrigin::FreshCompute | ReuseOrigin::OutputSuppressed => {
                    unreachable!("guarded by matches!")
                }
            },
        )
    } else if previous_artifact_id.is_some()
        && matches!(
            after_trace.output_change,
            OutputChange::Refreshed | OutputChange::Unchanged
        )
    {
        (
            previous_artifact_id.expect("checked above"),
            ArtifactTransitionKind::Refreshed {
                output_change: after_trace.output_change,
            },
        )
    } else {
        (
            graph.diagnostics_state_mut().allocate_lineage_artifact_id(),
            ArtifactTransitionKind::Replaced,
        )
    };
    after_trace.lineage_artifact_id = Some(artifact_id);
    graph
        .get_entry_mut(node)?
        .set_runtime_artifact_state(Some(after_trace));
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::artifact_transition(
            sequence,
            emitted_on_branch_id,
            node,
            artifact_id,
            previous_artifact_id,
            execution_record_id,
            semantic_segment_id,
            transition,
        ));
    Ok(())
}

pub fn stamp_trace_summary_and_record_lineage_transition(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    before_trace: Option<&crate::data::trace::RuntimeArtifactState>,
    execution_record_id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
) -> Result<(), crate::data::error::SignalError> {
    let Some(mut after_trace) = graph.get_entry(node)?.get_runtime_artifact_state().cloned() else {
        return Ok(());
    };
    let previous_artifact_id = before_trace.and_then(|summary| summary.lineage_artifact_id);
    let (artifact_id, transition) = if matches!(
        after_trace.reuse_origin,
        ReuseOrigin::MemoizedArtifactReuse
            | ReuseOrigin::SnapshotRestore
            | ReuseOrigin::ReconciliationAdoption
            | ReuseOrigin::CrossIdentityPersistentReuse
            | ReuseOrigin::PartialArtifactSplice
    ) {
        let artifact_id = previous_artifact_id
            .unwrap_or_else(|| graph.diagnostics_state_mut().allocate_lineage_artifact_id());
        (
            artifact_id,
            match after_trace.reuse_origin {
                ReuseOrigin::MemoizedArtifactReuse => ArtifactTransitionKind::MemoizedReuse,
                ReuseOrigin::SnapshotRestore => ArtifactTransitionKind::SnapshotRestoreReuse,
                ReuseOrigin::ReconciliationAdoption => {
                    ArtifactTransitionKind::ReconciliationAdoption
                }
                ReuseOrigin::CrossIdentityPersistentReuse => {
                    ArtifactTransitionKind::CrossIdentityPersistentReuse {
                        correspondence_kind: after_trace
                            .reuse_boundary_context
                            .as_ref()
                            .and_then(|ctx| ctx.persistent_correspondence())
                            .map(|evidence| evidence.kind())
                            .unwrap_or(crate::data::reuse::PersistentCorrespondenceKind::Unknown),
                    }
                }
                ReuseOrigin::PartialArtifactSplice => ArtifactTransitionKind::PartialArtifactSplice {
                    composition_region_count: after_trace
                        .reuse_boundary_context
                        .as_ref()
                        .and_then(|ctx| ctx.composition_regions())
                        .map(|regions| regions.as_slice().len() as u32)
                        .unwrap_or(0),
                    recomputed_region_count: after_trace.changed_partition_count,
                },
                ReuseOrigin::FreshCompute | ReuseOrigin::OutputSuppressed => {
                    unreachable!("guarded by matches!")
                }
            },
        )
    } else if previous_artifact_id.is_some()
        && matches!(
            after_trace.output_change,
            OutputChange::Refreshed | OutputChange::Unchanged
        )
    {
        (
            previous_artifact_id.expect("checked above"),
            ArtifactTransitionKind::Refreshed {
                output_change: after_trace.output_change,
            },
        )
    } else {
        (
            graph.diagnostics_state_mut().allocate_lineage_artifact_id(),
            ArtifactTransitionKind::Replaced,
        )
    };
    after_trace.execution_record_id = Some(execution_record_id.0);
    after_trace.semantic_segment_id = Some(semantic_segment_id.0);
    after_trace.lineage_artifact_id = Some(artifact_id);
    graph
        .get_entry_mut(node)?
        .set_runtime_artifact_state(Some(after_trace));
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::artifact_transition(
            sequence,
            emitted_on_branch_id,
            node,
            artifact_id,
            previous_artifact_id,
            execution_record_id,
            semantic_segment_id,
            transition,
        ));
    Ok(())
}

pub fn record_invalidation_lineage(
    graph: &mut SignalGraph,
    node: crate::data::handle::NodeId,
    cause: InvalidationCause,
) {
    let Some(artifact_id) = graph
        .get_entry(node)
        .ok()
        .and_then(|entry| entry.get_runtime_artifact_state())
        .and_then(|state| state.lineage_artifact_id)
    else {
        return;
    };
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::invalidation(
            sequence,
            emitted_on_branch_id,
            node,
            artifact_id,
            cause,
        ));
}


