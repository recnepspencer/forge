#![deny(unused_must_use)]

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowRunCleanupFailure, WorthQueryWorkflowRunCleanupOutcome,
    WorthQueryWorkflowRunCleanupPending, WorthQueryWorkflowRunTerminal,
};

fn ignore_terminal(terminal: WorthQueryWorkflowRunTerminal) {
    std::convert::identity(terminal);
}

fn ignore_outcome(outcome: WorthQueryWorkflowRunCleanupOutcome) {
    std::convert::identity(outcome);
}

fn ignore_pending(pending: WorthQueryWorkflowRunCleanupPending) {
    std::convert::identity(pending);
}

fn ignore_failure(failure: WorthQueryWorkflowRunCleanupFailure) {
    std::convert::identity(failure);
}

fn ignore_cleanup(terminal: WorthQueryWorkflowRunTerminal) {
    terminal.cleanup();
}

fn ignore_pending_retry(pending: WorthQueryWorkflowRunCleanupPending) {
    pending.retry();
}

fn ignore_failure_retry(failure: WorthQueryWorkflowRunCleanupFailure) {
    failure.retry();
}

fn main() {}
