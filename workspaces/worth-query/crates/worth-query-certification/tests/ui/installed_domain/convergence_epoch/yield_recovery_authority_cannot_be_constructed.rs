use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldRecoveryCleanupReceipt,
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};

fn private_construction() {
    let _ = WorthQueryDirectConvergenceYieldRunningRecovery {
        association: impossible(),
    };
    let _ = WorthQueryDirectConvergenceYieldTerminalCleanupRequired {
        association: impossible(),
    };
    let _ = WorthQueryDirectConvergenceYieldRecoveryCleanupReceipt {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldRunningRecovery {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldRecoveryCleanupReceipt {
        association: impossible(),
    };
}

fn impossible<T>() -> T {
    loop {}
}

fn main() {}
