use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionTerminalRecovery,
    WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryWorkflowConvergenceReadmissionTerminalRecovery,
    WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionTerminalRecovery as ManagedDirectTerminal,
    WorthQueryDirectReadmissionYieldReassemblyRecovery as ManagedDirectReassembly,
    WorthQueryWorkflowReadmissionTerminalRecovery as ManagedWorkflowTerminal,
    WorthQueryWorkflowReadmissionYieldReassemblyRecovery as ManagedWorkflowReassembly,
};

fn convert_direct_reassembly(
    recovery: WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
) {
    let _ = <WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery as Deref>::deref(
        &recovery,
    );
    let _: &ManagedDirectReassembly = AsRef::<ManagedDirectReassembly>::as_ref(&recovery);
    let _: &ManagedDirectReassembly = Borrow::<ManagedDirectReassembly>::borrow(&recovery);
    let _: ManagedDirectReassembly = Into::<ManagedDirectReassembly>::into(recovery);
}

fn convert_direct_terminal(recovery: WorthQueryDirectConvergenceReadmissionTerminalRecovery) {
    let _ = <WorthQueryDirectConvergenceReadmissionTerminalRecovery as Deref>::deref(&recovery);
    let _: &ManagedDirectTerminal = AsRef::<ManagedDirectTerminal>::as_ref(&recovery);
    let _: &ManagedDirectTerminal = Borrow::<ManagedDirectTerminal>::borrow(&recovery);
    let _: ManagedDirectTerminal = Into::<ManagedDirectTerminal>::into(recovery);
}

fn convert_workflow_reassembly(
    recovery: WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
) {
    let _ = <WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery as Deref>::deref(
        &recovery,
    );
    let _: &ManagedWorkflowReassembly = AsRef::<ManagedWorkflowReassembly>::as_ref(&recovery);
    let _: &ManagedWorkflowReassembly = Borrow::<ManagedWorkflowReassembly>::borrow(&recovery);
    let _: ManagedWorkflowReassembly = Into::<ManagedWorkflowReassembly>::into(recovery);
}

fn convert_workflow_terminal(recovery: WorthQueryWorkflowConvergenceReadmissionTerminalRecovery) {
    let _ = <WorthQueryWorkflowConvergenceReadmissionTerminalRecovery as Deref>::deref(&recovery);
    let _: &ManagedWorkflowTerminal = AsRef::<ManagedWorkflowTerminal>::as_ref(&recovery);
    let _: &ManagedWorkflowTerminal = Borrow::<ManagedWorkflowTerminal>::borrow(&recovery);
    let _: ManagedWorkflowTerminal = Into::<ManagedWorkflowTerminal>::into(recovery);
}

fn main() {}
