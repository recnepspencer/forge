use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::policy::ArtifactMaterializationMode;
use crate::logic::explain::{explain, NodeExplanation};
use crate::state::{SignalSnapshotMeta, SignalSnapshotV1};

impl SignalGraph {
    pub(crate) fn explanation_fact(&self, node: NodeId) -> Option<&ExplanationFact> {
        self.observation.diagnostics.explanation_facts().get(&node)
    }

    pub(crate) fn provenance_fact(&self, node: NodeId) -> Option<&ProvenanceFact> {
        self.observation.diagnostics.provenance_facts().get(&node)
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
        let current_diagnostics = self.observation.diagnostics.clone();
        let mut restored = snapshot.graph.clone();
        restored.observation.telemetry = snapshot.graph_telemetry.clone();
        restored
            .observation
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

    pub(crate) fn explain_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.explanation_fact(node) {
            let mut explanation = fact.explanation.clone();
            explanation.materialization_mode = ArtifactMaterializationMode::Retained;
            return Ok((Some(explanation), ArtifactMaterializationMode::Retained));
        }
        if self.runtime_policy().can_reconstruct_explanation() {
            let mut explanation = explain(self, node)?;
            explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
            return Ok((
                Some(explanation),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    pub(crate) fn provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.provenance_fact(node) {
            let mut fact = fact.clone();
            fact.materialization_mode = ArtifactMaterializationMode::Retained;
            return Ok((Some(fact), ArtifactMaterializationMode::Retained));
        }
        if self.runtime_policy().can_reconstruct_provenance() {
            let mut explanation = explain(self, node)?;
            explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
            return Ok((
                Some(ProvenanceFact::from_explanation(&explanation)),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    #[cfg(test)]
    pub(crate) fn test_storage_counts(&self) -> ((usize, usize), (usize, usize), usize) {
        (
            self.topology.dependency_edges.storage_counts(),
            self.topology.subscriber_edges.storage_counts(),
            self.topology.dependency_snapshots.snapshot_count(),
        )
    }
}
