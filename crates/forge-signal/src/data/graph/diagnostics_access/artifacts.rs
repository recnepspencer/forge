use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::policy::ArtifactMaterializationMode;
use crate::logic::explain::{explain, NodeExplanation};
use crate::state::{
    SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRetentionPolicy,
    SnapshotArtifactRestoreMode, SnapshotDependencyRestoreMode, SnapshotRestoreIntent,
    SnapshotRestoreCoarseReason, SnapshotRestorePlan,
};

impl SignalGraph {
    pub(crate) fn explanation_fact(&self, node: NodeId) -> Option<&ExplanationFact> {
        self.observation.diagnostics.explanation_facts().get(&node)
    }

    pub(crate) fn provenance_fact(&self, node: NodeId) -> Option<&ProvenanceFact> {
        self.observation.diagnostics.provenance_facts().get(&node)
    }

    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let policy = self.runtime_policy();
        let artifact_retention = SnapshotArtifactRetentionPolicy::from_runtime_policy(policy);
        let meta = self
            .diagnostics_state_mut()
            .allocate_snapshot_meta(policy, artifact_retention);
        crate::diagnostics::recorder::record_snapshot_event(
            self,
            crate::diagnostics::replay::ReplayEventKind::SnapshotCaptured,
            Some(meta.snapshot_id),
            format!("snapshot {}", meta.snapshot_id.0),
        );
        SignalSnapshotV1 {
            meta,
            graph: self.clone(),
            diagnostics: self
                .diagnostics_state()
                .snapshot_payload_with_retention(artifact_retention),
            graph_telemetry: self.telemetry().clone(),
            runtime_telemetry: None,
            reconstructability: None,
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
        self.restore_snapshot_with_intent(snapshot, SnapshotRestoreIntent::restore_runtime_truth())
    }

    pub fn plan_snapshot_restore(
        &self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<SnapshotRestorePlan, SignalError> {
        self.validate_snapshot_compatibility(snapshot)?;

        let mut shared_node_count = 0_u64;
        let mut current_only_node_count = 0_u64;
        for index in 0..self.arena_capacity() {
            let Some(node) = self.live_node_id_at(index) else {
                continue;
            };
            if snapshot.graph.is_alive(node) {
                shared_node_count += 1;
            } else {
                current_only_node_count += 1;
            }
        }

        let mut snapshot_only_node_count = 0_u64;
        for index in 0..snapshot.graph.arena_capacity() {
            let Some(node) = snapshot.graph.live_node_id_at(index) else {
                continue;
            };
            if !self.is_alive(node) {
                snapshot_only_node_count += 1;
            }
        }

        let dependency_snapshot_batch =
            self.derive_dependency_snapshot_restore_batch(&snapshot.graph)?;
        let dependency_snapshot_delta_node_count =
            dependency_snapshot_batch.target_nodes().as_slice().len() as u64;
        let mut coarse_reasons = vec![
            SnapshotRestoreCoarseReason::EntryStateRewind,
            SnapshotRestoreCoarseReason::DiagnosticsHistoryRestore,
        ];
        if current_only_node_count > 0 || snapshot_only_node_count > 0 {
            coarse_reasons.push(SnapshotRestoreCoarseReason::NodeSetDifference);
        }

        Ok(SnapshotRestorePlan {
            intent,
            shared_node_count,
            current_only_node_count,
            snapshot_only_node_count,
            dependency_snapshot_batch,
            dependency_snapshot_delta_node_count,
            coarse_replacement_required: true,
            coarse_reasons,
        })
    }

    pub fn restore_snapshot_with_intent(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let restore_plan = self.plan_snapshot_restore(snapshot, intent)?;
        self.validate_snapshot_compatibility(snapshot)?;
        if matches!(
            intent.dependency_state,
            SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot
        ) {
            return Err(SignalError::invalid_input(
                "snapshot restore intent `SeedRecomputationFromSnapshot` is not implemented yet",
            ));
        }
        let current_diagnostics = self.observation.diagnostics.clone();
        let current_policy = current_diagnostics.policy();
        let mut restored = snapshot.graph.clone();
        restored.observation.telemetry = snapshot.graph_telemetry.clone();
        restored
            .observation
            .diagnostics
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                &current_diagnostics,
            );
        match intent.artifacts {
            SnapshotArtifactRestoreMode::RestoreCapturedRetention => {}
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy => {
                restored
                    .observation
                    .diagnostics
                    .set_policy(current_policy);
            }
        }
        *self = restored;
        self.telemetry_mut().checkpoint.snapshot_restore_count += 1;
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            self.telemetry_mut()
                .checkpoint
                .snapshot_restore_apply_active_policy_count += 1;
        }
        self.telemetry_mut()
            .checkpoint
            .snapshot_restore_shared_delta_node_count +=
            restore_plan.dependency_snapshot_delta_node_count;
        self.telemetry_mut().checkpoint.snapshot_restore_coarse_reason_count +=
            restore_plan.coarse_reasons.len() as u64;
        crate::diagnostics::recorder::record_snapshot_restore_lineage(
            self,
            snapshot.meta.snapshot_id,
        );
        Ok(())
    }

    pub(crate) fn materialize_explanation_artifact(
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

    pub(crate) fn materialize_provenance_artifact(
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

    pub(crate) fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
        self.record_hot_path_artifact_reconstruction();
        Ok(explanation)
    }

    pub(crate) fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = ArtifactMaterializationMode::Reconstructed;
        self.record_hot_path_artifact_reconstruction();
        Ok(ProvenanceFact::from_explanation(&explanation))
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
