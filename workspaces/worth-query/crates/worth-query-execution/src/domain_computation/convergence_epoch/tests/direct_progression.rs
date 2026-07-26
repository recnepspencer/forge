use super::fixture::{direct_epoch_fixture, FixtureDisposition};
use crate::domain_computation::{
    WorthQueryConvergenceTerminalKind, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectGraphStepOutcome, WorthQueryGraphProviderCallKind,
    WorthQueryManagedGraphCallRequest, WorthQueryManagedRunTerminalKind,
};

#[test]
fn real_installed_direct_authorities_progress_to_converged_and_cleanup() {
    let epoch = direct_epoch_fixture(FixtureDisposition::Converged);
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        "convergence-iteration",
    )) {
        Ok(started) => started,
        Err(_) => panic!("real installed graph authority must start the convergence iteration"),
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("single-step fixture provider must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("exact managed completion must rejoin the pending epoch"),
    };
    let terminal = match outcome {
        WorthQueryDirectConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("installed provider convergence decision must remain distinct"),
    };

    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Converged
    );
    assert_eq!(terminal.counters().iteration_count(), 1);
    assert_eq!(terminal.counters().provider_work_unit_count(), 1);
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.incumbents()[0].occurrence_identity(),
        "candidate-1"
    );
    assert_eq!(
        terminal.incumbents()[0]
            .domain_evidence()
            .output_occurrence_identity(),
        "candidate-1"
    );
    assert!(terminal.latest_report().is_some());
    let cleanup = match terminal.cleanup() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("converged direct epoch must retain cleanup authority"),
    };
    assert_eq!(cleanup.counters().cleanup_count(), 1);
}

#[test]
fn epoch_owned_iteration_bound_terminalizes_continue_as_exhausted() {
    let mut epoch = direct_epoch_fixture(FixtureDisposition::Continue);
    for ordinal in 1..=3 {
        let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            format!("convergence-iteration-{ordinal}"),
        )) {
            Ok(started) => started,
            Err(_) => panic!("bounded iteration must start"),
        };
        let (pending, active) = started.into_parts();
        let completion = match active.advance() {
            WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
            _ => panic!("single-step fixture provider must complete"),
        };
        let outcome = match pending.admit_completion(completion) {
            Ok(outcome) => outcome,
            Err(_) => panic!("exact managed completion must rejoin the pending epoch"),
        };
        match outcome {
            WorthQueryDirectConvergenceIterationOutcome::Continue(next) if ordinal < 3 => {
                epoch = next;
            }
            WorthQueryDirectConvergenceIterationOutcome::Exhausted(terminal) if ordinal == 3 => {
                assert_eq!(
                    terminal.kind(),
                    WorthQueryConvergenceTerminalKind::Exhausted
                );
                assert_eq!(terminal.counters().iteration_count(), 3);
                assert_eq!(terminal.counters().provider_work_unit_count(), 3);
                assert_eq!(terminal.incumbents().len(), 1);
                assert_eq!(
                    terminal.incumbents()[0].occurrence_identity(),
                    "candidate-3"
                );
                assert_eq!(
                    terminal.managed_terminal().kind(),
                    WorthQueryManagedRunTerminalKind::Exhausted
                );
                assert_eq!(
                    terminal
                        .managed_terminal()
                        .provider_work()
                        .admitted_receipt_count(),
                    3
                );
                assert_eq!(
                    terminal
                        .managed_terminal()
                        .provider_work()
                        .retained_artifact_count(),
                    0
                );
                assert!(!terminal
                    .managed_terminal()
                    .provider_work()
                    .checkpoint_available());
                if terminal.cleanup().is_err() {
                    panic!("exhausted direct epoch must retain cleanup authority");
                }
                return;
            }
            _ => panic!("iteration bound produced the wrong typed progression"),
        }
    }
    panic!("iteration bound did not terminalize the epoch")
}
