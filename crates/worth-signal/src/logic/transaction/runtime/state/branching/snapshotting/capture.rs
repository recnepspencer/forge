use crate::data::error::SignalError;
use crate::state::{
    SignalBranchHandle, SignalCheckpointImage, SignalSnapshotV1, SnapshotArtifactRetentionPolicy,
};

use super::super::super::runtime_state::SignalRuntime;
use super::super::branches::SnapshotBranchState;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn capture_snapshot(&mut self) -> Result<SignalSnapshotV1, SignalError> {
        let mut snapshot = self.graph.capture_snapshot();
        let retained_replay = self
            .graph
            .observe()
            .replay_events()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        snapshot.diagnostic_graph.clear_branch_mutation_nodes();
        snapshot.runtime_telemetry = Some(self.telemetry);
        snapshot.reconstructability = Some(
            super::super::super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                snapshot.meta.branch_id,
                snapshot.meta.snapshot_id,
                snapshot.meta.replay_head,
                super::super::super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
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
                super::super::super::reconstructability::TemporalReconstructabilityArtifact::from_temporal_state(
                    &self.temporal,
                ),
            ),
        );
        let mut branch_state = self.capture_heavy_branch_state()?;
        branch_state
            .mutation_ledger_mut()
            .clear_all(Some(snapshot.meta.snapshot_id));
        self.branches.insert_snapshot(
            SnapshotBranchState::from_branch_state(&branch_state).packet(snapshot.meta.snapshot_id),
        );
        self.branches.store_branch_state(branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(snapshot)
    }

    pub fn capture_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, SignalError> {
        if branch.id == self.graph.current_branch().id {
            return self.capture_snapshot();
        }
        let stored_state = self
            .branches
            .branch_state(branch.id)
            .ok_or_else(|| SignalError::unknown_branch(Some(branch.id), branch.name.clone()))?;
        Self::ensure_managed_queue_branch_transfer_allowed(stored_state.resource())?;
        let Some((snapshot, branch_catalog, snapshot_state)) =
            self.branches.with_stored_branch_state_mut(branch.id, |state| {
            let policy = state.graph().runtime_policy();
            let artifact_retention = SnapshotArtifactRetentionPolicy::from_runtime_policy(policy);
            let meta = state
                .graph_mut()
                .diagnostics_state_mut()
                .allocate_snapshot_meta(policy, artifact_retention);
            state
                .graph_mut()
                .diagnostics_state_mut()
                .set_branch_head_snapshot(branch.id, meta.snapshot_id);
            let diagnostics = state
                .graph()
                .diagnostics_state()
                .snapshot_payload_with_retention(artifact_retention);
            let graph_telemetry = *state.graph().telemetry();
            let retained_replay = state
                .graph()
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
                        authority: state.graph().capture_checkpoint_authority(),
                        dependency_snapshot_batch: state
                            .graph()
                            .capture_checkpoint_dependency_snapshot_batch(),
                        graph_telemetry: *state.graph().telemetry(),
                    },
                    diagnostic_graph: {
                        let mut graph = state.graph().clone_stateful();
                        graph.clear_branch_mutation_nodes();
                        graph
                    },
                    diagnostics,
                graph_telemetry,
                runtime_telemetry: Some(*state.runtime_telemetry()),
                reconstructability: Some(
                    super::super::super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                        branch.id,
                        snapshot_id,
                        replay_head,
                        super::super::super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
                            crate::data::telemetry::CheckpointTelemetry {
                                event_flushes: 0,
                                event_flush_nanos: 0,
                                checkpoint_flushes: state
                                    .checkpoint()
                                    .telemetry()
                                    .checkpoint
                                    .checkpoint_flushes,
                                checkpoint_flush_nanos: state
                                    .checkpoint()
                                    .telemetry()
                                    .checkpoint
                                    .checkpoint_flush_nanos,
                                rollback_count: 0,
                                snapshot_restore_count: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .snapshot_restore_count,
                                snapshot_restore_apply_active_policy_count: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .snapshot_restore_apply_active_policy_count,
                                snapshot_restore_shared_delta_node_count: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .snapshot_restore_shared_delta_node_count,
                                snapshot_restore_coarse_reason_count: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .snapshot_restore_coarse_reason_count,
                                checkpoint_size: state.runtime_telemetry().checkpoint.checkpoint_size,
                                journal_replay_span: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .journal_replay_span,
                                restore_authority_breadth: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .restore_authority_breadth,
                                restore_required_derived_breadth: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .restore_required_derived_breadth,
                                restore_diagnostic_richness_breadth: state
                                    .runtime_telemetry()
                                    .checkpoint
                                    .restore_diagnostic_richness_breadth,
                            },
                        ),
                        &retained_replay,
                        super::super::super::reconstructability::TemporalReconstructabilityArtifact::from_temporal_state(
                            state.temporal(),
                        ),
                    ),
                ),
            };
            let branch_catalog = state.graph().diagnostics_state().branch_catalog().clone();
            state
                .mutation_ledger_mut()
                .clear_all(Some(snapshot.meta.snapshot_id));
            (
                snapshot,
                branch_catalog,
                SnapshotBranchState::from_branch_state(state),
            )
        }) else {
            return Err(SignalError::unknown_branch(Some(branch.id), branch.name));
        };
        self.branches
            .insert_snapshot(snapshot_state.packet(snapshot.meta.snapshot_id));
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(snapshot)
    }
}
