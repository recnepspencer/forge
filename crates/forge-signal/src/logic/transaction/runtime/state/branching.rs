use crate::data::error::SignalError;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId, SignalSnapshotV1};

use super::branches::BranchState;
use super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let mut snapshot = self.graph.capture_snapshot();
        snapshot.runtime_telemetry = Some(self.telemetry.clone());
        snapshot.reconstructability = Some(
            super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                snapshot.meta.branch_id,
                snapshot.meta.snapshot_id,
                snapshot.meta.replay_head,
                super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
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
        let branch_state = self.capture_branch_state();
        self.branches
            .insert_snapshot(snapshot.meta.snapshot_id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        snapshot
    }

    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.graph.restore_snapshot(snapshot)?;
        if let Some(telemetry) = &snapshot.runtime_telemetry {
            self.telemetry = telemetry.clone();
        }
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }

    pub fn create_branch(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SignalBranchHandle, SignalError> {
        let current_branch_name = self.graph.current_branch().name;
        let parent_branch_id = self.graph.current_branch().id;
        let handle = self.graph.diagnostics_state_mut().create_branch(name);
        let mut branch_state = self.capture_branch_state();
        branch_state
            .authority
            .graph
            .diagnostics_state_mut()
            .set_active_branch(handle.id);
        self.branches.insert_branch(handle.id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchCreated,
            None,
            format!("created branch `{}`", handle.name),
        );
        crate::diagnostics::recorder::record_branch_fork_lineage(
            &mut self.graph,
            handle.id,
            parent_branch_id,
            handle.name.clone(),
            current_branch_name.to_string(),
        );
        Ok(handle)
    }

    pub fn switch_branch(&mut self, branch: SignalBranchHandle) -> Result<(), SignalError> {
        let current = self.graph.current_branch();
        let current_state = self.capture_branch_state();
        self.branches.insert_branch(current.id, current_state);
        let Some(state) = self.branches.cloned_branch_state(branch.id) else {
            return Err(SignalError::unknown_branch(Some(branch.id), branch.name));
        };
        self.load_branch_state(state);
        self.graph
            .diagnostics_state_mut()
            .set_active_branch(branch.id);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchSwitched,
            None,
            format!("switched from `{}` to `{}`", current.name, branch.name),
        );
        crate::diagnostics::recorder::record_branch_switch_lineage(
            &mut self.graph,
            current.id,
            branch.id,
            current.name.to_string(),
            branch.name.clone(),
        );
        Ok(())
    }

    pub fn capture_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, SignalError> {
        if branch.id == self.graph.current_branch().id {
            return Ok(self.capture_snapshot());
        }
        let (snapshot, branch_catalog) = {
            let Some(state) = self
                .branches
                .branch_state_mut_with_allocator_sync(branch.id)
            else {
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
                graph: state.authority.graph.clone_stateful(),
                diagnostics,
                graph_telemetry,
                runtime_telemetry: Some(state.derived.telemetry.clone()),
                reconstructability: Some(
                    super::reconstructability::ReconstructabilityRecord::from_snapshot_boundary(
                        branch.id,
                        snapshot_id,
                        replay_head,
                        super::reconstructability::CheckpointRecord::from_checkpoint_telemetry(
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
            (snapshot, branch_catalog)
        };
        if let Some(state) = self.branches.cloned_branch_state(branch.id) {
            self.branches
                .insert_snapshot(snapshot.meta.snapshot_id, state);
        }
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
            .ok_or_else(|| SignalError::unknown_branch(Some(branch.id), branch.name))?;
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
            authority: super::reconstructability::AuthorityState {
                graph,
                config: snapshot_state.authority.config,
            },
            derived: super::reconstructability::DerivedState {
                checkpoint: snapshot_state.derived.checkpoint,
                telemetry: snapshot
                    .runtime_telemetry
                    .clone()
                    .unwrap_or(snapshot_state.derived.telemetry),
            },
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

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph.current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph.known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph
            .branch_handle(branch_id)
            .or_else(|| self.branches.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        if self.graph.branch_handle(branch_id).is_some() {
            self.graph.branch_ancestry(branch_id)
        } else {
            self.branches.branch_ancestry(branch_id)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.graph
            .branch_head_snapshot_id(branch_id)
            .or_else(|| self.branches.branch_head_snapshot_id(branch_id))
    }

    fn replay_graph_for_branch(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&crate::data::graph::SignalGraph> {
        self.branches
            .replay_graph(branch_id, self.graph.current_branch().id, &self.graph)
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> crate::diagnostics::ReplaySlice {
        self.replay_graph_for_branch(branch_id)
            .map(|graph| graph.replay_for_branch(branch_id))
            .unwrap_or_default()
    }
}
