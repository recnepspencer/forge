use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldCleanupPending,
    WorthQueryWorkflowConvergenceYieldCleanupReceipt,
};

fn impossible<T>() -> T {
    loop {}
}

fn construct_cleanup_authority() {
    let _ = WorthQueryDirectConvergenceYieldCleanupReceipt {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldCleanupReceipt {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceYieldCleanupPending {
        association: impossible(),
    };
}

fn main() {}
