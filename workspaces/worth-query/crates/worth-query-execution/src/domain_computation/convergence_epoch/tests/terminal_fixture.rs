use super::fixture::{
    direct_epoch_fixture, workflow_epoch_fixture, FixtureDisposition, WORKFLOW_STAGE,
};
use crate::domain_computation::{
    WorthQueryConverged, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceTerminal, WorthQueryDirectGraphStepOutcome,
    WorthQueryGraphProviderCallKind, WorthQueryIndeterminate, WorthQueryManagedGraphCallRequest,
    WorthQueryStableWithoutProof, WorthQueryWorkflowConvergenceIterationOutcome,
    WorthQueryWorkflowConvergenceTerminal, WorthQueryWorkflowGraphStepOutcome,
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
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("terminal matrix provider must complete"),
    };
    match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("terminal matrix completion must rejoin"),
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
    let epoch = workflow_epoch_fixture(FixtureDisposition::Converged);
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
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("ordinary workflow provider must complete"),
    };
    match pending.admit_completion(completion) {
        Ok(WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal)) => terminal,
        Ok(_) => panic!("ordinary workflow comparator must converge"),
        Err(_) => panic!("ordinary workflow completion must rejoin"),
    }
}
