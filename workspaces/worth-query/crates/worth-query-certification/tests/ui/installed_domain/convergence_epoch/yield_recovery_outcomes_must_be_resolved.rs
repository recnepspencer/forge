#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldRecoveryRequired,
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryPausedDirectConvergenceIteration, WorthQueryPausedWorkflowConvergenceIteration,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRecoveryRequired,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};

fn preserve_direct_recovery(
    recovery: WorthQueryDirectConvergenceYieldRecoveryRequired,
) -> WorthQueryDirectConvergenceYieldRecoveryRequired {
    recovery
}

fn preserve_workflow_recovery(
    recovery: WorthQueryWorkflowConvergenceYieldRecoveryRequired,
) -> WorthQueryWorkflowConvergenceYieldRecoveryRequired {
    recovery
}

fn preserve_workflow_cleanup(
    outcome: WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome,
) -> WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome {
    outcome
}

fn discard_outcomes(
    direct_paused: WorthQueryPausedDirectConvergenceIteration,
    direct_recovery: WorthQueryDirectConvergenceYieldRecoveryRequired,
    direct_running: WorthQueryDirectConvergenceYieldRunningRecovery,
    direct_cleanup: WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    workflow_paused: WorthQueryPausedWorkflowConvergenceIteration,
    workflow_recovery: WorthQueryWorkflowConvergenceYieldRecoveryRequired,
    workflow_running: WorthQueryWorkflowConvergenceYieldRunningRecovery,
    workflow_cleanup: WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
    workflow_pending: WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    workflow_outcome: WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome,
) {
    direct_paused.yield_iteration();
    preserve_direct_recovery(direct_recovery);
    direct_running.resume();
    direct_cleanup.finish();
    workflow_paused.yield_iteration();
    preserve_workflow_recovery(workflow_recovery);
    workflow_running.resume();
    workflow_cleanup.finish();
    workflow_pending.retry();
    preserve_workflow_cleanup(workflow_outcome);
}

fn main() {}
