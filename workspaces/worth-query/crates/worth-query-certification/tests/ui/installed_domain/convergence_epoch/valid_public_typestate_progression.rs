use worth_query_host::facade::convergence_epoch::{
    WorthQueryAdmittedDirectConvergenceEpoch, WorthQueryAdmittedWorkflowConvergenceEpoch,
    WorthQueryBoundConvergenceReport, WorthQueryConverged,
    WorthQueryDirectConvergenceIterationStartRejection,
    WorthQueryDirectConvergenceReadmissionCleanupOutcome,
    WorthQueryDirectConvergenceReadmissionCleanupRequired,
    WorthQueryDirectConvergenceReadmissionOutcome, WorthQueryDirectConvergenceStepOutcome,
    WorthQueryDirectConvergenceTerminal, WorthQueryDirectConvergenceYieldCleanupOutcome,
    WorthQueryDirectConvergenceYieldCleanupReceipt,
    WorthQueryDirectConvergenceYieldRecoveryRequired,
    WorthQueryDirectConvergenceYieldRunningRecovery,
    WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
    WorthQueryIteratingDirectConvergenceEpoch, WorthQueryIteratingWorkflowConvergenceEpoch,
    WorthQueryRetainedConvergenceCandidateEvidence, WorthQueryStartedDirectConvergenceIteration,
    WorthQueryStartedWorkflowConvergenceIteration, WorthQueryWorkflowConvergenceCleanupOutcome,
    WorthQueryWorkflowConvergenceIterationStartRejection,
    WorthQueryWorkflowConvergenceReadmissionCleanupOutcome,
    WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
    WorthQueryWorkflowConvergenceReadmissionOutcome, WorthQueryWorkflowConvergenceStartRejection,
    WorthQueryWorkflowConvergenceStepOutcome, WorthQueryWorkflowConvergenceTerminal,
    WorthQueryWorkflowConvergenceYieldCleanupOutcome, WorthQueryYieldedDirectConvergenceIteration,
    WorthQueryYieldedWorkflowConvergenceIteration,
    WorthQueryWorkflowConvergenceYieldCleanupReceipt,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome,
    WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
    WorthQueryWorkflowConvergenceYieldRecoveryRequired,
    WorthQueryWorkflowConvergenceYieldRunningRecovery,
    WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
};
use worth_query_host::facade::installed::domain_computation::WorthQueryManagedGraphCallRequest;
use worth_query_host::facade::runtime::WorthQueryExecutionRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

mod readmission_recovery_positive;
mod readmission_denial_positive;
mod yield_denial_positive;

fn start(
    admitted: WorthQueryAdmittedDirectConvergenceEpoch,
) -> WorthQueryIteratingDirectConvergenceEpoch {
    admitted.start()
}

fn begin(
    iterating: WorthQueryIteratingDirectConvergenceEpoch,
    request: WorthQueryManagedGraphCallRequest,
) -> Result<
    WorthQueryStartedDirectConvergenceIteration,
    WorthQueryDirectConvergenceIterationStartRejection,
> {
    iterating.begin_iteration(request)
}

fn advance(
    started: WorthQueryStartedDirectConvergenceIteration,
) -> WorthQueryDirectConvergenceStepOutcome {
    started.advance()
}

fn start_workflow(
    admitted: WorthQueryAdmittedWorkflowConvergenceEpoch,
) -> Result<WorthQueryIteratingWorkflowConvergenceEpoch, WorthQueryWorkflowConvergenceStartRejection>
{
    admitted.start()
}

fn begin_workflow(
    iterating: WorthQueryIteratingWorkflowConvergenceEpoch,
    stage_identity: &str,
    request: WorthQueryManagedGraphCallRequest,
) -> Result<
    WorthQueryStartedWorkflowConvergenceIteration,
    WorthQueryWorkflowConvergenceIterationStartRejection,
> {
    iterating.begin_stage_iteration(stage_identity, request)
}

fn advance_workflow(
    started: WorthQueryStartedWorkflowConvergenceIteration,
) -> WorthQueryWorkflowConvergenceStepOutcome {
    started.advance()
}

