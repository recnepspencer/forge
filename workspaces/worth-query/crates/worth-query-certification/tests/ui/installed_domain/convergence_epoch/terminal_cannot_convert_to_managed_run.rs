use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceTerminal, WorthQueryWorkflowConvergenceTerminal,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunTerminal, WorthQueryManagedProviderWorkEvidence,
    WorthQueryWorkflowRunTerminal,
};

type DirectTerminal = WorthQueryDirectConvergenceTerminal<WorthQueryConverged>;
type WorkflowTerminal = WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>;

fn require_direct_deref<T: Deref<Target = WorthQueryDirectRunTerminal>>(_: &T) {}
fn require_workflow_deref<T: Deref<Target = WorthQueryWorkflowRunTerminal>>(_: &T) {}
fn require_direct_as_ref<T: AsRef<WorthQueryDirectRunTerminal>>(_: &T) {}
fn require_workflow_as_ref<T: AsRef<WorthQueryWorkflowRunTerminal>>(_: &T) {}
fn require_direct_borrow<T: Borrow<WorthQueryDirectRunTerminal>>(_: &T) {}
fn require_workflow_borrow<T: Borrow<WorthQueryWorkflowRunTerminal>>(_: &T) {}
fn require_provider_deref<T: Deref<Target = WorthQueryManagedProviderWorkEvidence>>(_: &T) {}
fn require_provider_as_ref<T: AsRef<WorthQueryManagedProviderWorkEvidence>>(_: &T) {}
fn require_provider_borrow<T: Borrow<WorthQueryManagedProviderWorkEvidence>>(_: &T) {}
fn require_provider_into<T: Into<WorthQueryManagedProviderWorkEvidence>>() {}

fn trait_extraction(direct: DirectTerminal, workflow: WorkflowTerminal) {
    require_direct_deref(&direct);
    require_workflow_deref(&workflow);
    require_direct_as_ref(&direct);
    require_workflow_as_ref(&workflow);
    require_direct_borrow(&direct);
    require_workflow_borrow(&workflow);
    require_provider_deref(&direct);
    require_provider_deref(&workflow);
    require_provider_as_ref(&direct);
    require_provider_as_ref(&workflow);
    require_provider_borrow(&direct);
    require_provider_borrow(&workflow);
    require_provider_into::<DirectTerminal>();
    require_provider_into::<WorkflowTerminal>();
    let _: WorthQueryDirectRunTerminal = direct.into();
    let _: WorthQueryWorkflowRunTerminal = workflow.into();
}

fn main() {}
