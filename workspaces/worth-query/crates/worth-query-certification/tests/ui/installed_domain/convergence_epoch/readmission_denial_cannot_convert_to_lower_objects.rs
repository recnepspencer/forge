use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionDenied,
    WorthQueryWorkflowConvergenceReadmissionDenied,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionDenied, WorthQueryWorkflowReadmissionDenied,
};

fn direct_deref<T: Deref<Target = WorthQueryDirectReadmissionDenied>>(_: &T) {}
fn workflow_deref<T: Deref<Target = WorthQueryWorkflowReadmissionDenied>>(_: &T) {}
fn direct_as_ref<T: AsRef<WorthQueryDirectReadmissionDenied>>(_: &T) {}
fn workflow_as_ref<T: AsRef<WorthQueryWorkflowReadmissionDenied>>(_: &T) {}
fn direct_borrow<T: Borrow<WorthQueryDirectReadmissionDenied>>(_: &T) {}
fn workflow_borrow<T: Borrow<WorthQueryWorkflowReadmissionDenied>>(_: &T) {}

fn extract(
    direct: WorthQueryDirectConvergenceReadmissionDenied,
    workflow: WorthQueryWorkflowConvergenceReadmissionDenied,
) {
    direct_deref(&direct);
    workflow_deref(&workflow);
    direct_as_ref(&direct);
    workflow_as_ref(&workflow);
    direct_borrow(&direct);
    workflow_borrow(&workflow);
    let _: WorthQueryDirectReadmissionDenied = direct.into();
    let _: WorthQueryWorkflowReadmissionDenied = workflow.into();
}

fn main() {}
