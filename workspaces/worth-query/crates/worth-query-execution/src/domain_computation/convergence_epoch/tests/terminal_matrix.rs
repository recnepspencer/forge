use super::fixture::{direct_epoch_fixture, FixtureDisposition};
use super::terminal_fixture::{direct_terminal_outcome, indeterminate_terminal};
use crate::domain_computation::{
    WorthQueryConvergenceFeasibility, WorthQueryConvergenceProgress,
    WorthQueryConvergenceRepeatedState, WorthQueryConvergenceTerminalKind,
    WorthQueryConvergenceTerminalState, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceTerminal, WorthQueryDirectGraphStepOutcome,
    WorthQueryGraphProviderCallKind, WorthQueryManagedGraphCallRequest,
    WorthQueryManagedProviderSessionDisposition, WorthQueryManagedRunCleanupDisposition,
    WorthQueryManagedRunTerminalKind,
};
use worth_runtime_bridge::facade::BridgeManagedExecutionCancellationReason;

#[test]
fn installed_domain_semantics_preserve_distinct_terminal_kinds() {
    let cases = [
        (
            FixtureDisposition::Converged,
            WorthQueryConvergenceTerminalKind::Converged,
            1,
        ),
        (
            FixtureDisposition::StableWithoutProof,
            WorthQueryConvergenceTerminalKind::StableWithoutProof,
            1,
        ),
        (
            FixtureDisposition::FeasibleIncumbent,
            WorthQueryConvergenceTerminalKind::FeasibleIncumbent,
            1,
        ),
        (
            FixtureDisposition::Oscillating,
            WorthQueryConvergenceTerminalKind::Oscillating,
            0,
        ),
    ];
    for (fixture, expected, incumbent_count) in cases {
        match direct_terminal_outcome(fixture) {
            WorthQueryDirectConvergenceIterationOutcome::Converged(terminal) => {
                assert_semantic_terminal(terminal, expected, incumbent_count)
            }
            WorthQueryDirectConvergenceIterationOutcome::StableWithoutProof(terminal) => {
                assert_semantic_terminal(terminal, expected, incumbent_count)
            }
            WorthQueryDirectConvergenceIterationOutcome::FeasibleIncumbent(terminal) => {
                assert_semantic_terminal(terminal, expected, incumbent_count)
            }
            WorthQueryDirectConvergenceIterationOutcome::Oscillating(terminal) => {
                assert_semantic_terminal(terminal, expected, incumbent_count)
            }
            _ => panic!("semantic fixture reached the wrong terminal"),
        }
    }
}

#[test]
fn incoherent_terminal_semantics_become_indeterminate_without_becoming_a_report() {
    let terminal = indeterminate_terminal(FixtureDisposition::IncoherentStable);
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    assert!(terminal
        .domain_failure()
        .is_some_and(|detail| detail.contains("contradicts")));
    assert!(terminal.latest_report().is_none());
    assert_eq!(terminal.incumbents().len(), 0);
    assert_eq!(terminal.counters().provider_work_unit_count(), 1);
    assert_eq!(terminal.counters().comparator_call_count(), 1);
    assert_eq!(terminal.counters().progress_check_count(), 1);
    assert_eq!(terminal.counters().repeated_state_probe_count(), 1);
    assert_eq!(
        terminal.managed_terminal().kind(),
        WorthQueryManagedRunTerminalKind::Failed
    );
    if terminal.cleanup().is_err() {
        panic!("rejected domain report must retain cleanup authority");
    }
}

#[test]
fn stalled_progress_remains_explicit_domain_evidence() {
    let terminal = indeterminate_terminal(FixtureDisposition::Stalled);
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    let report = terminal
        .latest_report()
        .expect("stalled progress must retain its admitted domain report");
    assert_eq!(
        report.decision().progress(),
        WorthQueryConvergenceProgress::Stalled
    );
    assert_eq!(
        report.decision().feasibility(),
        WorthQueryConvergenceFeasibility::Feasible
    );
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.managed_terminal().kind(),
        WorthQueryManagedRunTerminalKind::Completed
    );
    assert!(terminal.domain_failure().is_none());
    if terminal.cleanup().is_err() {
        panic!("stalled convergence terminal must retain cleanup authority");
    }
}

#[test]
fn indeterminate_comparison_retains_each_indeterminate_semantic_axis() {
    let terminal = indeterminate_terminal(FixtureDisposition::IndeterminateComparison);
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    let decision = terminal
        .latest_report()
        .expect("indeterminate comparison must retain its domain report")
        .decision();
    assert_eq!(
        decision.feasibility(),
        WorthQueryConvergenceFeasibility::Indeterminate
    );
    assert_eq!(
        decision.progress(),
        WorthQueryConvergenceProgress::Indeterminate
    );
    assert_eq!(
        decision.repeated_state(),
        WorthQueryConvergenceRepeatedState::Indeterminate
    );
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.managed_terminal().kind(),
        WorthQueryManagedRunTerminalKind::Completed
    );
    assert!(terminal.domain_failure().is_none());
    if terminal.cleanup().is_err() {
        panic!("indeterminate comparison must retain cleanup authority");
    }
}

