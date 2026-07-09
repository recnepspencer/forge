use worth_signal::facade::history::{RuntimeBranchId, RuntimeSnapshot};

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::summaries::{ReplaySummary, RuntimeSnapshotEnvelope};

use super::RuntimeCore;

impl RuntimeCore {
    pub fn snapshot(&mut self) -> Result<RuntimeSnapshotEnvelope, WORTHSignalJsError> {
        let snapshot: RuntimeSnapshot = {
            let mut history = self.runtime.history();
            history.snapshot()
        };
        let snapshot_key = runtime_snapshot_key(&snapshot);
        self.runtime_snapshots
            .insert(snapshot_key, snapshot.clone());
        self.snapshot_states
            .insert(snapshot_key, self.snapshot_branch_state());
        Ok(RuntimeSnapshotEnvelope {
            snapshot,
            state: self.lock_store()?.snapshot(&self.catalog),
        })
    }

    pub fn restore_snapshot(
        &mut self,
        envelope: RuntimeSnapshotEnvelope,
    ) -> Result<(), WORTHSignalJsError> {
        self.ensure_callback_snapshot_availability(&envelope.state)?;
        self.runtime
            .restore_snapshot(&envelope.snapshot)
            .map_err(WORTHSignalJsError::from)?;
        self.restore_runtime_store_snapshot(envelope.state)?;
        Ok(())
    }

    pub fn replay_for_branch(
        &mut self,
        branch_id: u64,
    ) -> Result<ReplaySummary, WORTHSignalJsError> {
        let replay = self.runtime.replay_for_branch(RuntimeBranchId(branch_id));
        self.replay_summary_with_callbacks(replay)
    }

    pub fn branch_snapshot(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, WORTHSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        let mut history = self.runtime.history();
        let snapshot = history
            .branch_snapshot(branch)
            .map_err(WORTHSignalJsError::from)?;
        let snapshot_key = runtime_snapshot_key(&snapshot);
        self.runtime_snapshots
            .insert(snapshot_key, snapshot.clone());
        self.snapshot_states
            .insert(snapshot_key, self.state_for_branch(branch_id));
        Ok(snapshot)
    }

    pub fn branch_snapshot_id(&mut self, branch_id: u64) -> Result<u64, WORTHSignalJsError> {
        Ok(self.branch_snapshot(branch_id)?.meta.snapshot_id.0)
    }

    pub fn branch_snapshot_envelope(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshotEnvelope, WORTHSignalJsError> {
        let snapshot = self.branch_snapshot(branch_id)?;
        let snapshot_key = runtime_snapshot_key(&snapshot);
        let state = self
            .snapshot_states
            .get(&snapshot_key)
            .map(|state| state.store.clone())
            .ok_or_else(|| {
                WORTHSignalJsError::internal(format!(
                    "snapshot `{}:{}` missing runtime-local branch state",
                    snapshot.meta.branch_id.0, snapshot.meta.snapshot_id.0
                ))
            })?;
        Ok(RuntimeSnapshotEnvelope { snapshot, state })
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<(), WORTHSignalJsError> {
        let snapshot_key = runtime_snapshot_key(&snapshot);
        let state = self
            .snapshot_states
            .get(&snapshot_key)
            .cloned()
            .ok_or_else(|| {
                WORTHSignalJsError::internal(format!(
                    "snapshot `{}:{}` is missing runtime-local branch semantic state",
                    snapshot.meta.branch_id.0, snapshot.meta.snapshot_id.0
                ))
            })?;
        self.ensure_callback_snapshot_availability(&state.store)?;
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        self.runtime
            .restore_branch_snapshot(branch, &snapshot)
            .map_err(WORTHSignalJsError::from)?;
        self.branch_states.insert(branch_id, state.clone());
        if self.runtime.current_branch().id.0 == branch_id {
            self.restore_branch_state(state)?;
        }
        Ok(())
    }

    pub fn restore_branch_snapshot_by_id(
        &mut self,
        branch_id: u64,
        snapshot_id: u64,
    ) -> Result<(), WORTHSignalJsError> {
        let snapshot = self
            .runtime_snapshots
            .get(&(branch_id, snapshot_id))
            .cloned()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "unknown runtime snapshot `{branch_id}:{snapshot_id}`"
                ))
            })?;
        self.restore_branch_snapshot(branch_id, snapshot)
    }
}

fn runtime_snapshot_key(snapshot: &RuntimeSnapshot) -> (u64, u64) {
    (snapshot.meta.branch_id.0, snapshot.meta.snapshot_id.0)
}