fn cleanup_yielded_direct(
    yielded: WorthQueryYieldedDirectConvergenceIteration,
) -> WorthQueryDirectConvergenceYieldCleanupOutcome {
    yielded.cleanup()
}

fn cleanup_yielded_workflow(
    yielded: WorthQueryYieldedWorkflowConvergenceIteration,
) -> WorthQueryWorkflowConvergenceYieldCleanupOutcome {
    yielded.cleanup()
}

fn resolve_direct_yield_cleanup(
    outcome: WorthQueryDirectConvergenceYieldCleanupOutcome,
) -> WorthQueryDirectConvergenceYieldCleanupReceipt {
    match outcome {
        WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(receipt)
        | WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(receipt) => {
            let _ = receipt.identity();
            let _ = receipt.counters();
            let _ = receipt.incumbents();
            let _ = receipt.latest_report();
            receipt
        }
    }
}

fn resolve_workflow_yield_cleanup(
    outcome: WorthQueryWorkflowConvergenceYieldCleanupOutcome,
) -> WorthQueryWorkflowConvergenceYieldCleanupReceipt {
    match outcome {
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(receipt)
        | WorthQueryWorkflowConvergenceYieldCleanupOutcome::RecoveryRequired(receipt) => receipt,
        WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(pending) => {
            let _ = pending.identity();
            let _ = pending.counters();
            let _ = pending.incumbents();
            let _ = pending.latest_report();
            resolve_workflow_yield_cleanup(pending.retry())
        }
    }
}

