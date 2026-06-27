use worth_ui::facade::WorthUiFileRustReplacementSemanticReceipt;

fn main() {
    let _receipt = WorthUiFileRustReplacementSemanticReceipt {
        file_next_artifact_digest: 1,
        rust_next_artifact_digest: 1,
        file_next_plan_digest: 2,
        rust_next_plan_digest: 2,
        file_candidate_plan_digest: 2,
        rust_candidate_plan_digest: 2,
        file_reconciliation_basis_digest: 3,
        rust_reconciliation_basis_digest: 3,
        file_query_rebind_basis_digest: 4,
        rust_query_rebind_basis_digest: 4,
        file_lane_support_digest: 5,
        rust_lane_support_digest: 5,
        file_lane_parity_reference_digest: None,
        rust_lane_parity_reference_digest: None,
        file_swap_receipt: uninitialized_field(),
        rust_swap_receipt: uninitialized_field(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
