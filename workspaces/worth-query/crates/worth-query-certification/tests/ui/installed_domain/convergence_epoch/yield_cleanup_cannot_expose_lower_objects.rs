use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldCleanupPending,
    WorthQueryWorkflowConvergenceYieldCleanupReceipt,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldCleanupReceipt as ManagedDirectReceipt,
    WorthQueryWorkflowYieldCleanupPending as ManagedWorkflowPending,
    WorthQueryWorkflowYieldCleanupReceipt as ManagedWorkflowReceipt,
};

fn expose_lower_objects(
    direct: &WorthQueryDirectConvergenceYieldCleanupReceipt,
    workflow: &WorthQueryWorkflowConvergenceYieldCleanupReceipt,
    pending: &WorthQueryWorkflowConvergenceYieldCleanupPending,
) {
    let _: &ManagedDirectReceipt = direct.managed_receipt();
    let _: &ManagedWorkflowReceipt = workflow.managed_receipt();
    let _: &ManagedWorkflowPending = pending.managed_pending();
}

fn clone_lower_objects(
    direct: WorthQueryDirectConvergenceYieldCleanupReceipt,
    workflow: WorthQueryWorkflowConvergenceYieldCleanupReceipt,
    pending: WorthQueryWorkflowConvergenceYieldCleanupPending,
) {
    let _ = direct.clone();
    let _ = workflow.clone();
    let _ = pending.clone();
}

fn main() {}
