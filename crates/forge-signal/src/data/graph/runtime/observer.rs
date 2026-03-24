use crate::data::error::SignalError;
use crate::data::graph::{signal_graph::SignalGraph, EvaluationStrategy};
use crate::data::handle::NodeId;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::data::trace::{
    assemble_historical_artifact_record, assemble_trace_summary, HistoricalArtifactRecord,
    RetainedDiagnosticArtifact, RuntimeArtifactState, TraceSummary,
};
use crate::diagnostics::access::GraphDiagnostics;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::lineage::{
    LineageArtifactId, LineageRecord, RetainedLineageView, SynthesizedLineageChain,
};
use crate::diagnostics::policy::{
    DiagnosticsAvailability, OrdinaryAccessLane, SignalRuntimePolicy,
};
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::replay::{
    ReplayCursor, ReplayEvent, RetainedReplayView, SynthesizedReplaySlice,
};
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::explain::{dependency_chain_to, explain, NodeExplanation};
use crate::presentation::dot::to_dot;
use crate::presentation::metrics::GraphMetrics;
use crate::state::SignalBranchHandle;

pub struct GraphObserver<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphMaterializer<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphObserver<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn graph(&self) -> &'a SignalGraph {
        self.graph
    }

    pub fn materialize(&self) -> GraphMaterializer<'a> {
        GraphMaterializer { graph: self.graph }
    }

    pub fn telemetry(&self) -> &'a crate::data::telemetry::RuntimeTelemetry {
        &self.graph.observation.telemetry
    }

    pub fn metrics(&self) -> GraphMetrics {
        let mut metrics = GraphMetrics::from_runtime_telemetry(
            self.telemetry(),
            self.graph.observation.partition_interner.token_count(),
        );
        metrics.storage.hot_path_artifact_reconstruction_count =
            self.graph.hot_path_artifact_reconstruction_count();
        metrics.storage.explicit_cold_materialization_request_count =
            self.graph.explicit_cold_materialization_request_count();
        metrics.storage.retained_forensic_read_count = self.graph.retained_forensic_read_count();
        metrics.storage.cold_explanation_reconstruction_count =
            self.graph.cold_explanation_reconstruction_count();
        metrics.storage.cold_provenance_reconstruction_count =
            self.graph.cold_provenance_reconstruction_count();
        metrics.storage.retained_artifact_read_count = self.graph.retained_artifact_read_count();
        metrics.storage.reconstructed_artifact_read_count =
            self.graph.reconstructed_artifact_read_count();
        metrics.storage.denied_reconstruction_by_budget_count =
            self.graph.denied_reconstruction_by_budget_count();
        metrics.storage.denied_reconstruction_by_tier_count =
            self.graph.denied_reconstruction_by_tier_count();
        metrics.storage.denied_reconstruction_explanation_api_count =
            self.graph.denied_reconstruction_explanation_api_count();
        metrics.storage.denied_reconstruction_provenance_api_count =
            self.graph.denied_reconstruction_provenance_api_count();
        metrics
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsTier {
        self.graph.observation.diagnostics.tier()
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph.derive_evaluation_strategy()
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph.observation.diagnostics.policy()
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsTier) -> GraphSummary {
        if self.graph.diagnostics_state().has_pending_change_input() {
            if let Some(summary) = self.graph.diagnostics_state().pending_graph_summary() {
                return summary.with_profile(profile);
            }
        } else if let Some(summary) = self.graph.diagnostics_state().latest_graph_summary() {
            return summary.with_profile(profile);
        }
        GraphSummary::from_graph(
            self.graph,
            profile,
            self.runtime_policy().retention_budget.detail_limit,
            OrdinaryAccessLane,
        )
    }

    pub fn diagnostics(&self) -> GraphDiagnostics<'a> {
        GraphDiagnostics::new(self.graph)
    }

    pub fn execution_history_summary(&self, profile: DiagnosticsTier) -> ExecutionHistorySummary {
        let retention_budget = SignalRuntimePolicy::for_tier(profile).retention_budget;
        if let Some(summary) = self.graph.diagnostics_state().recent_history().back() {
            if !retention_budget.retain_history_details || !summary.nodes.is_empty() {
                return summary.with_profile(profile);
            }
        }
        ExecutionHistorySummary::from_graph(
            self.graph,
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        )
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'a> {
        ExecutionInspector { graph: self.graph }
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&'a FlowSummary> {
        self.graph.observation.diagnostics.latest_flow()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&'a FailureSummary> {
        self.graph.observation.diagnostics.latest_failure()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&'a RollbackDiagnostic> {
        self.graph.observation.diagnostics.latest_rollback()
    }

    pub fn latest_frontier_execution_summary(&self) -> Option<&'a FrontierExecutionSummary> {
        self.graph
            .observation
            .diagnostics
            .latest_frontier_execution()
    }

    pub fn latest_invalidation_trace_records(&self) -> &'a [InvalidationTraceRecord] {
        self.graph
            .observation
            .diagnostics
            .latest_invalidation_trace_records()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &'a std::collections::VecDeque<ExecutionHistorySummary> {
        self.graph.observation.diagnostics.recent_history()
    }

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        explain(self.graph, node)
    }

    pub fn runtime_artifact_state(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RuntimeArtifactState>, SignalError> {
        Ok(self.graph.get_entry(node)?.get_runtime_artifact_state())
    }

    pub fn retained_diagnostic_artifact(
        &self,
        node: NodeId,
    ) -> Result<Option<&'a RetainedDiagnosticArtifact>, SignalError> {
        Ok(self.graph.get_entry(node)?.retained_diagnostic_artifact())
    }

    pub fn dependency_chain_to(
        &self,
        root: NodeId,
        target: NodeId,
    ) -> Result<Option<Vec<NodeId>>, SignalError> {
        dependency_chain_to(self.graph, root, target)
    }

    pub fn explanation_fact(&self, node: NodeId) -> Option<&'a ExplanationFact> {
        self.graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node)
    }

    pub fn provenance_fact(&self, node: NodeId) -> Option<&'a ProvenanceFact> {
        self.graph
            .observation
            .diagnostics
            .provenance_facts()
            .get(&node)
    }

    pub fn to_dot(&self) -> String {
        to_dot(self.graph)
    }

    pub fn replay_events(&self) -> &'a std::collections::VecDeque<ReplayEvent> {
        self.graph.observation.diagnostics.replay_events()
    }

    pub fn replay_where(
        &self,
        mut predicate: impl FnMut(&ReplayEvent) -> bool,
    ) -> SynthesizedReplaySlice {
        SynthesizedReplaySlice {
            start: None,
            end: None,
            frames: self
                .replay_events()
                .iter()
                .filter(|frame| predicate(frame))
                .cloned()
                .collect(),
        }
    }

    pub fn replay_slice(
        &self,
        start: Option<ReplayCursor>,
        end: Option<ReplayCursor>,
    ) -> RetainedReplayView<'a> {
        let start_index =
            start.and_then(|cursor| self.graph.diagnostics_state().replay_cursor_offset(cursor));
        let end_index = end
            .and_then(|cursor| self.graph.diagnostics_state().replay_cursor_offset(cursor))
            .map(|index| index + 1);
        if start_index.is_some() || end_index.is_some() {
            let start_index = start_index.unwrap_or(0);
            let end_index = end_index.unwrap_or_else(|| self.replay_events().len());
            return RetainedReplayView::new(
                start,
                end,
                self.replay_events(),
                start_index,
                end_index.saturating_sub(start_index),
            );
        }
        RetainedReplayView::new(
            start,
            end,
            self.replay_events(),
            0,
            self.replay_events().len(),
        )
    }

    pub fn replay_for_branch(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> RetainedReplayView<'a> {
        self.graph
            .diagnostics_state()
            .replay_events_for_branch(branch_id)
            .map(|frames| RetainedReplayView::new(None, None, frames, 0, frames.len()))
            .unwrap_or_else(RetainedReplayView::empty)
    }

    pub fn replay_for_node(&self, node: NodeId) -> RetainedReplayView<'a> {
        self.graph
            .diagnostics_state()
            .replay_events_for_node(node)
            .map(|frames| RetainedReplayView::new(None, None, frames, 0, frames.len()))
            .unwrap_or_else(RetainedReplayView::empty)
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> RetainedReplayView<'a> {
        self.graph
            .diagnostics_state()
            .replay_events_for_artifact(artifact_id)
            .map(|frames| RetainedReplayView::new(None, None, frames, 0, frames.len()))
            .unwrap_or_else(RetainedReplayView::empty)
    }

    pub fn replay_from_cursor(&self, start: ReplayCursor) -> RetainedReplayView<'a> {
        self.replay_slice(Some(start), None)
    }

    pub fn replay_between(&self, start: ReplayCursor, end: ReplayCursor) -> RetainedReplayView<'a> {
        self.replay_slice(Some(start), Some(end))
    }

    pub fn replay_around_snapshot(
        &self,
        snapshot_id: crate::state::SignalSnapshotId,
    ) -> RetainedReplayView<'a> {
        let Some(cursor) = self
            .graph
            .diagnostics_state()
            .snapshot_replay_cursor(snapshot_id)
        else {
            return RetainedReplayView::empty();
        };
        let Some(index) = self.graph.diagnostics_state().replay_cursor_offset(cursor) else {
            return RetainedReplayView::empty();
        };
        let start = index.saturating_sub(4);
        let end = (index + 5).min(self.replay_events().len());
        RetainedReplayView::new(
            self.replay_events().get(start).map(|event| event.cursor),
            self.replay_events()
                .get(end.saturating_sub(1))
                .map(|event| event.cursor),
            self.replay_events(),
            start,
            end.saturating_sub(start),
        )
    }

    pub fn lineage_records(&self) -> &'a std::collections::VecDeque<LineageRecord> {
        self.graph.observation.diagnostics.lineage_records()
    }

    pub fn lineage_for_node(&self, node: NodeId) -> RetainedLineageView<'a> {
        self.graph
            .diagnostics_state()
            .lineage_records_for_node(node)
            .map(|records| RetainedLineageView::new(records, 0, records.len()))
            .unwrap_or_else(RetainedLineageView::empty)
    }

    pub fn lineage_for_artifact(&self, artifact_id: LineageArtifactId) -> RetainedLineageView<'a> {
        self.graph
            .diagnostics_state()
            .lineage_records_for_artifact(artifact_id)
            .map(|records| RetainedLineageView::new(records, 0, records.len()))
            .unwrap_or_else(RetainedLineageView::empty)
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.graph
            .get_entry(node)
            .ok()
            .and_then(|entry| entry.get_runtime_artifact_state())
            .and_then(|summary| summary.lineage_artifact_id)
    }

    pub fn lineage_chain_for_artifact(
        &self,
        artifact_id: LineageArtifactId,
    ) -> SynthesizedLineageChain {
        let mut chain = Vec::new();
        let mut current = Some(artifact_id);
        let mut visited = std::collections::BTreeSet::new();
        while let Some(artifact_id) = current {
            if !visited.insert(artifact_id) {
                break;
            }
            let artifact_records = self
                .graph
                .diagnostics_state()
                .lineage_records_for_artifact(artifact_id)
                .map(|records| records.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            if artifact_records.is_empty() {
                break;
            }
            current = artifact_records.iter().find_map(|record| {
                record
                    .parent_artifact_id()
                    .filter(|parent| *parent != artifact_id)
            });
            chain.extend(artifact_records);
        }
        SynthesizedLineageChain::new(chain)
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> SynthesizedLineageChain {
        self.current_lineage_artifact(node)
            .map(|artifact_id| self.lineage_chain_for_artifact(artifact_id))
            .unwrap_or_default()
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph.observation.diagnostics.active_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph
            .observation
            .diagnostics
            .branch_catalog()
            .values()
            .cloned()
            .collect()
    }

    pub fn branch_handle(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<SignalBranchHandle> {
        self.graph
            .observation
            .diagnostics
            .branch_catalog()
            .get(&branch_id)
            .cloned()
    }

    pub fn branch_head_snapshot_id(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<crate::state::SignalSnapshotId> {
        self.branch_handle(branch_id)
            .and_then(|branch| branch.head_snapshot_id)
    }

    pub fn branch_ancestry(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Vec<SignalBranchHandle> {
        let mut lineage = Vec::new();
        let mut current = self.branch_handle(branch_id);
        while let Some(branch) = current {
            current = branch
                .parent_branch_id
                .and_then(|parent_id| self.branch_handle(parent_id));
            lineage.push(branch);
        }
        lineage.reverse();
        lineage
    }
}

impl<'a> GraphMaterializer<'a> {
    /// Cold artifact access that assembles a historical view from runtime and
    /// optional retained lanes.
    pub fn materialize_historical_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<HistoricalArtifactRecord>, SignalError> {
        let entry = self.graph.get_entry(node)?;
        Ok(assemble_historical_artifact_record(
            node,
            entry.get_runtime_artifact_state(),
            entry.retained_diagnostic_artifact(),
            entry.get_causality(),
        ))
    }

    /// Cold artifact access that assembles a trace summary from runtime and
    /// optional retained lanes.
    pub fn materialize_trace_summary(
        &self,
        node: NodeId,
    ) -> Result<Option<TraceSummary>, SignalError> {
        let entry = self.graph.get_entry(node)?;
        Ok(assemble_trace_summary(
            entry.get_runtime_artifact_state(),
            entry.retained_diagnostic_artifact(),
        ))
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        let fact = self
            .graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node)?;
        let mut explanation = if fact.compact_projection {
            self.graph
                .reconstruct_explanation_artifact_without_retained_fast_path(node)
                .ok()?
        } else {
            fact.explanation.clone()
        };
        explanation.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
        self.graph.record_retained_forensic_read();
        self.graph.record_retained_artifact_read();
        Some(explanation)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph.reconstruct_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        let explanation_fact = self
            .graph
            .observation
            .diagnostics
            .explanation_facts()
            .get(&node);
        let mut fact = match (
            explanation_fact.map(|fact| fact.compact_projection),
            self.graph
                .observation
                .diagnostics
                .provenance_facts()
                .get(&node)
                .cloned(),
        ) {
            (Some(true), _) => self
                .graph
                .reconstruct_provenance_artifact_without_retained_fast_path(node)
                .ok()?,
            (_, Some(fact)) => fact,
            _ => return None,
        };
        fact.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
        self.graph.record_retained_forensic_read();
        self.graph.record_retained_artifact_read();
        Some(fact)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph.reconstruct_provenance_artifact(node)
    }

    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.graph.materialize_explanation_artifact(node)
    }

    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), SignalError> {
        self.graph.materialize_provenance_artifact(node)
    }
}
