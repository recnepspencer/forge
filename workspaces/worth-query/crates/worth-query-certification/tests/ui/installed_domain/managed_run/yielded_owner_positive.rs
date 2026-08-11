use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionOutcome, WorthQueryDirectYieldCleanupOutcome,
    WorthQueryDirectYieldRecoveryRequired, WorthQueryExecutionRuntime,
    WorthQueryWorkflowReadmissionOutcome, WorthQueryWorkflowYieldCleanupOutcome,
    WorthQueryWorkflowYieldCleanupPending, WorthQueryWorkflowYieldRecoveryReleaseOutcome,
    WorthQueryWorkflowYieldRecoveryReleasePending, WorthQueryWorkflowYieldRecoveryRequired,
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};
use worth_runtime_bridge::facade::RuntimeBridge;

fn inspect_direct(run: &WorthQueryYieldedDirectRun) {
    let inspection = run.inspection();
    let _ = inspection.logical_run_identity();
    let _ = inspection.yielded_attempt_identity();
    let _ = inspection.operation_binding_identity();
    let _ = inspection.installed_operation_identity();
    let _ = inspection.semantic_basis_identity();
    let _ = inspection.installation_generation();
    let _ = inspection.provider_session_identity();
    let _ = inspection.checkpoint();
    let _ = inspection.provider_work();
    let _ = inspection.run_counters();
    let _ = inspection.yield_counters();
    let _ = inspection.retained_capacity_reservation_count();
}

fn inspect_workflow(run: &WorthQueryYieldedWorkflowRun) {
    let inspection = run.inspection();
    let _ = inspection.logical_run_identity();
    let _ = inspection.yielded_attempt_identity();
    let _ = inspection.checkpoint();
    let _ = inspection.artifact_evidence();
}

fn cleanup_direct(run: WorthQueryYieldedDirectRun) -> WorthQueryDirectYieldCleanupOutcome {
    run.cleanup()
}

fn cleanup_workflow(run: WorthQueryYieldedWorkflowRun) -> WorthQueryWorkflowYieldCleanupOutcome {
    run.cleanup()
}

fn inspect_direct_cleanup(outcome: WorthQueryDirectYieldCleanupOutcome) {
    let receipt = match outcome {
        WorthQueryDirectYieldCleanupOutcome::Complete(receipt)
        | WorthQueryDirectYieldCleanupOutcome::RecoveryRequired(receipt) => receipt,
    };
    let _ = receipt.inspection().resources_released();
}

fn retry_workflow_cleanup(
    pending: WorthQueryWorkflowYieldCleanupPending,
) -> WorthQueryWorkflowYieldCleanupOutcome {
    pending.retry()
}

fn inspect_workflow_cleanup(outcome: WorthQueryWorkflowYieldCleanupOutcome) {
    match outcome {
        WorthQueryWorkflowYieldCleanupOutcome::Complete(receipt)
        | WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(receipt) => {
            let _ = receipt.inspection().artifact_evidence();
        }
        WorthQueryWorkflowYieldCleanupOutcome::Pending(pending) => {
            let _ = retry_workflow_cleanup(pending);
        }
    }
}

fn cleanup_direct_recovery(
    recovery: WorthQueryDirectYieldRecoveryRequired,
) -> Result<
    worth_query_host::facade::installed::domain_computation::WorthQueryDirectYieldCleanupReceipt,
    WorthQueryDirectYieldRecoveryRequired,
> {
    recovery.cleanup_terminalized()
}

fn cleanup_workflow_recovery(
    recovery: WorthQueryWorkflowYieldRecoveryRequired,
) -> Result<WorthQueryWorkflowYieldRecoveryReleaseOutcome, WorthQueryWorkflowYieldRecoveryRequired>
{
    recovery.release_terminalized()
}

fn retry_workflow_recovery(
    pending: WorthQueryWorkflowYieldRecoveryReleasePending,
) -> Result<WorthQueryWorkflowYieldRecoveryReleaseOutcome, WorthQueryWorkflowYieldRecoveryRequired>
{
    pending.retry()
}

fn readmit_direct(
    run: WorthQueryYieldedDirectRun,
    runtime: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryDirectReadmissionOutcome {
    run.readmit_same_runtime(runtime, bridge)
}

fn readmit_workflow(
    run: WorthQueryYieldedWorkflowRun,
    runtime: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryWorkflowReadmissionOutcome {
    run.readmit_same_runtime(runtime, bridge)
}

fn main() {}
