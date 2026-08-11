use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowRunCleanupOutcome, WorthQueryWorkflowRunCleanupReceipt,
    WorthQueryWorkflowRunTerminal,
};

fn finish(mut outcome: WorthQueryWorkflowRunCleanupOutcome) -> WorthQueryWorkflowRunCleanupReceipt {
    loop {
        outcome = match outcome {
            WorthQueryWorkflowRunCleanupOutcome::Complete(receipt) => {
                let inspection = receipt.inspection();
                let _ = inspection.run_identity();
                let _ = inspection.logical_run_identity();
                let _ = inspection.terminal();
                let _ = inspection.disposition();
                let _ = inspection.provider_session_identity();
                let _ = inspection.resource_plan_identity();
                let _ = inspection.capacity_scope();
                let _ = inspection.released_reservation_count();
                let _ = inspection.resources_released();
                let _ = inspection.artifact_evidence();
                let _ = inspection.provider_work();
                let _ = inspection.counters();
                return receipt;
            }
            WorthQueryWorkflowRunCleanupOutcome::Pending(pending) => pending.retry(),
            WorthQueryWorkflowRunCleanupOutcome::RecoveryRequired(failure) => failure.retry(),
        };
    }
}

fn cleanup(terminal: WorthQueryWorkflowRunTerminal) -> WorthQueryWorkflowRunCleanupReceipt {
    finish(terminal.cleanup())
}

fn main() {}
