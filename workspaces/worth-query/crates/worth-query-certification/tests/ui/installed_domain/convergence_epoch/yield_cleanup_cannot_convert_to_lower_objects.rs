use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldCleanupPending,
    WorthQueryWorkflowConvergenceYieldCleanupReceipt,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldCleanupReceipt as ManagedDirectReceipt,
    WorthQueryWorkflowYieldCleanupPending as ManagedWorkflowPending,
    WorthQueryWorkflowYieldCleanupReceipt as ManagedWorkflowReceipt,
};

fn convert_lower_objects(
    direct: WorthQueryDirectConvergenceYieldCleanupReceipt,
    workflow: WorthQueryWorkflowConvergenceYieldCleanupReceipt,
    pending: WorthQueryWorkflowConvergenceYieldCleanupPending,
) {
    let _ = <WorthQueryDirectConvergenceYieldCleanupReceipt as Deref>::deref(&direct);
    let _: &ManagedDirectReceipt = AsRef::<ManagedDirectReceipt>::as_ref(&direct);
    let _: &ManagedDirectReceipt = Borrow::<ManagedDirectReceipt>::borrow(&direct);
    let _: ManagedDirectReceipt = Into::<ManagedDirectReceipt>::into(direct);

    let _ = <WorthQueryWorkflowConvergenceYieldCleanupReceipt as Deref>::deref(&workflow);
    let _: &ManagedWorkflowReceipt = AsRef::<ManagedWorkflowReceipt>::as_ref(&workflow);
    let _: &ManagedWorkflowReceipt = Borrow::<ManagedWorkflowReceipt>::borrow(&workflow);
    let _: ManagedWorkflowReceipt = Into::<ManagedWorkflowReceipt>::into(workflow);

    let _ = <WorthQueryWorkflowConvergenceYieldCleanupPending as Deref>::deref(&pending);
    let _: &ManagedWorkflowPending = AsRef::<ManagedWorkflowPending>::as_ref(&pending);
    let _: &ManagedWorkflowPending = Borrow::<ManagedWorkflowPending>::borrow(&pending);
    let _: ManagedWorkflowPending = Into::<ManagedWorkflowPending>::into(pending);
}

fn main() {}
