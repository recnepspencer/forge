use serde::{Deserialize, Serialize};

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
