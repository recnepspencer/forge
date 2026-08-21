use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use worth_proof::TransitionOutcome;

use super::super::runtime_state::SignalRuntime;
use super::{
    PlannedSignalBranchRetirement, SignalBranchRetirementDenial, SignalBranchRetirementReceipt,
    SignalBranchRetirementRequest,
};
use crate::state::SignalBranchId;

#[derive(Debug, Clone)]
pub struct SignalBranchRetirementBatchRequest {
    requests: Vec<SignalBranchRetirementRequest>,
}

impl SignalBranchRetirementBatchRequest {
    pub fn new(requests: Vec<SignalBranchRetirementRequest>) -> Self {
        Self { requests }
    }

    pub fn requests(&self) -> &[SignalBranchRetirementRequest] {
        &self.requests
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchRetirementBatchDenial {
    Empty,
    DuplicateBranch {
        branch_id: SignalBranchId,
    },
    Retirement {
        position: u32,
        denial: SignalBranchRetirementDenial,
    },
}

#[derive(Debug, Clone)]
pub struct PlannedSignalBranchRetirementBatch {
    pub(super) plans: Vec<PlannedSignalBranchRetirement>,
}

impl PlannedSignalBranchRetirementBatch {
    pub fn breadth(&self) -> u32 {
        self.plans.len() as u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalBranchRetirementBatchReceipt {
    receipts: Vec<SignalBranchRetirementReceipt>,
}

impl SignalBranchRetirementBatchReceipt {
    pub fn receipts(&self) -> &[SignalBranchRetirementReceipt] {
        &self.receipts
    }

    pub(super) fn new(receipts: Vec<SignalBranchRetirementReceipt>) -> Self {
        Self { receipts }
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn plan_branch_retirement_batch(
        &mut self,
        request: SignalBranchRetirementBatchRequest,
    ) -> TransitionOutcome<PlannedSignalBranchRetirementBatch, SignalBranchRetirementBatchDenial>
    {
        if request.requests().is_empty() {
            return TransitionOutcome::denied(SignalBranchRetirementBatchDenial::Empty);
        }
        let mut scheduled = BTreeSet::new();
        let mut plans = Vec::with_capacity(request.requests().len());
        for (position, retirement) in request.requests().iter().enumerate() {
            if scheduled.contains(&retirement.branch().id) {
                return TransitionOutcome::denied(
                    SignalBranchRetirementBatchDenial::DuplicateBranch {
                        branch_id: retirement.branch().id,
                    },
                );
            }
            self.with_telemetry(|telemetry| {
                telemetry.transaction.branch_retirement_plan_count += 1
            });
            if let Err(denial) = self.validate_retirement_request_after(retirement, &scheduled) {
                self.with_telemetry(|telemetry| {
                    telemetry.transaction.branch_retirement_denial_count += 1
                });
                return TransitionOutcome::denied(SignalBranchRetirementBatchDenial::Retirement {
                    position: position as u32,
                    denial,
                });
            }
            let validated_basis = match self.branch_basis_artifact(retirement.branch().clone()) {
                TransitionOutcome::Success(basis) => basis,
                other => panic!("validated batch retirement basis must succeed: {other:?}"),
            };
            plans.push(PlannedSignalBranchRetirement {
                request: retirement.clone(),
                validated_basis,
                planned_child_membership_count: 0,
            });
            scheduled.insert(retirement.branch().id);
        }
        TransitionOutcome::success(PlannedSignalBranchRetirementBatch { plans })
    }

    pub fn retire_branch_batch(
        &mut self,
        plan: PlannedSignalBranchRetirementBatch,
    ) -> TransitionOutcome<SignalBranchRetirementBatchReceipt, SignalBranchRetirementBatchDenial>
    {
        let mut scheduled = BTreeSet::new();
        for (position, retirement) in plan.plans.iter().enumerate() {
            if let Err(denial) =
                self.validate_retirement_request_after(retirement.request(), &scheduled)
            {
                return TransitionOutcome::denied(SignalBranchRetirementBatchDenial::Retirement {
                    position: position as u32,
                    denial,
                });
            }
            scheduled.insert(retirement.request().branch().id);
        }
        let mut receipts = Vec::with_capacity(plan.plans.len());
        for retirement in plan.plans {
            match self.retire_branch(retirement) {
                TransitionOutcome::Success(receipt) => receipts.push(receipt),
                other => panic!("prevalidated retirement batch must execute atomically: {other:?}"),
            }
        }
        TransitionOutcome::success(SignalBranchRetirementBatchReceipt::new(receipts))
    }

    pub fn branch_retirement_receipt(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&SignalBranchRetirementReceipt> {
        self.branches.branch_retirement_receipt(branch_id)
    }
}
