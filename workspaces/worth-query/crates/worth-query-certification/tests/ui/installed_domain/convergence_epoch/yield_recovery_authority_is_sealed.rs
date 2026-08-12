use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldRecoveryCleanupReceipt,
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};

fn wrong_phase_methods(
    direct_running: WorthQueryDirectConvergenceYieldRunningRecovery,
    direct_cleanup: WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    direct_receipt: WorthQueryDirectConvergenceYieldRecoveryCleanupReceipt,
    workflow_running: WorthQueryWorkflowConvergenceYieldRunningRecovery,
    workflow_cleanup: WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
    workflow_receipt: WorthQueryWorkflowConvergenceYieldRecoveryCleanupReceipt,
) {
    let _ = direct_running.finish();
    let _ = direct_cleanup.resume();
    let _ = direct_receipt.retry();
    let _ = workflow_running.finish();
    let _ = workflow_cleanup.resume();
    let _ = workflow_receipt.retry();
}

fn main() {}
