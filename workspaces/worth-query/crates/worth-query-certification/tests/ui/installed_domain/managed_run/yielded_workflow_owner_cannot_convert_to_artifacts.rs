use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowArtifactRegistryEvidence, WorthQueryYieldedWorkflowRun,
};

fn require_deref<T: Deref<Target = WorthQueryWorkflowArtifactRegistryEvidence>>(_: &T) {}
fn require_as_ref<T: AsRef<WorthQueryWorkflowArtifactRegistryEvidence>>(_: &T) {}
fn require_borrow<T: Borrow<WorthQueryWorkflowArtifactRegistryEvidence>>(_: &T) {}

fn workflow(run: WorthQueryYieldedWorkflowRun) {
    require_deref(&run);
    require_as_ref(&run);
    require_borrow(&run);
    let _: WorthQueryWorkflowArtifactRegistryEvidence = run.into();
}

fn main() {}
