use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
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
use crate::state::{SignalBranchHandle, SignalSnapshotMeta, SignalSnapshotV1};

impl SignalGraph {
    pub fn telemetry(&self) -> &crate::data::telemetry::RuntimeTelemetry {
        &self.telemetry
    }

    pub fn telemetry_mut(&mut self) -> &mut crate::data::telemetry::RuntimeTelemetry {
        &mut self.telemetry
    }

    pub fn reset_telemetry(&mut self) {
        self.telemetry = crate::data::telemetry::RuntimeTelemetry::default();
    }

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        explain(self, node)
    }

    pub fn dependency_chain_to(
        &self,
        root: NodeId,
        target: NodeId,
    ) -> Result<Option<Vec<NodeId>>, SignalError> {
        dependency_chain_to(self, root, target)
    }

    pub fn metrics(&self) -> GraphMetrics {
        GraphMetrics::from_runtime_telemetry(
            self.telemetry(),
            self.partition_interner.token_count(),
        )
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.diagnostics.profile()
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.diagnostics.policy()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.diagnostics.set_profile(profile);
    }

    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.diagnostics.set_policy(policy);
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        GraphSummary::from_graph(self, profile)
    }

    pub fn diagnostics(&self) -> GraphDiagnostics<'_> {
        GraphDiagnostics::new(self)
    }

    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        ExecutionHistorySummary::from_graph(self, profile)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'_> {
        ExecutionInspector { graph: self }
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&FlowSummary> {
        self.diagnostics.latest_flow()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&FailureSummary> {
        self.diagnostics.latest_failure()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&RollbackDiagnostic> {
        self.diagnostics.latest_rollback()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &std::collections::VecDeque<ExecutionHistorySummary> {
        self.diagnostics.recent_history()
    }

    pub fn replay_events(&self) -> &std::collections::VecDeque<ReplayEvent> {
        self.diagnostics.replay_events()
    }

    pub fn replay_slice(
        &self,
        start: Option<ReplayCursor>,
        end: Option<ReplayCursor>,
    ) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| start.is_none_or(|cursor| frame.cursor >= cursor))
            .filter(|frame| end.is_none_or(|cursor| frame.cursor <= cursor))
            .cloned()
            .collect();
        ReplaySlice { start, end, frames }
    }

    pub fn replay_for_branch(&self, branch_id: crate::state::SignalBranchId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.branch_id == branch_id)
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.node == Some(node))
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplaySlice {
        let frames = self
            .replay_events()
            .iter()
            .filter(|frame| frame.lineage_artifact_id == Some(artifact_id))
            .cloned()
            .collect();
        ReplaySlice {
            start: None,
            end: None,
            frames,
        }
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
        ReplaySlice {
            start: self.replay_events().get(start).map(|event| event.cursor),
            end: self
                .replay_events()
                .get(end.saturating_sub(1))
                .map(|event| event.cursor),
            frames: self
                .replay_events()
                .iter()
                .skip(start)
                .take(end.saturating_sub(start))
                .cloned()
                .collect(),
        }
    }

    pub fn explanation_fact(&self, node: NodeId) -> Option<&ExplanationFact> {
        self.diagnostics.explanation_facts().get(&node)
    }

    pub fn provenance_fact(&self, node: NodeId) -> Option<&ProvenanceFact> {
        self.diagnostics.provenance_facts().get(&node)
    }

    pub fn lineage_records(&self) -> &std::collections::VecDeque<LineageRecord> {
        self.diagnostics.lineage_records()
    }

    pub fn lineage_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.node == Some(node))
            .cloned()
            .collect()
    }

    pub fn lineage_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        self.lineage_records()
            .iter()
            .filter(|record| record.artifact_id == Some(artifact_id))
            .cloned()
            .collect()
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.get_entry(node)
            .ok()
            .and_then(|entry| entry.get_trace_summary())
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
                .filter(|record| record.artifact_id == Some(artifact_id))
                .cloned()
                .collect::<Vec<_>>();
            if artifact_records.is_empty() {
                break;
            }
            artifact_records.sort_by_key(|record| record.sequence);
            current = artifact_records.iter().find_map(|record| {
                record
                    .parent_artifact_id
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
        self.diagnostics.active_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.diagnostics
            .branch_catalog()
            .values()
            .cloned()
            .collect()
    }

    pub fn branch_handle(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<SignalBranchHandle> {
        self.diagnostics.branch_catalog().get(&branch_id).cloned()
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

    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let policy = self.runtime_policy();
        let meta = self.diagnostics_state_mut().allocate_snapshot_meta(policy);
        crate::diagnostics::recorder::record_snapshot_event(
            self,
            crate::diagnostics::replay::ReplayEventKind::SnapshotCaptured,
            Some(meta.snapshot_id),
            format!("snapshot {}", meta.snapshot_id.0),
        );
        SignalSnapshotV1 {
            meta,
            graph: self.clone(),
            diagnostics: self.diagnostics_state().snapshot_payload(),
            graph_telemetry: self.telemetry().clone(),
            runtime_telemetry: None,
        }
    }

    pub(crate) fn validate_snapshot_compatibility(
        &self,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        if snapshot.meta.schema_version != SignalSnapshotMeta::SCHEMA_VERSION {
            return Err(SignalError::invalid_input(format!(
                "snapshot schema version {} is incompatible with runtime schema {}",
                snapshot.meta.schema_version,
                SignalSnapshotMeta::SCHEMA_VERSION
            )));
        }
        if snapshot.meta.core_storage_profile != crate::data::core_profile::CORE_STORAGE_PROFILE_ID
        {
            return Err(SignalError::invalid_input(format!(
                "snapshot core storage profile `{}` is incompatible with active profile `{}`",
                snapshot.meta.core_storage_profile,
                crate::data::core_profile::CORE_STORAGE_PROFILE_ID
            )));
        }
        Ok(())
    }

    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.validate_snapshot_compatibility(snapshot)?;
        let current_diagnostics = self.diagnostics.clone();
        let mut restored = snapshot.graph.clone();
        restored.telemetry = snapshot.graph_telemetry.clone();
        restored
            .diagnostics
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                &current_diagnostics,
            );
        *self = restored;
        crate::diagnostics::recorder::record_snapshot_restore_lineage(
            self,
            snapshot.meta.snapshot_id,
        );
        Ok(())
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
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
        Ok(explanation)
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
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
        Ok(ProvenanceFact::from_explanation(&explanation))
    }

    #[cfg(test)]
    pub(crate) fn test_storage_counts(&self) -> ((usize, usize), (usize, usize), usize) {
        (
            self.dependency_edges.storage_counts(),
            self.subscriber_edges.storage_counts(),
            self.dependency_snapshots.snapshot_count(),
        )
    }

    pub fn explain_artifact(
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

    pub fn provenance_artifact(
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
        to_dot(self)
    }

    pub(crate) fn diagnostics_state(&self) -> &crate::diagnostics::state::DiagnosticsState {
        &self.diagnostics
    }

    pub(crate) fn diagnostics_state_mut(
        &mut self,
    ) -> &mut crate::diagnostics::state::DiagnosticsState {
        &mut self.diagnostics
    }

    pub(crate) fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[crate::data::output::ChangedRegion],
    ) {
        let causality_kind = self
            .get_entry(node)
            .ok()
            .and_then(|entry| entry.get_causality())
            .map(|causality| causality.kind.clone());
        self.diagnostics
            .note_change_input(node, aspect, changed_regions, causality_kind);
    }

    pub(crate) fn record_invalidation_diagnostics(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) {
        self.diagnostics.record_invalidation_result(
            invalidated_direct_subscribers,
            maybe_stale_direct_subscribers,
            partition_scoped_checks,
        );
    }

    pub(crate) fn clear_pending_diagnostics_input(&mut self) {
        self.diagnostics.clear_pending_input();
    }
}
