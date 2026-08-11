#![deny(unused_must_use)]

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionCleanupOutcome, WorthQueryDirectReadmissionCleanupPending,
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryWorkflowReadmissionCleanupOutcome,
    WorthQueryWorkflowReadmissionCleanupPending, WorthQueryWorkflowReadmissionCleanupRequired,
};

fn direct_outcome(value: WorthQueryDirectReadmissionCleanupOutcome) {
    std::convert::identity(value);
}
fn workflow_outcome(value: WorthQueryWorkflowReadmissionCleanupOutcome) {
    std::convert::identity(value);
}
fn direct_required(value: WorthQueryDirectReadmissionCleanupRequired) {
    std::convert::identity(value);
}
fn workflow_required(value: WorthQueryWorkflowReadmissionCleanupRequired) {
    std::convert::identity(value);
}
fn direct_pending(value: WorthQueryDirectReadmissionCleanupPending) {
    std::convert::identity(value);
}
fn workflow_pending(value: WorthQueryWorkflowReadmissionCleanupPending) {
    std::convert::identity(value);
}
fn finish_direct(value: WorthQueryDirectReadmissionCleanupRequired) {
    value.finish();
}
fn finish_workflow(value: WorthQueryWorkflowReadmissionCleanupRequired) {
    value.finish();
}
fn retry_direct(value: WorthQueryDirectReadmissionCleanupPending) {
    value.retry();
}
fn retry_workflow(value: WorthQueryWorkflowReadmissionCleanupPending) {
    value.retry();
}

fn main() {}
