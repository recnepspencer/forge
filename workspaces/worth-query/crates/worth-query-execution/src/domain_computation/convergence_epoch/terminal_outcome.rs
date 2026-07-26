use std::sync::Arc;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryCancelled, WorthQueryConverged, WorthQueryConvergenceDisposition,
    WorthQueryConvergenceTerminalKind, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceTerminal, WorthQueryExhausted, WorthQueryFeasibleIncumbent,
    WorthQueryIndeterminate, WorthQueryOscillating, WorthQueryStableWithoutProof,
    WorthQueryWorkflowConvergenceIterationOutcome, WorthQueryWorkflowConvergenceTerminal,
};
use crate::domain_computation::{WorthQueryDirectRunTerminal, WorthQueryWorkflowRunTerminal};

pub(super) const fn semantic_terminal_kind(
    disposition: WorthQueryConvergenceDisposition,
) -> WorthQueryConvergenceTerminalKind {
    match disposition {
        WorthQueryConvergenceDisposition::Converged => WorthQueryConvergenceTerminalKind::Converged,
        WorthQueryConvergenceDisposition::StableWithoutProof => {
            WorthQueryConvergenceTerminalKind::StableWithoutProof
        }
        WorthQueryConvergenceDisposition::FeasibleIncumbent => {
            WorthQueryConvergenceTerminalKind::FeasibleIncumbent
        }
        WorthQueryConvergenceDisposition::Oscillating => {
            WorthQueryConvergenceTerminalKind::Oscillating
        }
        WorthQueryConvergenceDisposition::Indeterminate
        | WorthQueryConvergenceDisposition::Continue => {
            WorthQueryConvergenceTerminalKind::Indeterminate
        }
    }
}

pub(super) fn direct_terminal_outcome(
    core: WorthQueryConvergenceEpochCore,
    managed: WorthQueryDirectRunTerminal,
    kind: WorthQueryConvergenceTerminalKind,
    domain_failure: Option<Arc<str>>,
) -> WorthQueryDirectConvergenceIterationOutcome {
    match kind {
        WorthQueryConvergenceTerminalKind::Converged => {
            WorthQueryDirectConvergenceIterationOutcome::Converged(
                WorthQueryDirectConvergenceTerminal::<WorthQueryConverged>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::StableWithoutProof => {
            WorthQueryDirectConvergenceIterationOutcome::StableWithoutProof(
                WorthQueryDirectConvergenceTerminal::<WorthQueryStableWithoutProof>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::FeasibleIncumbent => {
            WorthQueryDirectConvergenceIterationOutcome::FeasibleIncumbent(
                WorthQueryDirectConvergenceTerminal::<WorthQueryFeasibleIncumbent>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Oscillating => {
            WorthQueryDirectConvergenceIterationOutcome::Oscillating(
                WorthQueryDirectConvergenceTerminal::<WorthQueryOscillating>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Exhausted => {
            WorthQueryDirectConvergenceIterationOutcome::Exhausted(
                WorthQueryDirectConvergenceTerminal::<WorthQueryExhausted>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Cancelled => {
            WorthQueryDirectConvergenceIterationOutcome::Cancelled(
                WorthQueryDirectConvergenceTerminal::<WorthQueryCancelled>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Indeterminate => {
            WorthQueryDirectConvergenceIterationOutcome::Indeterminate(
                WorthQueryDirectConvergenceTerminal::<WorthQueryIndeterminate>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
    }
}

pub(super) fn workflow_terminal_outcome(
    core: WorthQueryConvergenceEpochCore,
    managed: WorthQueryWorkflowRunTerminal,
    kind: WorthQueryConvergenceTerminalKind,
    domain_failure: Option<Arc<str>>,
) -> WorthQueryWorkflowConvergenceIterationOutcome {
    match kind {
        WorthQueryConvergenceTerminalKind::Converged => {
            WorthQueryWorkflowConvergenceIterationOutcome::Converged(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryConverged>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::StableWithoutProof => {
            WorthQueryWorkflowConvergenceIterationOutcome::StableWithoutProof(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryStableWithoutProof>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::FeasibleIncumbent => {
            WorthQueryWorkflowConvergenceIterationOutcome::FeasibleIncumbent(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryFeasibleIncumbent>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Oscillating => {
            WorthQueryWorkflowConvergenceIterationOutcome::Oscillating(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryOscillating>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Exhausted => {
            WorthQueryWorkflowConvergenceIterationOutcome::Exhausted(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryExhausted>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Cancelled => {
            WorthQueryWorkflowConvergenceIterationOutcome::Cancelled(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryCancelled>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
        WorthQueryConvergenceTerminalKind::Indeterminate => {
            WorthQueryWorkflowConvergenceIterationOutcome::Indeterminate(
                WorthQueryWorkflowConvergenceTerminal::<WorthQueryIndeterminate>::new(
                    core,
                    managed,
                    domain_failure,
                ),
            )
        }
    }
}
