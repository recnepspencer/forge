use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldRecoveryRequired, WorthQueryWorkflowYieldRecoveryReleasePending,
    WorthQueryWorkflowYieldRecoveryRequired,
};

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

fn conversions_are_absent() {
    require_deref::<WorthQueryDirectConvergenceYieldRunningRecovery, WorthQueryDirectYieldRecoveryRequired>();
    require_as_ref::<WorthQueryDirectConvergenceYieldTerminalCleanupRequired, WorthQueryDirectYieldRecoveryRequired>();
    require_borrow::<WorthQueryWorkflowConvergenceYieldRunningRecovery, WorthQueryWorkflowYieldRecoveryRequired>();
    require_into::<WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired, WorthQueryWorkflowYieldRecoveryRequired>();
    require_into::<WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending, WorthQueryWorkflowYieldRecoveryReleasePending>();
}

fn clones_are_absent(
    direct: WorthQueryDirectConvergenceYieldRunningRecovery,
    workflow: WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
) {
    let _ = direct.clone();
    let _ = workflow.clone();
}

fn main() {}
