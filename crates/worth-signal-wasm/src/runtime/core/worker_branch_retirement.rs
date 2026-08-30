use crate::boundary::errors::WorthSignalJsError;
use worth_signal::facade::history::RuntimeBranchId;

use super::worker_branch_command_model::{WorkerRetireBranchReceipt, WorkerRetireBranchRequest};
use super::worker_branch_commands::{expect_success, require_basis, unknown_branch};
use super::worker_branch_snapshot_retirement::WorkerBranchSnapshotRetirement;
use super::RuntimeCore;

impl RuntimeCore {
    pub fn retire_worker_branch(
        &mut self,
        request: WorkerRetireBranchRequest,
    ) -> Result<WorkerRetireBranchReceipt, WorthSignalJsError> {
        let terminal_basis = self.worker_branch_basis(request.branch_id)?;
        require_basis(&request.expected_basis, &terminal_basis, "retireBranch")?;
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(request.branch_id))
            .ok_or_else(|| unknown_branch(request.branch_id))?;
        let retirement_basis = self.native_branch_basis_by_id(request.branch_id)?;
        let releasing_snapshots = WorkerBranchSnapshotRetirement::admitted_for(
            &self.admitted_runtime_snapshots,
            request.branch_id,
        );
        let plan = expect_success(
            self.runtime
                .plan_signal_branch_retirement_releasing_snapshots(
                    branch,
                    retirement_basis,
                    &releasing_snapshots,
                    request.reason.into(),
                ),
            "plan worker branch retirement",
        )?;
        WorkerBranchSnapshotRetirement::release(self, &[request.branch_id]);
        let receipt = expect_success(
            self.runtime.retire_signal_branch(plan),
            "retire worker branch",
        )?;
        self.reclaim_worker_branch_companion_state(request.branch_id);
        Ok(WorkerRetireBranchReceipt {
            retired_branch_id: request.branch_id,
            parent_branch_id: receipt.parent_branch_id().0,
            terminal_basis,
            closeout_digest: receipt.closeout_digest().to_owned(),
            reclaimed_branch_state_count: receipt.reclaimed_branch_state_count(),
            reclaimed_snapshot_state_count: receipt.reclaimed_snapshot_state_count(),
            reclaimed_runtime_meta_count: receipt.reclaimed_runtime_meta_count(),
            retained_proof_record_count: receipt.retained_proof_record_count(),
        })
    }
}
