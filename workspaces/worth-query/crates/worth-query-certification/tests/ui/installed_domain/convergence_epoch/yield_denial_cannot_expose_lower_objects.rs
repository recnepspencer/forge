use worth_query_host::facade::convergence_epoch::{
    WorthQueryDeniedDirectConvergenceYield, WorthQueryDeniedWorkflowConvergenceYield,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldDenied, WorthQueryWorkflowYieldDenied,
};

fn expose(
    direct: &WorthQueryDeniedDirectConvergenceYield,
    workflow: &WorthQueryDeniedWorkflowConvergenceYield,
) {
    let _: &WorthQueryDirectYieldDenied = direct.managed_denial();
    let _: &WorthQueryWorkflowYieldDenied = workflow.managed_denial();
    let _: &WorthQueryDirectYieldDenied = direct.run_denial();
    let _: &WorthQueryWorkflowYieldDenied = workflow.run_denial();
}

fn main() {}
