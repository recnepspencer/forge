use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};

fn expose_direct(
    running: &WorthQueryDirectConvergenceYieldRunningRecovery,
    cleanup: &WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
) {
    let _ = running.managed_recovery();
    let _ = cleanup.managed_recovery();
    let _ = running.kind();
    let _ = cleanup.resource_evidence();
}

fn expose_workflow(
    running: &WorthQueryWorkflowConvergenceYieldRunningRecovery,
    cleanup: &WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
    pending: &WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
) {
    let _ = running.managed_recovery();
    let _ = cleanup.managed_recovery();
    let _ = pending.managed_pending();
    let _ = running.artifact_evidence();
    let _ = cleanup.run_counters();
}

fn main() {}
