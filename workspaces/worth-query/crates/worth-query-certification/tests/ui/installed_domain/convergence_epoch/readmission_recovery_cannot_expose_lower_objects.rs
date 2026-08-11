use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionTerminalRecovery,
    WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryWorkflowConvergenceReadmissionTerminalRecovery,
    WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
};

fn expose_direct(
    reassembly: &WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
    terminal: &WorthQueryDirectConvergenceReadmissionTerminalRecovery,
) {
    let _ = reassembly.managed_recovery();
    let _ = terminal.managed_recovery();
}

fn expose_workflow(
    reassembly: &WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
    terminal: &WorthQueryWorkflowConvergenceReadmissionTerminalRecovery,
) {
    let _ = reassembly.managed_recovery();
    let _ = terminal.managed_recovery();
}

fn main() {}
