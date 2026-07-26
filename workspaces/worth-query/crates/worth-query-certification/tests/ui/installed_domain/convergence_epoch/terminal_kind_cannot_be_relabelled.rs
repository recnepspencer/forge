use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceIterationOutcome, WorthQueryWorkflowConvergenceIterationOutcome,
};

fn relabel_cancelled_as_converged(
    outcome: WorthQueryDirectConvergenceIterationOutcome,
) -> WorthQueryDirectConvergenceIterationOutcome {
    match outcome {
        WorthQueryDirectConvergenceIterationOutcome::Cancelled(terminal) => {
            WorthQueryDirectConvergenceIterationOutcome::Converged(terminal)
        }
        other => other,
    }
}

fn relabel_cancelled_workflow_as_converged(
    outcome: WorthQueryWorkflowConvergenceIterationOutcome,
) -> WorthQueryWorkflowConvergenceIterationOutcome {
    match outcome {
        WorthQueryWorkflowConvergenceIterationOutcome::Cancelled(terminal) => {
            WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal)
        }
        other => other,
    }
}

fn main() {}
