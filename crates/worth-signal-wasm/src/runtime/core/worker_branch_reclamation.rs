use super::RuntimeCore;

impl RuntimeCore {
    pub(super) fn reclaim_worker_branch_companion_state(&mut self, branch_id: u64) {
        self.branch_states.remove(&branch_id);
        self.snapshot_states
            .retain(|(stored_branch_id, _), _| *stored_branch_id != branch_id);
        self.runtime_snapshots
            .retain(|(stored_branch_id, _), _| *stored_branch_id != branch_id);
        self.admitted_runtime_snapshots
            .retain(|(stored_branch_id, _), _| *stored_branch_id != branch_id);
    }
}
