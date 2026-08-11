#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionDenied,
    WorthQueryWorkflowConvergenceReadmissionDenied,
};

fn preserve_direct(
    denial: WorthQueryDirectConvergenceReadmissionDenied,
) -> WorthQueryDirectConvergenceReadmissionDenied {
    denial
}

fn preserve_workflow(
    denial: WorthQueryWorkflowConvergenceReadmissionDenied,
) -> WorthQueryWorkflowConvergenceReadmissionDenied {
    denial
}

fn discard(
    direct: WorthQueryDirectConvergenceReadmissionDenied,
    direct_transition: WorthQueryDirectConvergenceReadmissionDenied,
    workflow: WorthQueryWorkflowConvergenceReadmissionDenied,
    workflow_transition: WorthQueryWorkflowConvergenceReadmissionDenied,
) {
    preserve_direct(direct);
    direct_transition.into_yielded();
    preserve_workflow(workflow);
    workflow_transition.into_yielded();
}

fn main() {}
