use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceTerminal, WorthQueryWorkflowConvergenceTerminal,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunTerminal, WorthQueryManagedProviderWorkEvidence,
    WorthQueryWorkflowRunTerminal,
};

fn direct_managed(
    terminal: &WorthQueryDirectConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryDirectRunTerminal {
    terminal.managed_terminal()
}

fn workflow_managed(
    terminal: &WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryWorkflowRunTerminal {
    terminal.managed_terminal()
}

fn direct_alias(
    terminal: &WorthQueryDirectConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryDirectRunTerminal {
    terminal.run_terminal()
}

fn workflow_alias(
    terminal: &WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryWorkflowRunTerminal {
    terminal.run_terminal()
}

fn direct_provider_work(
    terminal: &WorthQueryDirectConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryManagedProviderWorkEvidence {
    terminal.provider_work()
}

fn workflow_provider_work(
    terminal: &WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>,
) -> &WorthQueryManagedProviderWorkEvidence {
    terminal.provider_work()
}

fn main() {}
