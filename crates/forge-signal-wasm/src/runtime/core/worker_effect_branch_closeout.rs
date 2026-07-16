use forge_proof::TransitionOutcome;
use forge_signal::facade::history::{
    RuntimeBranchId, SignalBranchRetirementBatchRequest, SignalBranchRetirementRequest,
};

use crate::boundary::errors::ForgeSignalJsError;

use super::worker_branch_command_model::{
    WorkerCloseoutEffectBranchReceipt, WorkerCloseoutEffectBranchRequest,
    WorkerRetireBranchReceipt, WorkerRetireBranchRequest, WorkerRetireBranchesReceipt,
    WorkerRetireBranchesRequest,
};
use super::worker_branch_commands::{expect_success, require_basis};
use super::RuntimeCore;

impl RuntimeCore {
    pub fn retire_worker_branches(
        &mut self,
        request: WorkerRetireBranchesRequest,
    ) -> Result<WorkerRetireBranchesReceipt, ForgeSignalJsError> {
        let mut terminal_bases = Vec::with_capacity(request.retirements.len());
        let mut native_requests = Vec::with_capacity(request.retirements.len());
        for retirement in &request.retirements {
            let terminal_basis = self.worker_branch_basis(retirement.branch_id)?;
            require_basis(
                &retirement.expected_basis,
                &terminal_basis,
                "retireBranches",
            )?;
            terminal_bases.push(terminal_basis);
            native_requests.push(self.native_retirement_request(retirement)?);
        }
        let plan = expect_success(
            self.runtime
                .plan_branch_retirement_batch(SignalBranchRetirementBatchRequest::new(
                    native_requests,
                )),
            "plan worker retirement batch",
        )?;
        let receipt = expect_success(
            self.runtime.retire_branch_batch(plan),
            "retire worker branch batch",
        )?;
        let retirements = request
            .retirements
            .into_iter()
            .zip(terminal_bases)
            .zip(receipt.receipts().iter().cloned())
            .map(|((retirement, terminal_basis), native_receipt)| {
                self.reclaim_worker_branch_companion_state(retirement.branch_id);
                worker_retirement_receipt(retirement, terminal_basis, native_receipt)
            })
            .collect();
        Ok(WorkerRetireBranchesReceipt { retirements })
    }

    pub fn closeout_worker_effect_branch(
        &mut self,
        request: WorkerCloseoutEffectBranchRequest,
    ) -> Result<WorkerCloseoutEffectBranchReceipt, ForgeSignalJsError> {
        let effect_request = request.effect_retirement;
        let dependency_request = request.dependency_basis_retirement;
        let terminal_basis = self.worker_branch_basis(effect_request.branch_id)?;
        require_basis(
            &effect_request.expected_basis,
            &terminal_basis,
            "closeoutEffectBranch.effectRetirement",
        )?;
        let dependency_terminal_basis = if let Some(dependency) = &dependency_request {
            let observed = self.worker_branch_basis(dependency.branch_id)?;
            require_basis(
                &dependency.expected_basis,
                &observed,
                "closeoutEffectBranch.dependencyBasisRetirement",
            )?;
            Some(observed)
        } else {
            None
        };
        let mut retirement_requests = vec![self.native_retirement_request(&effect_request)?];
        if let Some(dependency) = &dependency_request {
            retirement_requests.push(self.native_retirement_request(dependency)?);
        }
        let retirement_plan = expect_success(
            self.runtime
                .plan_branch_retirement_batch(SignalBranchRetirementBatchRequest::new(
                    retirement_requests,
                )),
            "plan worker effect closeout retirement batch",
        )?;

        let canonical_transaction =
            self.apply_transaction_to_worker_branch(request.canonical_transaction)?;
        let retired = match self.runtime.retire_branch_batch(retirement_plan) {
            TransitionOutcome::Success(receipt) => receipt,
            other => panic!(
                "prevalidated effect retirement batch must remain executable after isolated canonical transaction: {other:?}",
            ),
        };
        let receipts = retired.receipts();
        self.reclaim_worker_branch_companion_state(effect_request.branch_id);
        let effect_retirement =
            worker_retirement_receipt(effect_request, terminal_basis, receipts[0].clone());
        let dependency_basis_retirement = dependency_request.map(|dependency| {
            self.reclaim_worker_branch_companion_state(dependency.branch_id);
            worker_retirement_receipt(
                dependency,
                dependency_terminal_basis.expect("dependency basis was prevalidated"),
                receipts[1].clone(),
            )
        });
        Ok(WorkerCloseoutEffectBranchReceipt {
            canonical_transaction,
            effect_retirement,
            dependency_basis_retirement,
        })
    }

    fn native_retirement_request(
        &mut self,
        request: &WorkerRetireBranchRequest,
    ) -> Result<SignalBranchRetirementRequest, ForgeSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(request.branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "closeoutEffectBranch references an unknown retirement branch",
                )
            })?;
        let native_head = expect_success(
            self.runtime.branch_transaction_head(branch.clone()),
            "read closing worker effect branch head",
        )?;
        Ok(SignalBranchRetirementRequest::new(
            branch,
            native_head,
            request.reason.into(),
        ))
    }

    fn reclaim_worker_branch_companion_state(&mut self, branch_id: u64) {
        self.branch_states.remove(&branch_id);
        self.snapshot_states
            .retain(|(stored_branch_id, _), _| *stored_branch_id != branch_id);
        self.runtime_snapshots
            .retain(|(stored_branch_id, _), _| *stored_branch_id != branch_id);
    }
}

fn worker_retirement_receipt(
    request: WorkerRetireBranchRequest,
    terminal_basis: super::WorkerBranchBasisReceipt,
    receipt: forge_signal::facade::SignalBranchRetirementReceipt,
) -> WorkerRetireBranchReceipt {
    WorkerRetireBranchReceipt {
        retired_branch_id: request.branch_id,
        parent_branch_id: receipt.parent_branch_id().0,
        terminal_basis,
        closeout_digest: receipt.closeout_digest().to_owned(),
        reclaimed_branch_state_count: receipt.reclaimed_branch_state_count(),
        reclaimed_snapshot_state_count: receipt.reclaimed_snapshot_state_count(),
        reclaimed_runtime_meta_count: receipt.reclaimed_runtime_meta_count(),
        retained_proof_record_count: receipt.retained_proof_record_count(),
    }
}
