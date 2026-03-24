use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::state::{
    SignalBranchHandle, SignalCheckpointImage, SignalSnapshotV1, SnapshotArtifactRestoreMode,
    SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode, SnapshotRestoreIntent,
};

use super::super::runtime_state::SignalRuntime;
use super::branches::BranchState;
use super::branches::SnapshotBranchState;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn restore_runtime_authority_from_snapshot_proof(
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
        let mut graph =
            SignalGraph::restore_from_checkpoint_authority(&snapshot.checkpoint_image.authority);
        graph.telemetry_mut().checkpoint.restore_authority_breadth +=
            graph.active_node_count() as u64;
        Ok(graph)
    }

    fn rebuild_runtime_required_derived_from_proof(
        graph: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        proof: &crate::logic::transaction::ReconstructabilityProof,
        restore_plan: &crate::state::SnapshotRestorePlan,
    ) -> Result<(), SignalError> {
        let mut rebuild_breadth = 0_u64;
        for requirement in &proof.required_rebuild {
            match requirement {
                crate::logic::transaction::RequiredDerivedRebuildSet::DependencyIndexes(_) => {
                    graph.apply_snapshot_batch_commit(
                        snapshot.checkpoint_image.dependency_snapshot_batch.clone(),
                    )?;
                    rebuild_breadth += snapshot
                        .checkpoint_image
                        .dependency_snapshot_batch
                        .target_nodes()
                        .as_slice()
                        .len() as u64;
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
                    graph.clear_branch_mutation_nodes();
                    rebuild_breadth += restore_plan.coarse_reasons.len() as u64;
                }
            }
        }
        graph
            .telemetry_mut()
            .checkpoint
            .restore_required_derived_breadth += rebuild_breadth;
        Ok(())
    }

    fn apply_runtime_diagnostic_policy_richness(
        graph: &mut SignalGraph,
        snapshot: &SignalSnapshotV1,
        current_diagnostics: &crate::diagnostics::state::DiagnosticsState,
        current_policy: crate::diagnostics::policy::SignalRuntimePolicy,
        intent: SnapshotRestoreIntent,
    ) {
        graph
            .diagnostics_state_mut()
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                current_diagnostics,
            );
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            graph.diagnostics_state_mut().set_policy(current_policy);
        }
        graph
            .telemetry_mut()
            .checkpoint
            .restore_diagnostic_richness_breadth += snapshot.diagnostics.recent_history.len()
            as u64
            + snapshot.diagnostics.replay_frames.len() as u64
            + snapshot.diagnostics.explanation_facts.len() as u64
            + snapshot.diagnostics.provenance_facts.len() as u64
            + snapshot.diagnostics.lineage_records.len() as u64;
    }

    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let mut snapshot = self.graph.capture_snapshot();
        let retained_replay = self
            .graph
            .observe()
            .replay_events()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        snapshot.diagnostic_graph.clear_branch_mutation_nodes();
        snapshot.runtime_telemetry = Some(self.telemetry.clone());
        snapshot.reconstructability = Some(
            super::super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                snapshot.meta.branch_id,
                snapshot.meta.snapshot_id,
                snapshot.meta.replay_head,
                super::super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
                    crate::data::telemetry::CheckpointTelemetry {
                        event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                        event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                        checkpoint_flushes: self
                            .checkpoint
                            .telemetry()
                            .checkpoint
                            .checkpoint_flushes,
                        checkpoint_flush_nanos: self
                            .checkpoint
                            .telemetry()
                            .checkpoint
                            .checkpoint_flush_nanos,
                        rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
                        snapshot_restore_count: self.telemetry.checkpoint.snapshot_restore_count,
                        snapshot_restore_apply_active_policy_count: self
                            .telemetry
                            .checkpoint
                            .snapshot_restore_apply_active_policy_count,
                        snapshot_restore_shared_delta_node_count: self
                            .telemetry
                            .checkpoint
                            .snapshot_restore_shared_delta_node_count,
                        snapshot_restore_coarse_reason_count: self
                            .telemetry
                            .checkpoint
                            .snapshot_restore_coarse_reason_count,
                        checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                        journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                        restore_authority_breadth: self
                            .telemetry
                            .checkpoint
                            .restore_authority_breadth,
                        restore_required_derived_breadth: self
                            .telemetry
                            .checkpoint
                            .restore_required_derived_breadth,
                        restore_diagnostic_richness_breadth: self
                            .telemetry
                            .checkpoint
                            .restore_diagnostic_richness_breadth,
                    },
                ),
                &retained_replay,
            ),
        );
        let mut branch_state = self.capture_heavy_branch_state();
        branch_state
            .mutation_ledger
            .clear_all(Some(snapshot.meta.snapshot_id));
        self.branches.insert_snapshot(
            snapshot.meta.snapshot_id,
            SnapshotBranchState::from_branch_state(&branch_state),
        );
        self.branches
            .store_branch_state(snapshot.meta.branch_id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        snapshot
    }

    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.restore_snapshot_with_intent(snapshot, SnapshotRestoreIntent::restore_runtime_truth())
    }

    pub fn restore_snapshot_with_intent(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let reconstructability_proof = snapshot.reconstructability_proof()?;
        let restore_plan = self.graph.plan_snapshot_restore(snapshot, intent)?;
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if matches!(
            intent.dependency_state,
            SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot
        ) {
            return Err(SignalError::invalid_input(
                "snapshot restore intent `SeedRecomputationFromSnapshot` is not implemented yet",
            ));
        }
        let snapshot_state = self
            .branches
            .snapshot_state(snapshot.meta.snapshot_id)
            .cloned();
        if let Some(snapshot_state) = snapshot_state {
            let current_diagnostics = self.graph.diagnostics_state().clone();
            let current_policy = current_diagnostics.policy();
            let mut graph = self.restore_runtime_authority_from_snapshot_proof(
                snapshot,
                &reconstructability_proof,
            )?;
            *graph.telemetry_mut() = snapshot.checkpoint_image.graph_telemetry.clone();
            Self::rebuild_runtime_required_derived_from_proof(
                &mut graph,
                snapshot,
                &reconstructability_proof,
                &restore_plan,
            )?;
            Self::apply_runtime_diagnostic_policy_richness(
                &mut graph,
                snapshot,
                &current_diagnostics,
                current_policy,
                intent,
            );
            graph.telemetry_mut().checkpoint.snapshot_restore_count += 1;
            if matches!(
                intent.artifacts,
                SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
            ) {
                graph
                    .telemetry_mut()
                    .checkpoint
                    .snapshot_restore_apply_active_policy_count += 1;
            }
            graph
                .diagnostics_state_mut()
                .set_active_branch(snapshot.meta.branch_id);
            graph
                .diagnostics_state_mut()
                .set_branch_head_snapshot(snapshot.meta.branch_id, snapshot.meta.snapshot_id);
            let mut state = BranchState {
                authority: super::super::reconstructability::AuthorityState {
                    graph,
                    config: snapshot_state.config,
                },
                derived: super::super::reconstructability::DerivedState {
                    checkpoint: snapshot_state.derived.checkpoint,
                    telemetry: snapshot
                        .runtime_telemetry
                        .clone()
                        .unwrap_or(snapshot_state.derived.telemetry),
                },
                ancestry: snapshot_state.ancestry,
                mutation_ledger: snapshot_state.mutation_ledger,
            };
            crate::diagnostics::recorder::record_snapshot_restore_lineage(
                &mut state.authority.graph,
                snapshot.meta.snapshot_id,
            );
            let retention_budget = state.authority.graph.runtime_policy().retention_budget;
            let profile = state.authority.graph.diagnostics_profile();
            let history = ExecutionHistorySummary::from_graph(
                &state.authority.graph,
                profile,
                retention_budget.detail_limit,
                retention_budget.retain_history_details,
                OrdinaryAccessLane,
            );
            let graph_summary = GraphSummary::from_graph(
                &state.authority.graph,
                profile,
                retention_budget.detail_limit,
                OrdinaryAccessLane,
            );
            state
                .authority
                .graph
                .diagnostics_state_mut()
                .refresh_retained_views(history, graph_summary);
            let branch_catalog = state
                .authority
                .graph
                .diagnostics_state()
                .branch_catalog()
                .clone();
            let preserved_transaction = self.telemetry.transaction;
            self.apply_branch_lifecycle_transfer(
                crate::logic::transaction::runtime::state::runtime_state::BranchLifecycleTransfer::Restore(
                    crate::logic::transaction::runtime::state::runtime_state::RestoreTransferPacket {
                    branch_id: snapshot.meta.branch_id,
                    state: state.clone(),
                    },
                ),
            )?;
            Self::merge_global_transaction_telemetry(
                preserved_transaction,
                &mut self.telemetry.transaction,
            );
            self.telemetry.checkpoint.snapshot_restore_count += 1;
            if matches!(
                intent.artifacts,
                SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
            ) {
                self.telemetry
                    .checkpoint
                    .snapshot_restore_apply_active_policy_count += 1;
            }
            self.telemetry
                .checkpoint
                .snapshot_restore_shared_delta_node_count +=
                restore_plan.dependency_snapshot_delta_node_count;
            self.telemetry
                .checkpoint
                .snapshot_restore_coarse_reason_count += restore_plan.coarse_reasons.len() as u64;
            self.branches
                .store_branch_state(snapshot.meta.branch_id, state);
            self.synchronize_branch_catalogs(branch_catalog);
            return Ok(());
        }

        self.graph.restore_snapshot_with_intent(snapshot, intent)?;
        if let Some(telemetry) = &snapshot.runtime_telemetry {
            self.telemetry = telemetry.clone();
        }
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }

    pub fn capture_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, SignalError> {
        if branch.id == self.graph.current_branch().id {
            return Ok(self.capture_snapshot());
        }
        let Some((snapshot, branch_catalog, branch_state)) =
            self.branches.with_stored_branch_state_mut(branch.id, |state| {
            let policy = state.authority.graph.runtime_policy();
            let artifact_retention = SnapshotArtifactRetentionPolicy::from_runtime_policy(policy);
            let meta = state
                .authority
                .graph
                .diagnostics_state_mut()
                .allocate_snapshot_meta(policy, artifact_retention);
            state
                .authority
                .graph
                .diagnostics_state_mut()
                .set_branch_head_snapshot(branch.id, meta.snapshot_id);
            let diagnostics = state
                .authority
                .graph
                .diagnostics_state()
                .snapshot_payload_with_retention(artifact_retention);
            let graph_telemetry = state.authority.graph.telemetry().clone();
            let retained_replay = state
                .authority
                .graph
                .observe()
                .replay_events()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            let replay_head = meta.replay_head;
            let snapshot_id = meta.snapshot_id;
            let snapshot = SignalSnapshotV1 {
                meta,
                    checkpoint_image: SignalCheckpointImage {
                        authority: state.authority.graph.capture_checkpoint_authority(),
                        dependency_snapshot_batch: state
                            .authority
                            .graph
                            .capture_checkpoint_dependency_snapshot_batch(),
                        graph_telemetry: state.authority.graph.telemetry().clone(),
                    },
                    diagnostic_graph: {
                        let mut graph = state.authority.graph.clone_stateful();
                        graph.clear_branch_mutation_nodes();
                        graph
                    },
                    diagnostics,
                graph_telemetry,
                runtime_telemetry: Some(state.derived.telemetry.clone()),
                reconstructability: Some(
                    super::super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                        branch.id,
                        snapshot_id,
                        replay_head,
                        super::super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
                            crate::data::telemetry::CheckpointTelemetry {
                                event_flushes: 0,
                                event_flush_nanos: 0,
                                checkpoint_flushes: state
                                    .derived
                                    .checkpoint
                                    .telemetry()
                                    .checkpoint
                                    .checkpoint_flushes,
                                checkpoint_flush_nanos: state
                                    .derived
                                    .checkpoint
                                    .telemetry()
                                    .checkpoint
                                    .checkpoint_flush_nanos,
                                rollback_count: 0,
                                snapshot_restore_count: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .snapshot_restore_count,
                                snapshot_restore_apply_active_policy_count: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .snapshot_restore_apply_active_policy_count,
                                snapshot_restore_shared_delta_node_count: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .snapshot_restore_shared_delta_node_count,
                                snapshot_restore_coarse_reason_count: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .snapshot_restore_coarse_reason_count,
                                checkpoint_size: state.derived.telemetry.checkpoint.checkpoint_size,
                                journal_replay_span: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .journal_replay_span,
                                restore_authority_breadth: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .restore_authority_breadth,
                                restore_required_derived_breadth: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .restore_required_derived_breadth,
                                restore_diagnostic_richness_breadth: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .restore_diagnostic_richness_breadth,
                            },
                        ),
                        &retained_replay,
                    ),
                ),
            };
            let branch_catalog = state.authority.graph.diagnostics_state().branch_catalog().clone();
            state
                .mutation_ledger
                .clear_all(Some(snapshot.meta.snapshot_id));
            (snapshot, branch_catalog, state.clone())
        }) else {
            return Err(SignalError::unknown_branch(Some(branch.id), branch.name));
        };
        self.branches.insert_snapshot(
            snapshot.meta.snapshot_id,
            SnapshotBranchState::from_branch_state(&branch_state),
        );
        self.branches.store_branch_state(branch.id, branch_state);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(snapshot)
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.restore_branch_snapshot_with_intent(
            branch,
            snapshot,
            SnapshotRestoreIntent::restore_runtime_truth(),
        )
    }

    pub fn restore_branch_snapshot_with_intent(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let reconstructability_proof = snapshot.reconstructability_proof()?;
        let restore_plan = self.graph.plan_snapshot_restore(snapshot, intent)?;
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if snapshot.meta.branch_id != branch.id {
            return Err(SignalError::incompatible_snapshot(format!(
                "snapshot `{}` from branch `{}` cannot be restored into branch `{}`",
                snapshot.meta.snapshot_id.0, snapshot.meta.branch_name, branch.name
            )));
        }
        if branch.id == self.graph.current_branch().id {
            return self.restore_snapshot_with_intent(snapshot, intent);
        }
        if matches!(
            intent.dependency_state,
            SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot
        ) {
            return Err(SignalError::invalid_input(
                "snapshot restore intent `SeedRecomputationFromSnapshot` is not implemented yet",
            ));
        }
        let snapshot_state = self
            .branches
            .snapshot_state(snapshot.meta.snapshot_id)
            .cloned()
            .ok_or_else(|| {
                SignalError::internal(format!(
                    "snapshot `{}` is missing runtime-local branch semantic state",
                    snapshot.meta.snapshot_id.0
                ))
            })?;
        let current_diagnostics = self
            .branches
            .branch_state(branch.id)
            .map(|state| state.authority.graph.diagnostics_state().clone())
            .ok_or_else(|| SignalError::unknown_branch(Some(branch.id), branch.name.clone()))?;
        let current_policy = current_diagnostics.policy();
        let mut graph = self
            .restore_runtime_authority_from_snapshot_proof(snapshot, &reconstructability_proof)?;
        *graph.telemetry_mut() = snapshot.checkpoint_image.graph_telemetry.clone();
        Self::rebuild_runtime_required_derived_from_proof(
            &mut graph,
            snapshot,
            &reconstructability_proof,
            &restore_plan,
        )?;
        Self::apply_runtime_diagnostic_policy_richness(
            &mut graph,
            snapshot,
            &current_diagnostics,
            current_policy,
            intent,
        );
        graph.telemetry_mut().checkpoint.snapshot_restore_count += 1;
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            graph
                .telemetry_mut()
                .checkpoint
                .snapshot_restore_apply_active_policy_count += 1;
        }
        graph.diagnostics_state_mut().set_active_branch(branch.id);
        graph
            .diagnostics_state_mut()
            .set_branch_head_snapshot(branch.id, snapshot.meta.snapshot_id);
        let state = BranchState {
            authority: super::super::reconstructability::AuthorityState {
                graph,
                config: snapshot_state.config,
            },
            derived: super::super::reconstructability::DerivedState {
                checkpoint: snapshot_state.derived.checkpoint,
                telemetry: snapshot
                    .runtime_telemetry
                    .clone()
                    .unwrap_or(snapshot_state.derived.telemetry),
            },
            ancestry: snapshot_state.ancestry,
            mutation_ledger: snapshot_state.mutation_ledger,
        };
        let mut state = state;
        crate::diagnostics::recorder::record_snapshot_restore_lineage(
            &mut state.authority.graph,
            snapshot.meta.snapshot_id,
        );
        let retention_budget = state.authority.graph.runtime_policy().retention_budget;
        let profile = state.authority.graph.diagnostics_profile();
        let history = ExecutionHistorySummary::from_graph(
            &state.authority.graph,
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        );
        let graph_summary = GraphSummary::from_graph(
            &state.authority.graph,
            profile,
            retention_budget.detail_limit,
            OrdinaryAccessLane,
        );
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .refresh_retained_views(history, graph_summary);
        let branch_catalog = state
            .authority
            .graph
            .diagnostics_state()
            .branch_catalog()
            .clone();
        self.telemetry.checkpoint.snapshot_restore_count += 1;
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            self.telemetry
                .checkpoint
                .snapshot_restore_apply_active_policy_count += 1;
        }
        self.telemetry
            .checkpoint
            .snapshot_restore_shared_delta_node_count +=
            restore_plan.dependency_snapshot_delta_node_count;
        self.telemetry
            .checkpoint
            .snapshot_restore_coarse_reason_count += restore_plan.coarse_reasons.len() as u64;
        self.branches.store_branch_state(branch.id, state);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }
}
