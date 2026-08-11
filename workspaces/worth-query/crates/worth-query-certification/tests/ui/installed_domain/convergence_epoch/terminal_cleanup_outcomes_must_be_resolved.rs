#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryWorkflowConvergenceCleanupFailure,
    WorthQueryWorkflowConvergenceCleanupOutcome, WorthQueryWorkflowConvergenceCleanupPending,
    WorthQueryWorkflowConvergenceTerminal,
};

type Terminal = WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>;
type Outcome = WorthQueryWorkflowConvergenceCleanupOutcome<WorthQueryConverged>;
type Pending = WorthQueryWorkflowConvergenceCleanupPending<WorthQueryConverged>;
type Failure = WorthQueryWorkflowConvergenceCleanupFailure<WorthQueryConverged>;

fn ignore_terminal(terminal: Terminal) {
    std::convert::identity(terminal);
}

fn ignore_outcome(outcome: Outcome) {
    std::convert::identity(outcome);
}

fn ignore_pending(pending: Pending) {
    std::convert::identity(pending);
}

fn ignore_failure(failure: Failure) {
    std::convert::identity(failure);
}

fn ignore_cleanup(terminal: Terminal) {
    terminal.cleanup();
}

fn ignore_pending_retry(pending: Pending) {
    pending.retry();
}

fn ignore_failure_retry(failure: Failure) {
    failure.retry();
}

fn main() {}
