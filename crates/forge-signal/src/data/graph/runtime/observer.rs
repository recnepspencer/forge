use crate::data::error::SignalError;
use crate::data::graph::{signal_graph::SignalGraph, EvaluationStrategy};
use crate::data::handle::NodeId;
use crate::data::trace::{
    assemble_historical_artifact_record, assemble_trace_summary, HistoricalArtifactRecord,
    RetainedDiagnosticArtifact, RuntimeArtifactState, TraceSummary,
};
use crate::diagnostics::access::GraphDiagnostics;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::policy::{ArtifactMaterializationMode, SignalRuntimePolicy};
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::replay::{ReplayCursor, ReplayEvent, ReplaySlice};
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::explain::{dependency_chain_to, explain, NodeExplanation};
use crate::presentation::dot::to_dot;
use crate::presentation::metrics::GraphMetrics;
use crate::state::SignalBranchHandle;

pub struct GraphObserver<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphObserver<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn graph(&self) -> &'a SignalGraph {
        self.graph
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
        metrics
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.graph.observation.diagnostics.profile()
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph.derive_evaluation_strategy()
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph.observation.diagnostics.policy()
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        GraphSummary::from_graph(self.graph, profile)
    }

    pub fn diagnostics(&self) -> GraphDiagnostics<'a> {
        GraphDiagnostics::new(self.graph)
    }

    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        ExecutionHistorySummary::from_graph(self.graph, profile)
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

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.explanation_fact(node).map(|fact| {
            let mut explanation = fact.explanation.clone();
            explanation.materialization_mode = ArtifactMaterializationMode::Retained;
            explanation
        })
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph.reconstruct_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.provenance_fact(node).cloned().map(|mut fact| {
            fact.materialization_mode = ArtifactMaterializationMode::Retained;
            fact
        })
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph.reconstruct_provenance_artifact(node)
    }

    /// Cold artifact access that may reconstruct explanation state if retained
    /// artifacts are unavailable and policy allows reconstruction.
    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.explanation_fact(node) {
            let mut explanation = fact.explanation.clone();
            explanation.materialization_mode = ArtifactMaterializationMode::Retained;
            return Ok((Some(explanation), ArtifactMaterializationMode::Retained));
        }
        if self.runtime_policy().can_reconstruct_explanation() {
            return Ok((
                Some(self.reconstruct_explanation_artifact(node)?),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    /// Cold artifact access that may reconstruct provenance state if retained
    /// artifacts are unavailable and policy allows reconstruction.
    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.provenance_fact(node) {
            let mut fact = fact.clone();
            fact.materialization_mode = ArtifactMaterializationMode::Retained;
            return Ok((Some(fact), ArtifactMaterializationMode::Retained));
        }
        if self.runtime_policy().can_reconstruct_provenance() {
            return Ok((
                Some(self.reconstruct_provenance_artifact(node)?),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    pub fn to_dot(&self) -> String {
        to_dot(self.graph)
    }

    pub fn replay_events(&self) -> &'a std::collections::VecDeque<ReplayEvent> {
        self.graph.observation.diagnostics.replay_events()
    }

    pub fn replay_where(&self, mut predicate: impl FnMut(&ReplayEvent) -> bool) -> ReplaySlice {
        ReplaySlice {
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
    ) -> ReplaySlice {
        let mut slice = self.replay_where(|frame| {
            start.is_none_or(|cursor| frame.cursor >= cursor)
                && end.is_none_or(|cursor| frame.cursor <= cursor)
        });
        slice.start = start;
        slice.end = end;
        slice
    }

    pub fn replay_for_branch(&self, branch_id: crate::state::SignalBranchId) -> ReplaySlice {
        self.replay_where(|frame| frame.branch_id == branch_id)
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplaySlice {
        self.replay_where(|frame| frame.node == Some(node))
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplaySlice {
        self.replay_where(|frame| frame.lineage_artifact_id == Some(artifact_id))
    }

    pub fn replay_from_cursor(&self, start: ReplayCursor) -> ReplaySlice {
        self.replay_slice(Some(start), None)
    }

    pub fn replay_between(&self, start: ReplayCursor, end: ReplayCursor) -> ReplaySlice {
        self.replay_slice(Some(start), Some(end))
    }

    pub fn replay_around_snapshot(
        &self,
        snapshot_id: crate::state::SignalSnapshotId,
    ) -> ReplaySlice {
        let Some(index) = self
            .replay_events()
            .iter()
            .position(|event| event.snapshot_id == Some(snapshot_id))
        else {
            return ReplaySlice::default();
        };
        let start = index.saturating_sub(4);
        let end = (index + 5).min(self.replay_events().len());
        let cursors = self
            .replay_events()
            .iter()
            .skip(start)
            .take(end.saturating_sub(start))
            .map(|event| event.cursor)
            .collect::<std::collections::BTreeSet<_>>();
        let mut slice = self.replay_where(|event| cursors.contains(&event.cursor));
        slice.start = self.replay_events().get(start).map(|event| event.cursor);
        slice.end = self
            .replay_events()
            .get(end.saturating_sub(1))
            .map(|event| event.cursor);
        slice
    }

    pub fn lineage_records(&self) -> &'a std::collections::VecDeque<LineageRecord> {
        self.graph.observation.diagnostics.lineage_records()
    }

    pub fn lineage_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.node() == Some(node))
            .cloned()
            .collect()
    }

    pub fn lineage_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.subject_artifact_id() == Some(artifact_id))
            .cloned()
            .collect()
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.graph
            .get_entry(node)
            .ok()
            .and_then(|entry| entry.get_runtime_artifact_state())
            .and_then(|summary| summary.lineage_artifact_id)
    }

    pub fn lineage_chain_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        let mut chain = Vec::new();
        let mut current = Some(artifact_id);
        let mut visited = std::collections::BTreeSet::new();
        while let Some(artifact_id) = current {
            if !visited.insert(artifact_id) {
                break;
            }
            let mut artifact_records = self
                .lineage_records()
                .iter()
                .filter(|record| record.subject_artifact_id() == Some(artifact_id))
                .cloned()
                .collect::<Vec<_>>();
            if artifact_records.is_empty() {
                break;
            }
            artifact_records.sort_by_key(|record| record.sequence);
            current = artifact_records.iter().find_map(|record| {
                record
                    .parent_artifact_id()
                    .filter(|parent| *parent != artifact_id)
            });
            chain.extend(artifact_records);
        }
        chain.sort_by_key(|record| record.sequence);
        chain
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
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