fn readmit_yielded_direct(
    yielded: WorthQueryYieldedDirectConvergenceIteration,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryDirectConvergenceReadmissionOutcome {
    yielded.readmit_same_runtime(query, bridge)
}

fn readmit_yielded_workflow(
    yielded: WorthQueryYieldedWorkflowConvergenceIteration,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryWorkflowConvergenceReadmissionOutcome {
    yielded.readmit_same_runtime(query, bridge)
}

fn inspect_and_cleanup_direct_terminal(
    terminal: WorthQueryDirectConvergenceTerminal<WorthQueryConverged>,
) {
    let _ = terminal.identity();
    let _ = terminal.kind();
    let _ = terminal.counters();
    let _ = terminal.incumbents();
    let _ = terminal.latest_report();
    let _ = terminal.indeterminate_cause();
    let mut outcome = terminal.cleanup();
    loop {
        match outcome {
            Ok(receipt) => {
                let _ = receipt.counters();
                break;
            }
            Err(failure) => {
                let _ = failure.counters();
                outcome = failure.retry();
            }
        }
    }
}

fn inspect_and_cleanup_workflow_terminal(
    terminal: WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>,
) {
    let _ = terminal.identity();
    let _ = terminal.kind();
    let _ = terminal.counters();
    let _ = terminal.incumbents();
    let _ = terminal.latest_report();
    let _ = terminal.indeterminate_cause();
    let mut outcome = terminal.cleanup();
    loop {
        match outcome {
            WorthQueryWorkflowConvergenceCleanupOutcome::Complete(receipt) => {
                let _ = receipt.counters();
                break;
            }
            WorthQueryWorkflowConvergenceCleanupOutcome::Pending(pending) => {
                let _ = pending.counters();
                outcome = pending.retry();
            }
            WorthQueryWorkflowConvergenceCleanupOutcome::RecoveryRequired(failure) => {
                let _ = failure.counters();
                outcome = failure.retry();
            }
        }
    }
}

fn finish_direct_readmission_cleanup(
    cleanup: WorthQueryDirectConvergenceReadmissionCleanupRequired,
) {
    let mut outcome = cleanup.finish();
    loop {
        match outcome {
            WorthQueryDirectConvergenceReadmissionCleanupOutcome::Complete(receipt)
            | WorthQueryDirectConvergenceReadmissionCleanupOutcome::RecoveryRequired(receipt) => {
                let _ = receipt.readmission_evidence();
                break;
            }
            WorthQueryDirectConvergenceReadmissionCleanupOutcome::Pending(pending) => {
                outcome = pending.retry();
            }
        }
    }
}

fn finish_workflow_readmission_cleanup(
    cleanup: WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
) {
    let mut outcome = cleanup.finish();
    loop {
        match outcome {
            WorthQueryWorkflowConvergenceReadmissionCleanupOutcome::Complete(receipt)
            | WorthQueryWorkflowConvergenceReadmissionCleanupOutcome::RecoveryRequired(receipt) => {
                let _ = receipt.readmission_evidence();
                break;
            }
            WorthQueryWorkflowConvergenceReadmissionCleanupOutcome::Pending(pending) => {
                outcome = pending.retry();
            }
        }
    }
}

fn inspect_selection_and_occurrence(
    report: &WorthQueryBoundConvergenceReport,
    candidate: &WorthQueryRetainedConvergenceCandidateEvidence,
) {
    let _semantic_selection = report.decision().candidate_selection_key();
    let _owner_derived_occurrence = candidate.occurrence_identity();
    let _candidate_state = candidate.state_identity();
    let _admitted_report = candidate.report_evidence_identity();
}

fn resolve_direct_yield_recovery(recovery: WorthQueryDirectConvergenceYieldRecoveryRequired) {
    match recovery {
        WorthQueryDirectConvergenceYieldRecoveryRequired::RunningAttempt(running) => {
            resolve_direct_running_recovery(running)
        }
        WorthQueryDirectConvergenceYieldRecoveryRequired::TerminalCleanup(cleanup) => {
            resolve_direct_terminal_cleanup(cleanup)
        }
    }
}

fn resolve_direct_running_recovery(running: WorthQueryDirectConvergenceYieldRunningRecovery) {
    match running.resume() {
        Ok(paused) => drop(paused),
        Err(cleanup) => resolve_direct_terminal_cleanup(cleanup),
    }
}

fn resolve_direct_terminal_cleanup(
    cleanup: WorthQueryDirectConvergenceYieldTerminalCleanupRequired,
) {
    match cleanup.finish() {
        Ok(receipt) => {
            let _ = receipt.counters();
        }
        Err(running) => resolve_direct_running_recovery(running),
    }
}

fn resolve_workflow_yield_recovery(recovery: WorthQueryWorkflowConvergenceYieldRecoveryRequired) {
    match recovery {
        WorthQueryWorkflowConvergenceYieldRecoveryRequired::RunningAttempt(running) => {
            resolve_workflow_running_recovery(running)
        }
        WorthQueryWorkflowConvergenceYieldRecoveryRequired::TerminalCleanup(cleanup) => {
            resolve_workflow_terminal_cleanup(cleanup)
        }
    }
}

fn resolve_workflow_running_recovery(running: WorthQueryWorkflowConvergenceYieldRunningRecovery) {
    match running.resume() {
        Ok(paused) => drop(paused),
        Err(cleanup) => resolve_workflow_terminal_cleanup(cleanup),
    }
}

fn resolve_workflow_terminal_cleanup(
    cleanup: WorthQueryWorkflowConvergenceYieldTerminalCleanupRequired,
) {
    match cleanup.finish() {
        Ok(outcome) => resolve_workflow_recovery_cleanup(outcome),
        Err(running) => resolve_workflow_running_recovery(running),
    }
}

fn resolve_workflow_recovery_cleanup(
    outcome: WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome,
) {
    match outcome {
        WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome::Complete(receipt)
        | WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome::RecoveryRequired(receipt) => {
            let _ = receipt.counters();
        }
        WorthQueryWorkflowConvergenceYieldRecoveryCleanupOutcome::Pending(pending) => {
            retry_workflow_recovery_cleanup(pending)
        }
    }
}

fn retry_workflow_recovery_cleanup(
    pending: WorthQueryWorkflowConvergenceYieldRecoveryCleanupPending,
) {
    match pending.retry() {
        Ok(outcome) => resolve_workflow_recovery_cleanup(outcome),
        Err(running) => resolve_workflow_running_recovery(running),
    }
}

fn main() {}
