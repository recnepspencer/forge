use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowRunCleanupFailure, WorthQueryWorkflowRunCleanupInspection,
    WorthQueryWorkflowRunCleanupPending, WorthQueryWorkflowRunCleanupReceipt,
};

fn rebuild_receipt(
    value: WorthQueryWorkflowRunCleanupReceipt,
) -> WorthQueryWorkflowRunCleanupReceipt {
    WorthQueryWorkflowRunCleanupReceipt { ..value }
}

fn rebuild_inspection(
    value: WorthQueryWorkflowRunCleanupInspection,
) -> WorthQueryWorkflowRunCleanupInspection {
    WorthQueryWorkflowRunCleanupInspection { ..value }
}

fn rebuild_pending(
    value: WorthQueryWorkflowRunCleanupPending,
) -> WorthQueryWorkflowRunCleanupPending {
    WorthQueryWorkflowRunCleanupPending { ..value }
}

fn rebuild_failure(
    value: WorthQueryWorkflowRunCleanupFailure,
) -> WorthQueryWorkflowRunCleanupFailure {
    WorthQueryWorkflowRunCleanupFailure { ..value }
}

fn main() {}
