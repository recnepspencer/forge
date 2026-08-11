use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionCleanupOutcome,
    WorthQueryDirectConvergenceReadmissionRecoveryRequired,
    WorthQueryDirectConvergenceReadmissionTerminalRecovery,
    WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryDirectConvergenceYieldReassembled, WorthQueryDirectConvergenceYieldReassemblyOutcome,
    WorthQueryWorkflowConvergenceReadmissionCleanupOutcome,
    WorthQueryWorkflowConvergenceReadmissionRecoveryRequired,
    WorthQueryWorkflowConvergenceReadmissionTerminalRecovery,
    WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
    WorthQueryWorkflowConvergenceYieldReassembled,
    WorthQueryWorkflowConvergenceYieldReassemblyOutcome,
};

pub(super) fn resolve_direct(recovery: WorthQueryDirectConvergenceReadmissionRecoveryRequired) {
    match recovery {
        WorthQueryDirectConvergenceReadmissionRecoveryRequired::YieldReassembly(recovery) => {
            resolve_direct_reassembly(recovery)
        }
        WorthQueryDirectConvergenceReadmissionRecoveryRequired::TerminalCleanup(recovery) => {
            finish_direct(recovery)
        }
    }
}

fn resolve_direct_reassembly(
    recovery: WorthQueryDirectConvergenceReadmissionYieldReassemblyRecovery,
) {
    let _ = recovery.readmission_evidence();
    match recovery.retry_to_yielded() {
        WorthQueryDirectConvergenceYieldReassemblyOutcome::Yielded(reassembled) => {
            consume_direct(reassembled)
        }
        WorthQueryDirectConvergenceYieldReassemblyOutcome::RecoveryRequired(recovery) => {
            finish_direct_cleanup(recovery.into_cleanup())
        }
    }
}

fn consume_direct(reassembled: WorthQueryDirectConvergenceYieldReassembled) {
    let _ = reassembled.readmission_evidence();
    drop(reassembled.into_yielded());
}

fn finish_direct(recovery: WorthQueryDirectConvergenceReadmissionTerminalRecovery) {
    let _ = recovery.readmission_evidence();
    finish_direct_cleanup(recovery.into_cleanup());
}

fn finish_direct_cleanup(
    cleanup: worth_query_host::facade::convergence_epoch::WorthQueryDirectConvergenceReadmissionCleanupRequired,
) {
    let mut outcome = cleanup.finish();
    while let WorthQueryDirectConvergenceReadmissionCleanupOutcome::Pending(pending) = outcome {
        outcome = pending.retry();
    }
}

pub(super) fn resolve_workflow(
    recovery: WorthQueryWorkflowConvergenceReadmissionRecoveryRequired,
) {
    match recovery {
        WorthQueryWorkflowConvergenceReadmissionRecoveryRequired::YieldReassembly(recovery) => {
            resolve_workflow_reassembly(recovery)
        }
        WorthQueryWorkflowConvergenceReadmissionRecoveryRequired::TerminalCleanup(recovery) => {
            finish_workflow(recovery)
        }
    }
}

fn resolve_workflow_reassembly(
    recovery: WorthQueryWorkflowConvergenceReadmissionYieldReassemblyRecovery,
) {
    let _ = recovery.readmission_evidence();
    match recovery.retry_to_yielded() {
        WorthQueryWorkflowConvergenceYieldReassemblyOutcome::Yielded(reassembled) => {
            consume_workflow(reassembled)
        }
        WorthQueryWorkflowConvergenceYieldReassemblyOutcome::RecoveryRequired(recovery) => {
            finish_workflow_cleanup(recovery.into_cleanup())
        }
    }
}

fn consume_workflow(reassembled: WorthQueryWorkflowConvergenceYieldReassembled) {
    let _ = reassembled.readmission_evidence();
    drop(reassembled.into_yielded());
}

fn finish_workflow(recovery: WorthQueryWorkflowConvergenceReadmissionTerminalRecovery) {
    let _ = recovery.readmission_evidence();
    finish_workflow_cleanup(recovery.into_cleanup());
}

fn finish_workflow_cleanup(
    cleanup: worth_query_host::facade::convergence_epoch::WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
) {
    let mut outcome = cleanup.finish();
    while let WorthQueryWorkflowConvergenceReadmissionCleanupOutcome::Pending(pending) = outcome {
        outcome = pending.retry();
    }
}
