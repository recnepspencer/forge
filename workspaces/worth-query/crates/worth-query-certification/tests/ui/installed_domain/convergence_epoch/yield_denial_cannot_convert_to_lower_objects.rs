use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDeniedDirectConvergenceYield, WorthQueryDeniedWorkflowConvergenceYield,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldDenied, WorthQueryWorkflowYieldDenied,
};

fn direct_deref<T: Deref<Target = WorthQueryDirectYieldDenied>>(_: &T) {}
fn workflow_deref<T: Deref<Target = WorthQueryWorkflowYieldDenied>>(_: &T) {}
fn direct_as_ref<T: AsRef<WorthQueryDirectYieldDenied>>(_: &T) {}
fn workflow_as_ref<T: AsRef<WorthQueryWorkflowYieldDenied>>(_: &T) {}
fn direct_borrow<T: Borrow<WorthQueryDirectYieldDenied>>(_: &T) {}
fn workflow_borrow<T: Borrow<WorthQueryWorkflowYieldDenied>>(_: &T) {}

fn extract(
    direct: WorthQueryDeniedDirectConvergenceYield,
    workflow: WorthQueryDeniedWorkflowConvergenceYield,
) {
    direct_deref(&direct);
    workflow_deref(&workflow);
    direct_as_ref(&direct);
    workflow_as_ref(&workflow);
    direct_borrow(&direct);
    workflow_borrow(&workflow);
    let _: WorthQueryDirectYieldDenied = direct.into();
    let _: WorthQueryWorkflowYieldDenied = workflow.into();
}

fn main() {}
