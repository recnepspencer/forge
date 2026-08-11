use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowYieldCleanupPending, WorthQueryWorkflowYieldRecoveryReleasePending,
};

fn yielded(pending: &WorthQueryWorkflowYieldCleanupPending) {
    let _ = pending.artifact_evidence();
    let _ = pending.checkpoint();
    let _ = pending.checkpoint_release();
    let _ = pending.run_counters();
    let _ = pending.yield_counters();
}

fn terminalized(pending: &WorthQueryWorkflowYieldRecoveryReleasePending) {
    let _ = pending.artifact_evidence();
    let _ = pending.pending_artifact_owner_count();
    let _ = pending.recovery();
}

fn main() {}
