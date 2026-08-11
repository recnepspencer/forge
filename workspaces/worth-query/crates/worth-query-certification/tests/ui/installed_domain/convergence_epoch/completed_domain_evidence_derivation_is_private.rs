use worth_query_host::facade::installed::domain_computation::WorthQueryBoundGraphExecutionReceipt;

fn bypass_completed_owner(receipt: &WorthQueryBoundGraphExecutionReceipt) {
    let _ = receipt.derive_direct_convergence_evidence((), "semantic-key");
}

fn main() {}
