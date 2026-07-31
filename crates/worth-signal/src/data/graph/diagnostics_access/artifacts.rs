use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::logic::explain::{
    explain, CausalDisposition, CausalLink, CausalLinkKind, NodeExplanation, ScopeProvenance,
    ScopeProvenanceKind,
};
use crate::state::{
    SignalCheckpointImage, SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRestoreMode,
    SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode, SnapshotRestoreCoarseReason,
    SnapshotRestoreIntent, SnapshotRestorePlan,
};

impl SignalGraph {
    fn attach_rewiring_topology_links(
        &self,
        explanation: &mut NodeExplanation,
        rewiring: &crate::logic::explain::RewiringSummary,
    ) {
        explanation
            .causal_links
            .reserve(rewiring.removed.len() + rewiring.added.len());
        for dependency in &rewiring.removed {
            explanation.causal_links.push(CausalLink {
                source: Some(dependency.source),
                aspect: Some(dependency.aspect),
                disposition: CausalDisposition::Topology,
                kind: CausalLinkKind::DependencyRemoved,
                scope: ScopeProvenance {
                    source_scope: dependency.subscription.clone(),
                    validation_scope: dependency.subscription.clone(),
                    kind: ScopeProvenanceKind::Direct,
                    note: Some("dependency rewired away from current topology".to_string()),
                },
                cached_version: None,
                current_version: None,
                comparator: None,
                reason: None,
                note: Some("rewiring removed this dependency during apply".to_string()),
            });
        }

        for dependency in &rewiring.added {
            explanation.causal_links.push(CausalLink {
                source: Some(dependency.source),
                aspect: Some(dependency.aspect),
                disposition: CausalDisposition::Topology,
                kind: CausalLinkKind::DependencyAdded,
                scope: ScopeProvenance {
                    source_scope: dependency.subscription.clone(),
                    validation_scope: dependency.subscription.clone(),
                    kind: ScopeProvenanceKind::Direct,
                    note: Some(
                        "dependency entered the active topology during rewiring".to_string(),
                    ),
                },
                cached_version: None,
                current_version: self
                    .node_version_for_scope(
                        dependency.source,
                        dependency.aspect,
                        dependency.subscription.as_ref(),
                    )
                    .ok(),
                comparator: None,
                reason: None,
                note: Some("rewiring added this dependency during apply".to_string()),
            });
        }
    }

    fn restore_authority_from_snapshot_proof(
        &self,
        snapshot: &SignalSnapshotV1,
        proof: &crate::logic::transaction::ReconstructabilityProof,
    ) -> Result<SignalGraph, SignalError> {
        if proof.checkpoint.authority_branch_id != snapshot.meta.branch_id
            || proof.checkpoint.authority_snapshot_id != Some(snapshot.meta.snapshot_id)
        {
            return Err(SignalError::incompatible_snapshot(format!(
                "snapshot `{}` reconstructability proof does not match snapshot identity",
                snapshot.meta.snapshot_id.0
            )));
        }
        let mut restored =
            SignalGraph::restore_from_checkpoint_authority(&snapshot.checkpoint_image.authority);
        restored
            .telemetry_mut()
            .checkpoint
            .restore_authority_breadth += restored.active_node_count() as u64;
        Ok(restored)
    }

    fn rebuild_required_derived_from_snapshot_proof(
        restored: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        proof: &crate::logic::transaction::ReconstructabilityProof,
        restore_plan: &SnapshotRestorePlan,
    ) -> Result<(), SignalError> {
        let mut rebuild_breadth = 0_u64;
        for requirement in &proof.required_rebuild {
            match requirement {
                crate::logic::transaction::RequiredDerivedRebuildSet::DependencyIndexes(_) => {
                    let classified_checkpoint_batch =
                        restore_plan.checkpoint_restore_batch().clone_inner();
                    rebuild_breadth +=
                        classified_checkpoint_batch.target_nodes().as_slice().len() as u64;
                    restored.apply_classified_snapshot_batch_commit(classified_checkpoint_batch)?;
                }
                crate::logic::transaction::RequiredDerivedRebuildSet::ReplaySuffix(replay) => {
                    if snapshot.diagnostics.replay_frames.len() < replay.replay_event_count as usize
                    {
                        return Err(SignalError::incompatible_snapshot(format!(
                            "snapshot `{}` replay payload is shorter than reconstructability proof",
                            snapshot.meta.snapshot_id.0
                        )));
                    }
                    rebuild_breadth += replay.replay_event_count as u64;
                }
                crate::logic::transaction::RequiredDerivedRebuildSet::MergeSupport(_) => {
                    restored.clear_branch_mutation_nodes();
                    rebuild_breadth += restore_plan.coarse_reasons().len() as u64;
                }
                crate::logic::transaction::RequiredDerivedRebuildSet::TemporalState(temporal) => {
                    rebuild_breadth += temporal
                        .scheduled_wake_count
                        .saturating_add(temporal.ready_wake_count)
                        .saturating_add(temporal.retired_wake_count);
                }
            }
        }
        restored
            .telemetry_mut()
            .checkpoint
            .restore_required_derived_breadth += rebuild_breadth;
        Ok(())
    }

