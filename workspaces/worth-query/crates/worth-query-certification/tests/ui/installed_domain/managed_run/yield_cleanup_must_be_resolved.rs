#![deny(unused_must_use)]
#![allow(path_statements)]

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectYieldCleanupOutcome, WorthQueryDirectYieldRecoveryRequired,
    WorthQueryWorkflowYieldCleanupOutcome, WorthQueryWorkflowYieldCleanupPending,
    WorthQueryWorkflowYieldRecoveryReleaseOutcome, WorthQueryWorkflowYieldRecoveryReleasePending,
    WorthQueryWorkflowYieldRecoveryRequired,
};

fn direct_outcome(value: WorthQueryDirectYieldCleanupOutcome) {
    std::convert::identity(value);
}
fn workflow_outcome(value: WorthQueryWorkflowYieldCleanupOutcome) {
    std::convert::identity(value);
}
fn workflow_pending(value: WorthQueryWorkflowYieldCleanupPending) {
    std::convert::identity(value);
}
fn direct_recovery(value: WorthQueryDirectYieldRecoveryRequired) {
    std::convert::identity(value);
}
fn workflow_recovery(value: WorthQueryWorkflowYieldRecoveryRequired) {
    std::convert::identity(value);
}
fn recovery_outcome(value: WorthQueryWorkflowYieldRecoveryReleaseOutcome) {
    std::convert::identity(value);
}
fn recovery_pending(value: WorthQueryWorkflowYieldRecoveryReleasePending) {
    std::convert::identity(value);
}
fn retry_workflow(value: WorthQueryWorkflowYieldCleanupPending) {
    value.retry();
}
fn retry_recovery(value: WorthQueryWorkflowYieldRecoveryReleasePending) {
    value.retry();
}
fn cleanup_direct_recovery(value: WorthQueryDirectYieldRecoveryRequired) {
    value.cleanup_terminalized();
}
fn cleanup_workflow_recovery(value: WorthQueryWorkflowYieldRecoveryRequired) {
    value.release_terminalized();
}

fn main() {}
