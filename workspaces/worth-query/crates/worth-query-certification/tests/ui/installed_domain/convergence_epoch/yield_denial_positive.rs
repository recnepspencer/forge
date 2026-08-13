use worth_query_host::facade::convergence_epoch::{
    WorthQueryDeniedDirectConvergenceYield, WorthQueryDeniedWorkflowConvergenceYield,
    WorthQueryDirectConvergenceStepOutcome, WorthQueryWorkflowConvergenceStepOutcome,
};

pub(super) fn resolve_direct(
    denial: WorthQueryDeniedDirectConvergenceYield,
) -> WorthQueryDirectConvergenceStepOutcome {
    denial.retry().advance()
}

pub(super) fn resolve_workflow(
    denial: WorthQueryDeniedWorkflowConvergenceYield,
) -> WorthQueryWorkflowConvergenceStepOutcome {
    denial.retry().advance()
}
