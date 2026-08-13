use super::fixture::{
    direct_epoch_fixture, workflow_epoch_fixture, FixtureDisposition, WORKFLOW_STAGE,
};
use crate::domain_computation::{
    WorthQueryConverged, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceStepOutcome, WorthQueryDirectConvergenceTerminal,
    WorthQueryGraphProviderCallKind, WorthQueryIndeterminate, WorthQueryManagedGraphCallRequest,
    WorthQueryStableWithoutProof, WorthQueryWorkflowConvergenceIterationOutcome,
    WorthQueryWorkflowConvergenceStepOutcome, WorthQueryWorkflowConvergenceTerminal,
};

pub(super) fn direct_terminal_outcome(
    disposition: FixtureDisposition,
) -> WorthQueryDirectConvergenceIterationOutcome {
    let epoch = direct_epoch_fixture(disposition);
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        "terminal-matrix",
    )) {
        Ok(started) => started,
        Err(_) => panic!("terminal matrix iteration must start"),
    };
    match started.advance() {
        WorthQueryDirectConvergenceStepOutcome::Completed(outcome) => outcome,
        _ => panic!("terminal matrix provider must complete and rejoin"),
    }
}

pub(super) fn converged_terminal() -> WorthQueryDirectConvergenceTerminal<WorthQueryConverged> {
    match direct_terminal_outcome(FixtureDisposition::Converged) {
        WorthQueryDirectConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("converged fixture reached the wrong terminal"),
    }
}

pub(super) fn stable_without_proof_terminal(
) -> WorthQueryDirectConvergenceTerminal<WorthQueryStableWithoutProof> {
    match direct_terminal_outcome(FixtureDisposition::StableWithoutProof) {
        WorthQueryDirectConvergenceIterationOutcome::StableWithoutProof(terminal) => terminal,
        _ => panic!("stable-without-proof fixture reached the wrong terminal"),
    }
}

pub(super) fn indeterminate_terminal(
    disposition: FixtureDisposition,
) -> WorthQueryDirectConvergenceTerminal<WorthQueryIndeterminate> {
    match direct_terminal_outcome(disposition) {
        WorthQueryDirectConvergenceIterationOutcome::Indeterminate(terminal) => terminal,
        _ => panic!("indeterminate fixture reached the wrong terminal"),
    }
}

pub(super) fn workflow_converged_terminal(
) -> WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged> {
    match workflow_terminal_outcome(FixtureDisposition::Converged) {
        WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("ordinary workflow comparator must converge"),
    }
}

pub(super) fn workflow_indeterminate_terminal(
    disposition: FixtureDisposition,
) -> WorthQueryWorkflowConvergenceTerminal<WorthQueryIndeterminate> {
    match workflow_terminal_outcome(disposition) {
        WorthQueryWorkflowConvergenceIterationOutcome::Indeterminate(terminal) => terminal,
        _ => panic!("ordinary workflow comparator must remain indeterminate"),
    }
}

fn workflow_terminal_outcome(
    disposition: FixtureDisposition,
) -> WorthQueryWorkflowConvergenceIterationOutcome {
    let epoch = workflow_epoch_fixture(disposition);
    let started = match epoch.begin_stage_iteration(
        WORKFLOW_STAGE,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "ordinary-workflow-convergence-iteration",
        ),
    ) {
        Ok(started) => started,
        Err(_) => panic!("ordinary workflow iteration must start"),
    };
    match started.advance() {
        WorthQueryWorkflowConvergenceStepOutcome::Completed(outcome) => outcome,
        _ => panic!("ordinary workflow provider must complete and rejoin"),
    }
}
