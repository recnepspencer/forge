use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{MemoizedResultOrigin, OutputChange};
use crate::data::reuse::ReuseBasis;
use crate::diagnostics::epochs::EventEpochSummary;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::logic::explain::{CausalDisposition, NodeExplanation, UpstreamCause};
use crate::logic::planner::{
    EvaluationPlan, ExecutionReport, SessionScratch, StageExecutionOutcome, TaskExecutionOutcome,
    TaskReason,
};
use crate::presentation::metrics::GraphMetrics;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphSummary {
    pub profile: DiagnosticsProfile,
    pub active_node_count: u32,
    pub arena_capacity: u32,
    pub tombstone_count: u32,
    pub clean_node_count: u32,
    pub maybe_stale_node_count: u32,
    pub dirty_node_count: u32,
    pub dependency_edge_count: u32,
    pub subscriber_edge_count: u32,
    pub nodes_with_partition_scopes: u32,
    pub nodes_with_trace_summary: u32,
    pub nodes_with_execution_record: u32,
    pub nodes_with_causality: u32,
    pub partition_interner_size: u32,
    pub sample_dirty_nodes: Vec<NodeId>,
    pub sample_nodes_with_execution_record: Vec<NodeId>,
    pub metrics: GraphMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPlanSummary {
    pub profile: DiagnosticsProfile,
    pub requested_target_count: u32,
    pub stage_count: u32,
    pub task_count: u32,
    pub max_stage_width: u32,
    pub contract_pruned_count: u32,
    pub stage_widths: Vec<u32>,
    pub direct_request_count: u32,
    pub transitive_task_count: u32,
    pub task_reason_counts: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReportSummary {
    pub profile: DiagnosticsProfile,
    pub stage_count: u32,
    pub task_count: u32,
    pub tasks_executed: u32,
    pub tasks_pruned: u32,
    pub tasks_validated_clean: u32,
    pub tasks_deferred_by_condition: u32,
    pub tasks_reverted_clean_by_condition: u32,
    pub tasks_satisfied_by_memoization: u32,
    pub tasks_with_suppressed_propagation: u32,
    pub prepared_evaluations_produced: u32,
    pub prepared_evaluations_applied: u32,
    pub dependency_capture_updates: u32,
    pub semantic_segment_count: u32,
    pub task_outcome_counts: BTreeMap<String, u32>,
    pub stage_outcome_counts: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationSummary {
    pub profile: DiagnosticsProfile,
    pub node: NodeId,
    pub materialization_mode: String,
    pub state: NodeState,
    pub dirty_aspect_count: u32,
    pub upstream_count: u32,
    pub changed_upstream_count: u32,
    pub skipped_upstream_count: u32,
    pub condition_deferred_count: u32,
    pub clean_upstream_count: u32,
    pub missing_snapshot_count: u32,
    pub dependency_removed_count: u32,
    pub conservative_cause_count: u32,
    pub direct_scope_count: u32,
    pub translated_scope_count: u32,
    pub discarded_scope_count: u32,
    pub insufficient_scope_count: u32,
    pub rewired_dependency_count: u32,
    pub direct_cause_kinds: Vec<String>,
    pub scope_provenance_kinds: Vec<String>,
    pub cause_note_samples: Vec<String>,
    pub triage_classes: Vec<String>,
    pub propagation_suppressed: bool,
    pub contract_reads_mask: u128,
    pub contract_produces_mask: u128,
    pub contract_partition_scope_count: u32,
    pub required_context: String,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub output_change: Option<OutputChange>,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub reuse_basis: Option<ReuseBasis>,
    pub reuse_certification_proof_count: u32,
    pub changed_region_count: u32,
    pub causality_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionHistoryNodeSummary {
    pub node: NodeId,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub output_change: Option<OutputChange>,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub reuse_basis: Option<ReuseBasis>,
    pub reuse_certification_proof_count: u32,
    pub changed_partition_count: u32,
    pub causality_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionHistorySummary {
    pub profile: DiagnosticsProfile,
    pub traced_node_count: u32,
    pub execution_record_count: u32,
    pub latest_execution_record_id: Option<u64>,
    pub nodes: Vec<ExecutionHistoryNodeSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventEpochHistorySummary {
    pub epochs: Vec<EventEpochSummary>,
}

impl GraphSummary {
    pub fn from_graph(graph: &SignalGraph, profile: DiagnosticsProfile) -> Self {
        let mut clean_node_count = 0_u32;
        let mut maybe_stale_node_count = 0_u32;
        let mut dirty_node_count = 0_u32;
        let mut dependency_edge_count = 0_u32;
        let mut subscriber_edge_count = 0_u32;
        let mut nodes_with_partition_scopes = 0_u32;
        let mut nodes_with_trace_summary = 0_u32;
        let mut nodes_with_execution_record = 0_u32;
        let mut nodes_with_causality = 0_u32;
        let mut sample_dirty_nodes = Vec::new();
        let mut sample_nodes_with_execution_record = Vec::new();

        for index in 0..graph.arena_capacity() {
            let Some(node) = graph.live_node_id_at(index) else {
                continue;
            };
            let Ok(entry) = graph.get_entry(node) else {
                continue;
            };
            match entry.get_state() {
                NodeState::Clean => clean_node_count += 1,
                NodeState::MaybeStale => maybe_stale_node_count += 1,
                NodeState::Dirty => {
                    dirty_node_count += 1;
                    if sample_dirty_nodes.len() < profile.detail_limit() {
                        sample_dirty_nodes.push(node);
                    }
                }
            }
            let Ok(dependencies) = graph.dependencies_of(node) else {
                continue;
            };
            dependency_edge_count += dependencies.len() as u32;
            if let Ok(subscribers) = graph.subscribers_of(node) {
                subscriber_edge_count += subscribers.len() as u32;
            }
            if dependencies.iter().any(|edge| edge.scope_ref().is_some()) {
                nodes_with_partition_scopes += 1;
            }
            if let Some(trace) = entry.get_runtime_artifact_state() {
                nodes_with_trace_summary += 1;
                if trace.execution_record_id.is_some() {
                    nodes_with_execution_record += 1;
                    if sample_nodes_with_execution_record.len() < profile.detail_limit() {
                        sample_nodes_with_execution_record.push(node);
                    }
                }
            }
            if entry.get_causality().is_some() {
                nodes_with_causality += 1;
            }
        }

        Self {
            profile,
            active_node_count: graph.active_node_count() as u32,
            arena_capacity: graph.arena_capacity() as u32,
            tombstone_count: graph.tombstone_count(),
            clean_node_count,
            maybe_stale_node_count,
            dirty_node_count,
            dependency_edge_count,
            subscriber_edge_count,
            nodes_with_partition_scopes,
            nodes_with_trace_summary,
            nodes_with_execution_record,
            nodes_with_causality,
            partition_interner_size: graph.observe().metrics().partition_interner_size as u32,
            sample_dirty_nodes,
            sample_nodes_with_execution_record,
            metrics: graph.observe().metrics(),
        }
    }
}

impl EvaluationPlanSummary {
    pub fn from_plan(plan: &EvaluationPlan, profile: DiagnosticsProfile) -> Self {
        Self::from_components(
            plan.summary.requested_target_count,
            plan.summary.stage_count,
            plan.summary.task_count,
            plan.summary.max_stage_width,
            plan.summary.contract_pruned_count,
            plan.stages.iter().map(|stage| stage.tasks.len() as u32),
            plan.stages.iter().flat_map(|stage| stage.tasks.iter()),
            profile,
        )
    }

    pub(crate) fn from_session(session: &SessionScratch<'_>, profile: DiagnosticsProfile) -> Self {
        Self::from_components(
            session.summary.requested_target_count,
            session.summary.stage_count,
            session.summary.task_count,
            session.summary.max_stage_width,
            session.summary.contract_pruned_count,
            session
                .stages
                .iter()
                .map(|stage| (stage.end - stage.start) as u32),
            session.tasks.iter(),
            profile,
        )
    }

    fn from_components<'a>(
        requested_target_count: u32,
        stage_count: u32,
        task_count: u32,
        max_stage_width: u32,
        contract_pruned_count: u32,
        stage_widths_iter: impl Iterator<Item = u32>,
        tasks_iter: impl Iterator<Item = &'a crate::logic::planner::EligibleTask>,
        profile: DiagnosticsProfile,
    ) -> Self {
        let mut task_reason_counts = BTreeMap::new();
        let mut direct_request_count = 0_u32;
        let mut stage_widths = stage_widths_iter.collect::<Vec<_>>();
        for task in tasks_iter {
            *task_reason_counts
                .entry(format!("{:?}", task.reason))
                .or_insert(0) += 1;
            if task.direct_request {
                direct_request_count += 1;
            }
        }
        if stage_widths.len() > profile.detail_limit() {
            stage_widths.truncate(profile.detail_limit());
        }
        Self {
            profile,
            requested_target_count,
            stage_count,
            task_count,
            max_stage_width,
            contract_pruned_count,
            stage_widths,
            direct_request_count,
            transitive_task_count: task_count.saturating_sub(direct_request_count),
            task_reason_counts,
        }
    }
}

impl ExecutionReportSummary {
    pub fn from_report(report: &ExecutionReport, profile: DiagnosticsProfile) -> Self {
        let mut task_outcome_counts = BTreeMap::new();
        let mut stage_outcome_counts = BTreeMap::new();
        for stage in &report.stages {
            *stage_outcome_counts
                .entry(format!("{:?}", stage.outcome))
                .or_insert(0) += 1;
            for task in &stage.task_records {
                *task_outcome_counts
                    .entry(format!("{:?}", task.outcome))
                    .or_insert(0) += 1;
            }
        }

        Self {
            profile,
            stage_count: report.stage_count,
            task_count: report.task_count,
            tasks_executed: report.tasks_executed,
            tasks_pruned: report.tasks_pruned,
            tasks_validated_clean: report.tasks_validated_clean,
            tasks_deferred_by_condition: report.tasks_deferred_by_condition,
            tasks_reverted_clean_by_condition: report.tasks_reverted_clean_by_condition,
            tasks_satisfied_by_memoization: report.tasks_satisfied_by_memoization,
            tasks_with_suppressed_propagation: report.tasks_with_suppressed_propagation,
            prepared_evaluations_produced: report.prepared_evaluations_produced,
            prepared_evaluations_applied: report.prepared_evaluations_applied,
            dependency_capture_updates: report.dependency_capture_updates,
            semantic_segment_count: report.semantic_segment_count,
            task_outcome_counts,
            stage_outcome_counts,
        }
    }
}

impl ExplanationSummary {
    pub fn from_explanation(explanation: &NodeExplanation, profile: DiagnosticsProfile) -> Self {
        let mut changed_upstream_count = 0_u32;
        let mut skipped_upstream_count = 0_u32;
        let mut condition_deferred_count = 0_u32;
        let mut clean_upstream_count = 0_u32;
        let mut missing_snapshot_count = 0_u32;
        let mut dependency_removed_count = 0_u32;
        let mut conservative_cause_count = 0_u32;
        let mut direct_scope_count = 0_u32;
        let mut translated_scope_count = 0_u32;
        let mut discarded_scope_count = 0_u32;
        let mut insufficient_scope_count = 0_u32;
        let mut direct_cause_kinds = Vec::new();
        let mut scope_provenance_kinds = Vec::new();
        let mut cause_note_samples = Vec::new();
        let mut triage_classes = Vec::new();

        for cause in &explanation.upstream {
            match cause {
                UpstreamCause::Changed { .. } => changed_upstream_count += 1,
                UpstreamCause::SkippedByComparator { .. } => skipped_upstream_count += 1,
                UpstreamCause::ConditionDeferred { .. } => condition_deferred_count += 1,
                UpstreamCause::Clean { .. } => clean_upstream_count += 1,
                UpstreamCause::MissingSnapshot { .. } => missing_snapshot_count += 1,
                UpstreamCause::DependencyRemoved { .. } => dependency_removed_count += 1,
            }
        }
        for link in &explanation.causal_links {
            if matches!(link.disposition, CausalDisposition::Conservative) {
                conservative_cause_count += 1;
            }
            match link.scope.kind {
                crate::logic::explain::ScopeProvenanceKind::Direct => direct_scope_count += 1,
                crate::logic::explain::ScopeProvenanceKind::Translated => {
                    translated_scope_count += 1
                }
                crate::logic::explain::ScopeProvenanceKind::Discarded => discarded_scope_count += 1,
                crate::logic::explain::ScopeProvenanceKind::InsufficientEvidence => {
                    insufficient_scope_count += 1
                }
                crate::logic::explain::ScopeProvenanceKind::None => {}
            }
            if direct_cause_kinds.len() < profile.detail_limit() {
                direct_cause_kinds.push(link.kind.clone());
            }
            if !matches!(
                link.scope.kind,
                crate::logic::explain::ScopeProvenanceKind::None
            ) && scope_provenance_kinds.len() < profile.detail_limit()
            {
                scope_provenance_kinds.push(format!("{:?}", link.scope.kind));
            }
            if let Some(note) = &link.note {
                if cause_note_samples.len() < profile.detail_limit() {
                    cause_note_samples.push(note.clone());
                }
            }
            push_triage_class(
                &mut triage_classes,
                triage_class_for_link(link, explanation.rewiring.is_some()),
            );
        }
        if explanation.rewiring.is_some() {
            push_triage_class(&mut triage_classes, Some("rewiring".to_string()));
        }

        Self {
            profile,
            node: explanation.node,
            materialization_mode: format!("{:?}", explanation.materialization_mode),
            state: explanation.state,
            dirty_aspect_count: explanation.dirty_aspects.bits().count_ones(),
            upstream_count: explanation.upstream.len() as u32,
            changed_upstream_count,
            skipped_upstream_count,
            condition_deferred_count,
            clean_upstream_count,
            missing_snapshot_count,
            dependency_removed_count,
            conservative_cause_count,
            direct_scope_count,
            translated_scope_count,
            discarded_scope_count,
            insufficient_scope_count,
            rewired_dependency_count: explanation
                .rewiring
                .as_ref()
                .map(|rewiring| (rewiring.added.len() + rewiring.removed.len()) as u32)
                .unwrap_or(0),
            direct_cause_kinds,
            scope_provenance_kinds,
            cause_note_samples,
            triage_classes,
            propagation_suppressed: explanation.propagation_suppressed,
            contract_reads_mask: explanation.contract_reads.bits() as u128,
            contract_produces_mask: explanation.contract_produces.bits() as u128,
            contract_partition_scope_count: explanation
                .contract_partition_scope
                .as_ref()
                .map(|scopes| scopes.len() as u32)
                .unwrap_or(0),
            required_context: format!("{:?}", explanation.required_context),
            execution_record_id: explanation.execution_record_id,
            semantic_segment_id: explanation.semantic_segment_id,
            output_change: explanation.output_change,
            memoized_origin: explanation.memoized_origin,
            reuse_basis: explanation.reuse_basis,
            reuse_certification_proof_count: explanation
                .reuse_certification
                .as_ref()
                .map(|record| record.proofs.len() as u32)
                .unwrap_or(0),
            changed_region_count: explanation.changed_regions.len() as u32,
            causality_kind: explanation.causality.as_ref().map(|c| c.kind.clone()),
        }
    }
}

impl ExecutionHistorySummary {
    pub fn from_graph(graph: &SignalGraph, profile: DiagnosticsProfile) -> Self {
        let mut traced_node_count = 0_u32;
        let mut execution_record_count = 0_u32;
        let mut latest_execution_record_id = None;
        let mut nodes = Vec::new();
        let retain_nodes = profile.retains_history_details();

        for index in 0..graph.arena_capacity() {
            let Some(node) = graph.live_node_id_at(index) else {
                continue;
            };
            let Ok(entry) = graph.get_entry(node) else {
                continue;
            };
            let Some(trace) = entry.get_runtime_artifact_state() else {
                continue;
            };
            traced_node_count += 1;
            if let Some(id) = trace.execution_record_id {
                execution_record_count += 1;
                latest_execution_record_id =
                    Some(latest_execution_record_id.map_or(id, |current: u64| current.max(id)));
            }
            if retain_nodes {
                nodes.push(ExecutionHistoryNodeSummary {
                    node,
                    execution_record_id: trace.execution_record_id,
                    semantic_segment_id: trace.semantic_segment_id,
                    output_change: Some(trace.output_change),
                    memoized_origin: Some(trace.memoized_origin),
                    reuse_basis: Some(trace.reuse_basis),
                    reuse_certification_proof_count: entry
                        .retained_diagnostic_artifact()
                        .and_then(|retained| retained.reuse_certification.as_ref())
                        .map(|record| record.proofs.len() as u32)
                        .unwrap_or(0),
                    changed_partition_count: trace.changed_partition_count,
                    causality_kind: entry.get_causality().map(|c| c.kind.clone()),
                });
            }
        }

        if retain_nodes {
            nodes.sort_by(|left, right| {
                right
                    .execution_record_id
                    .cmp(&left.execution_record_id)
                    .then_with(|| right.semantic_segment_id.cmp(&left.semantic_segment_id))
                    .then_with(|| left.node.index().cmp(&right.node.index()))
                    .then_with(|| left.node.generation().cmp(&right.node.generation()))
            });
            if nodes.len() > profile.detail_limit() {
                nodes.truncate(profile.detail_limit());
            }
        }

        Self {
            profile,
            traced_node_count,
            execution_record_count,
            latest_execution_record_id,
            nodes,
        }
    }
}

impl EvaluationPlan {
    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> EvaluationPlanSummary {
        EvaluationPlanSummary::from_plan(self, profile)
    }
}

impl ExecutionReport {
    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> ExecutionReportSummary {
        ExecutionReportSummary::from_report(self, profile)
    }
}

impl NodeExplanation {
    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> ExplanationSummary {
        ExplanationSummary::from_explanation(self, profile)
    }
}

fn triage_class_for_link(
    link: &crate::logic::explain::CausalLink,
    rewired: bool,
) -> Option<String> {
    if rewired || matches!(link.disposition, CausalDisposition::Topology) {
        return Some("rewiring".to_string());
    }
    if matches!(
        link.scope.kind,
        crate::logic::explain::ScopeProvenanceKind::Direct
            | crate::logic::explain::ScopeProvenanceKind::Translated
            | crate::logic::explain::ScopeProvenanceKind::Discarded
            | crate::logic::explain::ScopeProvenanceKind::InsufficientEvidence
    ) {
        return Some("locality".to_string());
    }
    if matches!(link.disposition, CausalDisposition::Conservative)
        || link.kind == "SkippedByComparator"
        || link.kind.starts_with("ConditionDeferred::")
    {
        return Some("validation".to_string());
    }
    None
}

fn push_triage_class(target: &mut Vec<String>, value: Option<String>) {
    let Some(value) = value else {
        return;
    };
    if !target.contains(&value) {
        target.push(value);
    }
}

fn _stage_outcome_key(outcome: StageExecutionOutcome) -> &'static str {
    match outcome {
        StageExecutionOutcome::CompletedSerial => "CompletedSerial",
        #[cfg(feature = "parallel")]
        StageExecutionOutcome::CompletedParallel => "CompletedParallel",
    }
}

fn _task_outcome_key(outcome: TaskExecutionOutcome) -> &'static str {
    match outcome {
        TaskExecutionOutcome::Recomputed => "Recomputed",
        TaskExecutionOutcome::ValidatedClean => "ValidatedClean",
        TaskExecutionOutcome::ConditionDeferred => "ConditionDeferred",
        TaskExecutionOutcome::ConditionRevertedClean => "ConditionRevertedClean",
        TaskExecutionOutcome::MemoizedReuse => "MemoizedReuse",
        TaskExecutionOutcome::PropagationSuppressed => "PropagationSuppressed",
        TaskExecutionOutcome::Pruned => "Pruned",
    }
}

fn _task_reason_key(reason: TaskReason) -> &'static str {
    match reason {
        TaskReason::Dirty => "Dirty",
        TaskReason::MaybeStaleValidation => "MaybeStaleValidation",
        TaskReason::ConditionForced => "ConditionForced",
        TaskReason::RequestedTarget => "RequestedTarget",
        TaskReason::DependencyRequired => "DependencyRequired",
        TaskReason::MemoValidation => "MemoValidation",
        TaskReason::PartitionScopedDependency => "PartitionScopedDependency",
        TaskReason::OutputDiffDependent => "OutputDiffDependent",
    }
}
