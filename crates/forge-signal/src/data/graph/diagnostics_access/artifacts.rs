use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::explain::{explain, NodeExplanation};
use crate::state::{
    SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRetentionPolicy,
    SnapshotArtifactRestoreMode, SnapshotDependencyRestoreMode, SnapshotRestoreIntent,
    SnapshotRestoreCoarseReason, SnapshotRestorePlan,
};

impl SignalGraph {
    pub(crate) fn record_operational_diagnostic_facts(
        &mut self,
        node: NodeId,
        rewiring: Option<crate::logic::explain::RewiringSummary>,
    ) -> Result<(), SignalError> {
        let policy = self.runtime_policy();
        if !policy.retains_explanation_facts() && !policy.retains_provenance_facts() {
            return Ok(());
        }
        let entry = self.get_entry(node)?;
        let Some(runtime) = entry.get_runtime_artifact_state() else {
            return Ok(());
        };
        let eval = entry.get_eval_config();
        let retained = entry.retained_diagnostic_artifact();
        let causality = entry.get_causality();
        let explanation_fact = policy.retains_explanation_facts().then(|| {
            ExplanationFact::from_runtime_projection(
                node,
                *entry.get_state(),
                eval.contract.semantics.reads,
                eval.contract.semantics.produces,
                eval.contract.semantics.partition_scope.clone(),
                eval.contract.semantics.required_context,
                eval.condition.clone(),
                runtime,
                retained,
                causality,
                rewiring.clone(),
            )
        });
        let provenance_fact = policy.retains_provenance_facts().then(|| {
            ProvenanceFact::from_runtime_projection(
                node,
                *entry.get_state(),
                eval.contract.semantics.reads,
                eval.contract.semantics.produces,
                eval.contract.semantics.partition_scope.clone(),
                eval.contract.semantics.required_context,
                eval.condition.clone(),
                runtime,
                retained,
                causality,
                rewiring,
            )
        });
        let diagnostics = self.diagnostics_state_mut();
        if let Some(fact) = explanation_fact {
            diagnostics.record_explanation_fact(fact);
        }
        if let Some(fact) = provenance_fact {
            diagnostics.record_provenance_fact(fact);
        }
        Ok(())
    }

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
        let retention_budget = self.runtime_policy().retention_budget;
        let profile = self.diagnostics_profile();
        let history = ExecutionHistorySummary::from_graph(
            self,
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        );
        let graph_summary = GraphSummary::from_graph(
            self,
            profile,
            retention_budget.detail_limit,
            OrdinaryAccessLane,
        );
        self.diagnostics_state_mut()
            .refresh_retained_views(history, graph_summary);
        Ok(())
    }

    pub(crate) fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.record_explicit_cold_materialization_request();
        if let Some(fact) = self.explanation_fact(node) {
            let mut explanation = fact.explanation.clone();
            explanation.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
            self.record_retained_artifact_read();
            return Ok((Some(explanation), DiagnosticsAvailability::RetainedAvailable));
        }
        if matches!(
            self.runtime_policy().retention_budget.explanation_retention,
            crate::diagnostics::policy::ArtifactRetentionPolicy::Omit
        ) {
            self.record_denied_reconstruction_by_tier(true);
            return Ok((None, DiagnosticsAvailability::OmittedByTier));
        }
        if self.runtime_policy().can_reconstruct_explanation() {
            return Ok((
                Some(self.reconstruct_explanation_artifact(node)?),
                DiagnosticsAvailability::ReconstructedAvailable,
            ));
        }
        self.record_denied_reconstruction_by_budget(true);
        Ok((None, DiagnosticsAvailability::DeniedByBudget))
    }

    pub(crate) fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, DiagnosticsAvailability), SignalError> {
        self.record_explicit_cold_materialization_request();
        if let Some(fact) = self.provenance_fact(node) {
            let mut fact = fact.clone();
            fact.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
            self.record_retained_artifact_read();
            return Ok((Some(fact), DiagnosticsAvailability::RetainedAvailable));
        }
        if matches!(
            self.runtime_policy().retention_budget.provenance_retention,
            crate::diagnostics::policy::ArtifactRetentionPolicy::Omit
        ) {
            self.record_denied_reconstruction_by_tier(false);
            return Ok((None, DiagnosticsAvailability::OmittedByTier));
        }
        if self.runtime_policy().can_reconstruct_provenance() {
            return Ok((
                Some(self.reconstruct_provenance_artifact(node)?),
                DiagnosticsAvailability::ReconstructedAvailable,
            ));
        }
        self.record_denied_reconstruction_by_budget(false);
        Ok((None, DiagnosticsAvailability::DeniedByBudget))
    }

    pub(crate) fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_explanation_reconstruction();
        Ok(explanation)
    }

    pub(crate) fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        let mut explanation = explain(self, node)?;
        explanation.materialization_mode = DiagnosticsAvailability::ReconstructedAvailable;
        self.record_hot_path_artifact_reconstruction();
        self.record_cold_provenance_reconstruction();
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


