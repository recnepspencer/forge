use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryYieldedDirectConvergenceIteration, WorthQueryYieldedWorkflowConvergenceIteration,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn require_direct_deref<T: Deref<Target = WorthQueryYieldedDirectRun>>(_: &T) {}
fn require_workflow_deref<T: Deref<Target = WorthQueryYieldedWorkflowRun>>(_: &T) {}
fn require_direct_as_ref<T: AsRef<WorthQueryYieldedDirectRun>>(_: &T) {}
fn require_workflow_as_ref<T: AsRef<WorthQueryYieldedWorkflowRun>>(_: &T) {}
fn require_direct_borrow<T: Borrow<WorthQueryYieldedDirectRun>>(_: &T) {}
fn require_workflow_borrow<T: Borrow<WorthQueryYieldedWorkflowRun>>(_: &T) {}

fn trait_extraction(
    direct: WorthQueryYieldedDirectConvergenceIteration,
    workflow: WorthQueryYieldedWorkflowConvergenceIteration,
) {
    require_direct_deref(&direct);
    require_workflow_deref(&workflow);
    require_direct_as_ref(&direct);
    require_workflow_as_ref(&workflow);
    require_direct_borrow(&direct);
    require_workflow_borrow(&workflow);
    let _: WorthQueryYieldedDirectRun = direct.into();
    let _: WorthQueryYieldedWorkflowRun = workflow.into();
}

fn main() {}