    fn apply_snapshot_diagnostic_policy_richness(
        restored: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        current_diagnostics: &crate::diagnostics::state::DiagnosticsState,
        current_policy: crate::diagnostics::policy::SignalRuntimePolicy,
        intent: SnapshotRestoreIntent,
    ) {
        restored
            .observation
            .diagnostics
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                current_diagnostics,
            );
        match intent.artifacts {
            SnapshotArtifactRestoreMode::RestoreCapturedRetention => {}
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy => {
                restored.observation.diagnostics.set_policy(current_policy);
            }
        }
        let diagnostics_breadth = snapshot.diagnostics.recent_history.len() as u64
            + snapshot.diagnostics.replay_frames.len() as u64
            + snapshot.diagnostics.explanation_facts.len() as u64
            + snapshot.diagnostics.provenance_facts.len() as u64
            + snapshot.diagnostics.lineage_records.len() as u64;
        restored
            .telemetry_mut()
            .checkpoint
            .restore_diagnostic_richness_breadth += diagnostics_breadth;
    }

    pub(crate) fn record_operational_diagnostic_facts(
        &mut self,
        node: NodeId,
        rewiring: Option<crate::logic::explain::RewiringSummary>,
    ) -> Result<(), SignalError> {
        let policy = self.runtime_policy();
        if !policy.retains_explanation_facts() && !policy.retains_provenance_facts() {
            return Ok(());
        }
        let Some(runtime) = self.node_runtime_artifact_state(node)? else {
            return Ok(());
        };
        let contract = self.get_contract(node)?.clone();
        let condition = self.node_eval_config(node)?.condition.clone();
        let state = self.get_state(node)?;
        let cold_artifact = self.node_cold_artifact_record(node)?;
        let execution_trace = self.node_execution_trace_stamp(node)?;
        let causality = self.causality_of(node)?;
        let mut compact_explanation = ExplanationFact::compact_explanation_from_runtime_projection(
            node,
            state,
            contract.semantics.reads,
            contract.semantics.produces,
            contract.semantics.partition_scope.clone(),
            contract.semantics.required_context,
            condition,
            runtime,
            cold_artifact,
            execution_trace,
            causality,
            rewiring.clone(),
        );
        if let Some(rewiring) = compact_explanation.rewiring.clone() {
            self.attach_rewiring_topology_links(&mut compact_explanation, &rewiring);
        }
        compact_explanation.materialization_mode = DiagnosticsAvailability::RetainedAvailable;
        let explanation_fact = policy.retains_explanation_facts().then(|| {
            let mut fact = ExplanationFact::from_explanation(&compact_explanation);
            fact.compact_projection = true;
            fact
        });
        let provenance_fact = policy
            .retains_provenance_facts()
            .then(|| ProvenanceFact::from_explanation(&compact_explanation));
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
        let snapshot_id = meta.snapshot_id;
        let replay_head = meta.replay_head;
        let retained_replay = self
            .observe()
            .replay_events()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        SignalSnapshotV1 {
            meta,
            checkpoint_image: SignalCheckpointImage {
                authority: self.capture_checkpoint_authority(),
                dependency_snapshot_batch: self.capture_checkpoint_dependency_snapshot_batch(),
                graph_telemetry: *self.telemetry(),
            },
            diagnostic_graph: self.clone(),
            diagnostics: self
                .diagnostics_state()
                .snapshot_payload_with_retention(artifact_retention),
            graph_telemetry: *self.telemetry(),
            runtime_telemetry: None,
            reconstructability: Some(
                crate::logic::transaction::ReconstructabilityRecord::from_snapshot_boundary(
                    self.current_branch().id,
                    snapshot_id,
                    replay_head,
                    crate::logic::transaction::CheckpointRecord::from_checkpoint_telemetry(
                        crate::data::telemetry::CheckpointTelemetry {
                            checkpoint_size: self.telemetry().checkpoint.checkpoint_size,
                            journal_replay_span: self.telemetry().checkpoint.journal_replay_span,
                            restore_authority_breadth: self
                                .telemetry()
                                .checkpoint
                                .restore_authority_breadth,
                            restore_required_derived_breadth: self
                                .telemetry()
                                .checkpoint
                                .restore_required_derived_breadth,
                            restore_diagnostic_richness_breadth: self
                                .telemetry()
                                .checkpoint
                                .restore_diagnostic_richness_breadth,
                            ..crate::data::telemetry::CheckpointTelemetry::default()
                        },
                    ),
                    &retained_replay,
                    crate::logic::transaction::TemporalReconstructabilityArtifact::default(),
                ),
            ),
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
            if snapshot.diagnostic_graph.is_alive(node) {
                shared_node_count += 1;
            } else {
                current_only_node_count += 1;
            }
        }

        let mut snapshot_only_node_count = 0_u64;
        for index in 0..SignalGraph::checkpoint_authority_arena_capacity(
            &snapshot.checkpoint_image.authority,
        ) {
            let Some(node) = SignalGraph::checkpoint_authority_live_node_id_at(
                &snapshot.checkpoint_image.authority,
                index,
            ) else {
                continue;
            };
            if !self.is_alive(node) {
                snapshot_only_node_count += 1;
            }
        }

        let dependency_snapshot_delta_batch = self
            .derive_dependency_snapshot_restore_batch_from_checkpoint_batch(
                &snapshot.checkpoint_image.authority,
                &snapshot.checkpoint_image.dependency_snapshot_batch,
            )?;
        let dependency_snapshot_delta_node_count = dependency_snapshot_delta_batch
            .target_nodes()
            .as_slice()
            .len() as u64;
        let dependency_snapshot_batch = snapshot
            .checkpoint_image
            .dependency_snapshot_batch
            .clone()
            .classify();
        let mut coarse_reasons = vec![
            SnapshotRestoreCoarseReason::EntryStateRewind,
            SnapshotRestoreCoarseReason::DiagnosticsHistoryRestore,
        ];
        if current_only_node_count > 0 || snapshot_only_node_count > 0 {
            coarse_reasons.push(SnapshotRestoreCoarseReason::NodeSetDifference);
        }

        Ok(SnapshotRestorePlan::new(
            intent,
            shared_node_count,
            current_only_node_count,
            snapshot_only_node_count,
            crate::state::CheckpointRestoreSnapshotBatch::new(dependency_snapshot_batch),
            crate::state::RestoreDeltaAccounting::new(dependency_snapshot_delta_node_count),
            true,
            coarse_reasons,
        ))
    }

    pub fn restore_snapshot_with_intent(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let reconstructability_proof = snapshot.reconstructability_proof()?;
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
        let mut restored =
            self.restore_authority_from_snapshot_proof(snapshot, &reconstructability_proof)?;
        restored.observation.telemetry = snapshot.checkpoint_image.graph_telemetry;
        Self::rebuild_required_derived_from_snapshot_proof(
            &mut restored,
            snapshot,
            &reconstructability_proof,
            &restore_plan,
        )?;
        Self::apply_snapshot_diagnostic_policy_richness(
            &mut restored,
            snapshot,
            &current_diagnostics,
            current_policy,
            intent,
        );
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
            restore_plan.dependency_snapshot_delta_node_count();
        self.telemetry_mut()
            .checkpoint
            .snapshot_restore_coarse_reason_count += restore_plan.coarse_reasons().len() as u64;
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
            return Ok((
                Some(explanation),
                DiagnosticsAvailability::RetainedAvailable,
            ));
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

    pub(crate) fn reconstruct_explanation_artifact_without_retained_fast_path(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        let mut explanation = crate::logic::explain::explain_reconstructing_with_policy_resolver(
            self, node, &resolver,
        )?;
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

    pub(crate) fn reconstruct_provenance_artifact_without_retained_fast_path(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        let mut explanation = crate::logic::explain::explain_reconstructing_with_policy_resolver(
            self, node, &resolver,
        )?;
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
