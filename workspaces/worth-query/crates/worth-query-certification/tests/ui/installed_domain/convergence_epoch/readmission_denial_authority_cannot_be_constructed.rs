use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionDenied,
    WorthQueryWorkflowConvergenceReadmissionDenied,
};

fn impossible<T>() -> T {
    loop {}
}

fn construct() {
    let _ = WorthQueryDirectConvergenceReadmissionDenied {
        association: impossible(),
    };
    let _ = WorthQueryWorkflowConvergenceReadmissionDenied {
        association: impossible(),
    };
}

fn main() {}
