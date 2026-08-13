use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::state::{
    SignalCheckpointImage, SignalSnapshotMeta, SignalSnapshotV1, SnapshotArtifactRetentionPolicy,
    SnapshotDependencyRestoreMode, SnapshotRestoreCoarseReason, SnapshotRestoreIntent,
    SnapshotRestorePlan,
};

impl SignalGraph {
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
            SignalGraph::restore_from_checkpoint_authority(&snapshot.checkpoint_image.authority)?;
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
        restored.readmit_checkpoint_causes()?;
        Ok(())
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
            crate::state::SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
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
        self.refresh_snapshot_summaries();
        Ok(())
    }
}