#[test]
fn comparator_failure_retains_exact_attempted_work_without_admitting_a_report() {
    let terminal = indeterminate_terminal(FixtureDisposition::ComparatorFailure);
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    assert!(terminal
        .domain_failure()
        .is_some_and(|detail| detail.contains("comparator")));
    assert!(terminal.latest_report().is_none());
    assert_eq!(terminal.counters().provider_work_unit_count(), 1);
    assert_eq!(terminal.counters().comparator_call_count(), 1);
    assert_eq!(terminal.counters().progress_check_count(), 0);
    assert_eq!(terminal.counters().repeated_state_probe_count(), 0);
    assert_eq!(terminal.incumbents().len(), 0);
    assert_eq!(
        terminal.managed_terminal().kind(),
        WorthQueryManagedRunTerminalKind::Failed
    );
    if terminal.cleanup().is_err() {
        panic!("comparator failure must retain cleanup authority");
    }
}

#[test]
fn managed_signal_cancellation_remains_a_distinct_convergence_terminal() {
    let epoch = direct_epoch_fixture(FixtureDisposition::Converged);
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        "cancelled-terminal",
    )) {
        Ok(started) => started,
        Err(_) => panic!("cancelled terminal iteration must start"),
    };
    let (pending, active) = started.into_parts();
    active
        .request_cancellation(BridgeManagedExecutionCancellationReason::HostRequested)
        .expect("Signal cancellation request must admit");
    let managed = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Cancelled(terminal) => terminal,
        _ => panic!("Signal cancellation must terminalize before provider work"),
    };
    let outcome = match pending.admit_managed_terminal(managed) {
        Ok(outcome) => outcome,
        Err(_) => panic!("exact cancelled run must rejoin its pending epoch"),
    };
    let terminal = match outcome {
        WorthQueryDirectConvergenceIterationOutcome::Cancelled(terminal) => terminal,
        _ => panic!("cancelled managed run must remain a cancelled convergence terminal"),
    };
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Cancelled
    );
    assert_eq!(terminal.counters().provider_work_unit_count(), 0);
    assert_eq!(terminal.incumbents().len(), 0);
    assert!(terminal.latest_report().is_none());
    let managed = terminal.managed_terminal();
    assert_eq!(managed.kind(), WorthQueryManagedRunTerminalKind::Cancelled);
    assert_eq!(
        managed.provider_work().session_disposition(),
        WorthQueryManagedProviderSessionDisposition::Interrupted
    );
    assert_eq!(managed.provider_work().issued_call_count(), 1);
    assert_eq!(managed.provider_work().interrupted_call_count(), 1);
    assert_eq!(managed.provider_work().provider_step_attempt_count(), 0);
    assert!(!managed.provider_work().checkpoint_available());
    let cleanup = match terminal.cleanup() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("cancelled convergence terminal must retain cleanup authority"),
    };
    assert_eq!(
        cleanup.managed_receipt().disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}

fn assert_semantic_terminal<State>(
    terminal: WorthQueryDirectConvergenceTerminal<State>,
    expected: WorthQueryConvergenceTerminalKind,
    incumbent_count: usize,
) where
    State: WorthQueryConvergenceTerminalState,
{
    assert_eq!(terminal.kind(), expected);
    assert_eq!(terminal.counters().iteration_count(), 1);
    assert_eq!(terminal.counters().comparator_call_count(), 1);
    assert_eq!(terminal.counters().progress_check_count(), 1);
    assert_eq!(terminal.counters().repeated_state_probe_count(), 1);
    assert!(terminal.domain_failure().is_none());
    assert_eq!(terminal.incumbents().len(), incumbent_count);
    let report = terminal
        .latest_report()
        .expect("semantic terminal must retain its admitted report");
    assert_eq!(report.domain_work().comparator_call_count(), 1);
    assert_eq!(report.domain_work().progress_check_count(), 1);
    assert_eq!(report.domain_work().repeated_state_probe_count(), 1);
    let managed = terminal.managed_terminal();
    assert_eq!(managed.kind(), WorthQueryManagedRunTerminalKind::Completed);
    assert_eq!(
        managed.provider_work().session_disposition(),
        WorthQueryManagedProviderSessionDisposition::ReceiptsAdmitted
    );
    assert_eq!(managed.provider_work().retained_artifact_count(), 0);
    assert_eq!(managed.provider_work().retained_bytes(), 0);
    assert!(!managed.provider_work().checkpoint_available());
    let cleanup = match terminal.cleanup() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("semantic terminal must retain cleanup authority"),
    };
    assert_eq!(
        cleanup.managed_receipt().disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}
