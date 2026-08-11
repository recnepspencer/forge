use worth_query_host::facade::convergence_epoch::{
    WorthQueryDeniedDirectConvergenceYield, WorthQueryDeniedWorkflowConvergenceYield,
};

fn impossible<T>() -> T {
    loop {}
}

fn construct() {
    let _ = WorthQueryDeniedDirectConvergenceYield {
        association: impossible(),
    };
    let _ = WorthQueryDeniedWorkflowConvergenceYield {
        association: impossible(),
    };
}

fn main() {}
