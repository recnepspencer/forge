use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowRunCleanupInspection, WorthQueryWorkflowRunCleanupReceipt,
};

fn inspect(
    receipt: &WorthQueryWorkflowRunCleanupReceipt,
    inspection: &WorthQueryWorkflowRunCleanupInspection,
) {
    let _ = receipt.bridge();
    let _ = receipt.relational();
    let _ = receipt.attempt();
    let _ = inspection.bridge();
    let _ = inspection.relational();
    let _ = inspection.attempt();
}

fn main() {}
