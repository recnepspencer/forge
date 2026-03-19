use crate::data::error::SignalError;
use crate::state::{SignalBranchHandle, SignalSnapshotV1};

use super::branches::BranchState;
use super::super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let mut snapshot = self.graph.capture_snapshot();
        snapshot.graph.clear_branch_mutation_nodes();
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
                        checkpoint_size: self.telemetry.checkpoint.checkpoint_size,
                        journal_replay_span: self.telemetry.checkpoint.journal_replay_span,
                    },
                ),
            ),
        );
        let mut branch_state = self.capture_branch_state();
        branch_state
            .mutation_ledger
            .clear_all(Some(snapshot.meta.snapshot_id));
        self.branches
            .insert_snapshot(snapshot.meta.snapshot_id, branch_state.clone());
        self.branches
            .insert_branch(snapshot.meta.branch_id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        snapshot
    }

    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.graph.validate_snapshot_compatibility(snapshot)?;
        let snapshot_state = self.branches.snapshot_state(snapshot.meta.snapshot_id).cloned();
        if let Some(snapshot_state) = snapshot_state {
            let current_diagnostics = self.graph.diagnostics_state().clone();
            let mut graph = snapshot.graph.clone();
            *graph.telemetry_mut() = snapshot.graph_telemetry.clone();
            graph
                .diagnostics_state_mut()
                .restore_snapshot_payload_preserving_history_from(
                    snapshot.diagnostics.clone(),
                    &current_diagnostics,
                );
            graph.diagnostics_state_mut().set_active_branch(snapshot.meta.branch_id);
            graph.diagnostics_state_mut().set_branch_head_snapshot(
                snapshot.meta.branch_id,
                snapshot.meta.snapshot_id,
            );
            let mut state = BranchState {
                authority: super::super::reconstructability::AuthorityState {
                    graph,
                    config: snapshot_state.authority.config,
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
            let branch_catalog = state.authority.graph.diagnostics_state().branch_catalog().clone();
            self.load_branch_state(state.clone());
            self.branches.insert_branch(snapshot.meta.branch_id, state);
            self.synchronize_branch_catalogs(branch_catalog);
            return Ok(());
        }

        self.graph.restore_snapshot(snapshot)?;
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
        let (snapshot, branch_catalog, branch_state) = {
            let Some(state) = self.branches.branch_state_mut_with_allocator_sync(branch.id) else {
                return Err(SignalError::unknown_branch(Some(branch.id), branch.name));
            };
            let policy = state.authority.graph.runtime_policy();
            let meta = state
                .authority
                .graph
                .diagnostics_state_mut()
                .allocate_snapshot_meta(policy);
            state
                .authority
                .graph
                .diagnostics_state_mut()
                .set_branch_head_snapshot(branch.id, meta.snapshot_id);
            let diagnostics = state.authority.graph.diagnostics_state().snapshot_payload();
            let graph_telemetry = state.authority.graph.telemetry().clone();
            let replay_head = meta.replay_head;
            let snapshot_id = meta.snapshot_id;
            let snapshot = SignalSnapshotV1 {
                meta,
                graph: {
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
                                checkpoint_size: state.derived.telemetry.checkpoint.checkpoint_size,
                                journal_replay_span: state
                                    .derived
                                    .telemetry
                                    .checkpoint
                                    .journal_replay_span,
                            },
                        ),
                    ),
                ),
            };
            let branch_catalog = state.authority.graph.diagnostics_state().branch_catalog().clone();
            state
                .mutation_ledger
                .clear_all(Some(snapshot.meta.snapshot_id));
            (snapshot, branch_catalog, state.clone())
        };
        self.branches
            .insert_snapshot(snapshot.meta.snapshot_id, branch_state.clone());
        self.branches.insert_branch(branch.id, branch_state);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(snapshot)
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if snapshot.meta.branch_id != branch.id {
            return Err(SignalError::incompatible_snapshot(format!(
                "snapshot `{}` from branch `{}` cannot be restored into branch `{}`",
                snapshot.meta.snapshot_id.0, snapshot.meta.branch_name, branch.name
            )));
        }
        if branch.id == self.graph.current_branch().id {
            return self.restore_snapshot(snapshot);
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
        let mut graph = snapshot.graph.clone();
        *graph.telemetry_mut() = snapshot.graph_telemetry.clone();
        graph
            .diagnostics_state_mut()
            .restore_snapshot_payload_preserving_history_from(
                snapshot.diagnostics.clone(),
                &current_diagnostics,
            );
        graph.diagnostics_state_mut().set_active_branch(branch.id);
        graph
            .diagnostics_state_mut()
            .set_branch_head_snapshot(branch.id, snapshot.meta.snapshot_id);
        let state = BranchState {
            authority: super::super::reconstructability::AuthorityState {
                graph,
                config: snapshot_state.authority.config,
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
        let branch_catalog = state.authority.graph.diagnostics_state().branch_catalog().clone();
        self.branches.insert_branch(branch.id, state);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }
}
