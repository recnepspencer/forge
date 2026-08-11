use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionCleanupOutcome, WorthQueryDirectReadmissionCleanupReceipt,
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryWorkflowReadmissionCleanupOutcome,
    WorthQueryWorkflowReadmissionCleanupReceipt, WorthQueryWorkflowReadmissionCleanupRequired,
};

fn finish_direct(
    cleanup: WorthQueryDirectReadmissionCleanupRequired,
) -> WorthQueryDirectReadmissionCleanupReceipt {
    let mut outcome = cleanup.finish();
    loop {
        outcome = match outcome {
            WorthQueryDirectReadmissionCleanupOutcome::Complete(receipt)
            | WorthQueryDirectReadmissionCleanupOutcome::RecoveryRequired(receipt) => {
                let inspection = receipt.inspection();
                let _ = inspection.logical_run_identity();
                let _ = inspection.yielded_attempt_identity();
                let _ = inspection.disposition();
                let _ = inspection.checkpoint();
                let _ = inspection.restored_execution();
                let _ = inspection.provider_session_identity();
                let _ = inspection.resource_plan_identity();
                let _ = inspection.capacity_scope();
                let _ = inspection.released_reservation_count();
                let _ = inspection.bridge_signal_terminal();
                let _ = inspection.bridge_signal_transition_performed();
                let _ = inspection.resources_released();
                let _ = inspection.provider_work();
                let _ = inspection.run_counters();
                let _ = inspection.yield_counters();
                let _ = inspection.readmission_evidence();
                return receipt;
            }
            WorthQueryDirectReadmissionCleanupOutcome::Pending(pending) => {
                let inspection = pending.inspection();
                let _ = inspection.bridge_cleanup_pending();
                let _ = inspection.readmission_evidence();
                pending.retry()
            }
        };
    }
}

fn finish_workflow(
    cleanup: WorthQueryWorkflowReadmissionCleanupRequired,
) -> WorthQueryWorkflowReadmissionCleanupReceipt {
    let mut outcome = cleanup.finish();
    loop {
        outcome = match outcome {
            WorthQueryWorkflowReadmissionCleanupOutcome::Complete(receipt)
            | WorthQueryWorkflowReadmissionCleanupOutcome::RecoveryRequired(receipt) => {
                let inspection = receipt.inspection();
                let _ = inspection.logical_run_identity();
                let _ = inspection.yielded_attempt_identity();
                let _ = inspection.disposition();
                let _ = inspection.checkpoint();
                let _ = inspection.restored_execution();
                let _ = inspection.provider_session_identity();
                let _ = inspection.resource_plan_identity();
                let _ = inspection.capacity_scope();
                let _ = inspection.released_reservation_count();
                let _ = inspection.bridge_signal_terminal();
                let _ = inspection.bridge_signal_transition_performed();
                let _ = inspection.artifact_evidence();
                let _ = inspection.generation_rollback();
                let _ = inspection.resources_released();
                let _ = inspection.provider_work();
                let _ = inspection.run_counters();
                let _ = inspection.yield_counters();
                let _ = inspection.readmission_evidence();
                return receipt;
            }
            WorthQueryWorkflowReadmissionCleanupOutcome::Pending(pending) => {
                let inspection = pending.inspection();
                let _ = inspection.artifact_cleanup_pending();
                let _ = inspection.bridge_cleanup_pending();
                let _ = inspection.readmission_evidence();
                pending.retry()
            }
        };
    }
}

fn main() {}
