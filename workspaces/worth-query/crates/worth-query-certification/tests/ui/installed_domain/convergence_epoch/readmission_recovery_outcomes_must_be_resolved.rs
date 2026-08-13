#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionRecoveryRequired,
    WorthQueryDirectConvergenceReadmissionTerminalRecovery,
    WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryDirectConvergenceYieldReassembled, WorthQueryDirectConvergenceYieldReassemblyOutcome,
    WorthQueryWorkflowConvergenceReadmissionRecoveryRequired,
    WorthQueryWorkflowConvergenceReadmissionTerminalRecovery,
    WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryWorkflowConvergenceYieldReassembled,
    WorthQueryWorkflowConvergenceYieldReassemblyOutcome,
};

fn impossible<T>() -> T {
    loop {}
}

fn discard_direct() {
    impossible::<WorthQueryDirectConvergenceReadmissionRecoveryRequired>();
    impossible::<WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery>();
    impossible::<WorthQueryDirectConvergenceReadmissionTerminalRecovery>();
    impossible::<WorthQueryDirectConvergenceYieldReassemblyOutcome>();
    impossible::<WorthQueryDirectConvergenceYieldReassembled>();
}

fn discard_workflow() {
    impossible::<WorthQueryWorkflowConvergenceReadmissionRecoveryRequired>();
    impossible::<WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery>();
    impossible::<WorthQueryWorkflowConvergenceReadmissionTerminalRecovery>();
    impossible::<WorthQueryWorkflowConvergenceYieldReassemblyOutcome>();
    impossible::<WorthQueryWorkflowConvergenceYieldReassembled>();
}

fn main() {}
