use worth_signal::facade::history::RuntimeBranch;
use worth_signal::facade::history::RuntimeBranchId;

use crate::boundary::errors::WORTHSignalJsError;

use super::state::{BranchRuntimeMetadata, BranchRuntimeState};
use super::RuntimeCore;

impl RuntimeCore {
    pub fn current_branch(&self) -> RuntimeBranch {
        self.runtime.current_branch()
    }

    pub fn branches(&self) -> Vec<RuntimeBranch> {
        self.runtime.known_branches()
    }

    pub fn create_branch(&mut self, name: String) -> Result<RuntimeBranch, WORTHSignalJsError> {
        let state = self.snapshot_branch_state();
        let branch = self
            .runtime
            .create_branch(name)
            .map_err(WORTHSignalJsError::from)?;
        self.branch_states.insert(branch.id.0, state);
        Ok(branch)
    }

    pub fn switch_branch(&mut self, branch_id: u64) -> Result<(), WORTHSignalJsError> {
        let current_branch_id = self.runtime.current_branch().id.0;
        let current_state = self.snapshot_branch_state();
        self.branch_states
            .insert(current_branch_id, current_state.clone());
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        let target_state = self
            .branch_states
            .get(&branch_id)
            .cloned()
            .unwrap_or(current_state);
        self.ensure_callback_snapshot_availability(&target_state.store)?;
        self.runtime
            .switch_branch(branch)
            .map_err(WORTHSignalJsError::from)?;
        self.restore_branch_state(target_state)?;
        Ok(())
    }

    pub(super) fn snapshot_branch_metadata(&self) -> BranchRuntimeMetadata {
        BranchRuntimeMetadata {
            catalog: self.catalog.clone(),
            nodes_by_id: self.nodes_by_id.clone(),
            dense_grids: self.dense_grids.clone(),
        }
    }

    pub(super) fn snapshot_branch_state(&self) -> BranchRuntimeState {
        BranchRuntimeState {
            metadata: self.snapshot_branch_metadata(),
            store: self
                .lock_store()
                .map(|store| store.snapshot(&self.catalog))
                .unwrap_or_default(),
        }
    }

    pub(super) fn restore_branch_metadata(&mut self, metadata: BranchRuntimeMetadata) {
        self.catalog = metadata.catalog;
        self.nodes_by_id = metadata.nodes_by_id;
        self.dense_grids = metadata.dense_grids;
    }

    pub(super) fn restore_branch_state(
        &mut self,
        state: BranchRuntimeState,
    ) -> Result<(), WORTHSignalJsError> {
        self.restore_branch_metadata(state.metadata);
        self.restore_runtime_store_snapshot(state.store)
    }

    pub(super) fn state_for_branch(&self, branch_id: u64) -> BranchRuntimeState {
        if self.runtime.current_branch().id.0 == branch_id {
            self.snapshot_branch_state()
        } else {
            self.branch_states
                .get(&branch_id)
                .cloned()
                .unwrap_or_default()
        }
    }
}
