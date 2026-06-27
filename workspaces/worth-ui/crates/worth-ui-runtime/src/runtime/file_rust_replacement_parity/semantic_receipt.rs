use crate::runtime::{WorthUiFileRustReplacementPipelineReport, WorthUiPlanSwapReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFileRustReplacementSemanticReceipt {
    file_next_artifact_digest: u64,
    rust_next_artifact_digest: u64,
    file_next_plan_digest: u64,
    rust_next_plan_digest: u64,
    file_candidate_plan_digest: u64,
    rust_candidate_plan_digest: u64,
    file_reconciliation_basis_digest: u64,
    rust_reconciliation_basis_digest: u64,
    file_query_rebind_basis_digest: u64,
    rust_query_rebind_basis_digest: u64,
    file_lane_support_digest: u64,
    rust_lane_support_digest: u64,
    file_lane_parity_reference_digest: Option<u64>,
    rust_lane_parity_reference_digest: Option<u64>,
    file_swap_receipt: WorthUiPlanSwapReceipt,
    rust_swap_receipt: WorthUiPlanSwapReceipt,
}

impl WorthUiFileRustReplacementSemanticReceipt {
    pub(crate) fn from_reports(
        file: &WorthUiFileRustReplacementPipelineReport,
        rust: &WorthUiFileRustReplacementPipelineReport,
    ) -> Self {
        let file_swap = file.swap_receipt();
        let rust_swap = rust.swap_receipt();
        Self {
            file_next_artifact_digest: file_swap.next_active_artifact_digest(),
            rust_next_artifact_digest: rust_swap.next_active_artifact_digest(),
            file_next_plan_digest: file_swap.next_active_plan_digest(),
            rust_next_plan_digest: rust_swap.next_active_plan_digest(),
            file_candidate_plan_digest: file.candidate_plan_digest(),
            rust_candidate_plan_digest: rust.candidate_plan_digest(),
            file_reconciliation_basis_digest: file_swap.reconciliation_basis_digest(),
            rust_reconciliation_basis_digest: rust_swap.reconciliation_basis_digest(),
            file_query_rebind_basis_digest: file_swap.query_rebind_basis_digest(),
            rust_query_rebind_basis_digest: rust_swap.query_rebind_basis_digest(),
            file_lane_support_digest: file.lane_support_digest(),
            rust_lane_support_digest: rust.lane_support_digest(),
            file_lane_parity_reference_digest: file_swap.lane_parity_semantic_reference_digest(),
            rust_lane_parity_reference_digest: rust_swap.lane_parity_semantic_reference_digest(),
            file_swap_receipt: file_swap,
            rust_swap_receipt: rust_swap,
        }
    }

    pub(crate) fn artifact_digests_match(self) -> bool {
        self.file_next_artifact_digest == self.rust_next_artifact_digest
    }

    pub(crate) fn plan_digests_match(self) -> bool {
        self.file_next_plan_digest == self.rust_next_plan_digest
            && self.file_candidate_plan_digest == self.file_next_plan_digest
            && self.rust_candidate_plan_digest == self.rust_next_plan_digest
    }

    pub(crate) fn activation_receipts_match(self) -> bool {
        self.file_swap_receipt == self.rust_swap_receipt
            && self.file_reconciliation_basis_digest == self.rust_reconciliation_basis_digest
            && self.file_query_rebind_basis_digest == self.rust_query_rebind_basis_digest
    }

    pub(crate) fn lane_receipts_match(self) -> bool {
        self.file_lane_support_digest == self.rust_lane_support_digest
            && self.file_lane_parity_reference_digest == self.rust_lane_parity_reference_digest
    }

    pub fn file_next_artifact_digest(self) -> u64 {
        self.file_next_artifact_digest
    }

    pub fn rust_next_artifact_digest(self) -> u64 {
        self.rust_next_artifact_digest
    }

    pub fn file_next_plan_digest(self) -> u64 {
        self.file_next_plan_digest
    }

    pub fn rust_next_plan_digest(self) -> u64 {
        self.rust_next_plan_digest
    }

    pub fn file_reconciliation_basis_digest(self) -> u64 {
        self.file_reconciliation_basis_digest
    }

    pub fn rust_reconciliation_basis_digest(self) -> u64 {
        self.rust_reconciliation_basis_digest
    }

    pub fn file_query_rebind_basis_digest(self) -> u64 {
        self.file_query_rebind_basis_digest
    }

    pub fn rust_query_rebind_basis_digest(self) -> u64 {
        self.rust_query_rebind_basis_digest
    }

    pub fn file_swap_receipt(self) -> WorthUiPlanSwapReceipt {
        self.file_swap_receipt
    }

    pub fn rust_swap_receipt(self) -> WorthUiPlanSwapReceipt {
        self.rust_swap_receipt
    }
}
