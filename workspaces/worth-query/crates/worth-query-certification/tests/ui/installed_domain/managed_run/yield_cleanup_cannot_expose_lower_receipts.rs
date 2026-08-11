use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldCleanupReceipt, WorthQueryWorkflowYieldCleanupReceipt,
    WorthQueryWorkflowYieldRecoveryRelease,
};

fn direct(receipt: &WorthQueryDirectYieldCleanupReceipt) {
    let _ = receipt.logical_run_identity();
    let _ = receipt.yielded_attempt_identity();
    let _ = receipt.checkpoint();
    let _ = receipt.checkpoint_release();
    let _ = receipt.recovery_evidence();
    let _ = receipt.bridge();
    let _ = receipt.relational();
    let _ = receipt.attempt();
    let _ = receipt.provider_work();
    let _ = receipt.run_counters();
    let _ = receipt.yield_counters();
}

fn workflow(receipt: &WorthQueryWorkflowYieldCleanupReceipt) {
    let _ = receipt.logical_run_identity();
    let _ = receipt.yielded_attempt_identity();
    let _ = receipt.checkpoint();
    let _ = receipt.checkpoint_release();
    let _ = receipt.bridge();
    let _ = receipt.relational();
    let _ = receipt.attempt();
    let _ = receipt.artifact_evidence();
    let _ = receipt.provider_work();
    let _ = receipt.run_counters();
    let _ = receipt.yield_counters();
}

fn terminalized(receipt: &WorthQueryWorkflowYieldRecoveryRelease) {
    let _ = receipt.logical_run_identity();
    let _ = receipt.yielded_attempt_identity();
    let _ = receipt.bridge();
    let _ = receipt.relational();
    let _ = receipt.attempt();
    let _ = receipt.artifact_evidence();
    let _ = receipt.provider_work();
    let _ = receipt.run_counters();
    let _ = receipt.yield_counters();
    let _ = receipt.recovery_evidence();
}

fn main() {}
