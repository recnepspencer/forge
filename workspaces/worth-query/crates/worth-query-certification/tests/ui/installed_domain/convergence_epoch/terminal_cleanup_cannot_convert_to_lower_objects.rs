use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceCleanupFailure,
    WorthQueryDirectConvergenceCleanupReceipt, WorthQueryWorkflowConvergenceCleanupFailure,
    WorthQueryWorkflowConvergenceCleanupPending, WorthQueryWorkflowConvergenceCleanupReceipt,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupFailure, WorthQueryDirectRunCleanupReceipt,
    WorthQueryWorkflowRunCleanupFailure, WorthQueryWorkflowRunCleanupPending,
    WorthQueryWorkflowRunCleanupReceipt,
};

type DirectReceipt = WorthQueryDirectConvergenceCleanupReceipt<WorthQueryConverged>;
type DirectFailure = WorthQueryDirectConvergenceCleanupFailure<WorthQueryConverged>;
type WorkflowReceipt = WorthQueryWorkflowConvergenceCleanupReceipt<WorthQueryConverged>;
type WorkflowPending = WorthQueryWorkflowConvergenceCleanupPending<WorthQueryConverged>;
type WorkflowFailure = WorthQueryWorkflowConvergenceCleanupFailure<WorthQueryConverged>;

fn require_deref<T, Lower: ?Sized>()
where
    T: Deref<Target = Lower>,
{
}
fn require_as_ref<T, Lower: ?Sized>()
where
    T: AsRef<Lower>,
{
}
fn require_borrow<T, Lower: ?Sized>()
where
    T: Borrow<Lower>,
{
}
fn require_into<T, Lower>()
where
    T: Into<Lower>,
{
}

fn extract_lower_objects() {
    require_deref::<DirectReceipt, WorthQueryDirectRunCleanupReceipt>();
    require_as_ref::<DirectReceipt, WorthQueryDirectRunCleanupReceipt>();
    require_borrow::<DirectReceipt, WorthQueryDirectRunCleanupReceipt>();
    require_into::<DirectReceipt, WorthQueryDirectRunCleanupReceipt>();

    require_deref::<DirectFailure, WorthQueryDirectRunCleanupFailure>();
    require_as_ref::<DirectFailure, WorthQueryDirectRunCleanupFailure>();
    require_borrow::<DirectFailure, WorthQueryDirectRunCleanupFailure>();
    require_into::<DirectFailure, WorthQueryDirectRunCleanupFailure>();

    require_deref::<WorkflowReceipt, WorthQueryWorkflowRunCleanupReceipt>();
    require_as_ref::<WorkflowReceipt, WorthQueryWorkflowRunCleanupReceipt>();
    require_borrow::<WorkflowReceipt, WorthQueryWorkflowRunCleanupReceipt>();
    require_into::<WorkflowReceipt, WorthQueryWorkflowRunCleanupReceipt>();

    require_deref::<WorkflowPending, WorthQueryWorkflowRunCleanupPending>();
    require_as_ref::<WorkflowPending, WorthQueryWorkflowRunCleanupPending>();
    require_borrow::<WorkflowPending, WorthQueryWorkflowRunCleanupPending>();
    require_into::<WorkflowPending, WorthQueryWorkflowRunCleanupPending>();

    require_deref::<WorkflowFailure, WorthQueryWorkflowRunCleanupFailure>();
    require_as_ref::<WorkflowFailure, WorthQueryWorkflowRunCleanupFailure>();
    require_borrow::<WorkflowFailure, WorthQueryWorkflowRunCleanupFailure>();
    require_into::<WorkflowFailure, WorthQueryWorkflowRunCleanupFailure>();
}

fn main() {}
