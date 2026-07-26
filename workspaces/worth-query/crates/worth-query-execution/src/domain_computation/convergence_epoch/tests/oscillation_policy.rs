use super::fixture::{
    direct_admission_fixture_with_contract, FixtureConvergenceContract, FixtureDisposition,
};
use crate::domain_computation::{
    WorthQueryConvergenceTerminalKind, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceTerminal, WorthQueryDirectGraphStepOutcome,
    WorthQueryGraphProviderCallKind, WorthQueryIndeterminate, WorthQueryManagedGraphCallRequest,
    WorthQueryOscillating,
};

#[test]
fn installed_repeated_state_policies_reject_contradictory_domain_reports() {
    let impossible = indeterminate_terminal(
        FixtureDisposition::OscillatingSelected,
        FixtureConvergenceContract::OscillationImpossible,
    );
    assert_indeterminate_policy_denial(impossible);

    let continued_repetition = indeterminate_terminal(
        FixtureDisposition::RepeatedContinue,
        FixtureConvergenceContract::Bounded,
    );
    assert_indeterminate_policy_denial(continued_repetition);
}

#[test]
fn installed_oscillation_postures_govern_incumbent_selection() {
    let denied = oscillating_terminal(
        FixtureDisposition::Oscillating,
        FixtureConvergenceContract::Bounded,
    );
    assert_eq!(
        denied.kind(),
        WorthQueryConvergenceTerminalKind::Oscillating
    );
    assert!(denied.incumbents().is_empty());
    assert!(denied.latest_report().is_some());
    assert!(denied.domain_failure().is_none());
    if denied.cleanup().is_err() {
        panic!("detect-and-deny oscillation must retain cleanup authority");
    }

    let selected = oscillating_terminal(
        FixtureDisposition::OscillatingSelected,
        FixtureConvergenceContract::OscillationSelectIncumbent,
    );
    assert_eq!(
        selected.kind(),
        WorthQueryConvergenceTerminalKind::Oscillating
    );
    assert_eq!(selected.incumbents().len(), 1);
    assert!(selected.latest_report().is_some());
    assert!(selected.domain_failure().is_none());
    if selected.cleanup().is_err() {
        panic!("detect-and-select oscillation must retain cleanup authority");
    }

    let classified = oscillating_terminal(
        FixtureDisposition::DomainClassifiedOscillation,
        FixtureConvergenceContract::OscillationDomainClassified,
    );
    assert_eq!(
        classified.kind(),
        WorthQueryConvergenceTerminalKind::Oscillating
    );
    assert_eq!(classified.incumbents().len(), 1);
    assert!(classified.latest_report().is_some());
    assert!(classified.domain_failure().is_none());
    if classified.cleanup().is_err() {
        panic!("domain-classified oscillation must retain cleanup authority");
    }
}

fn assert_indeterminate_policy_denial(
    terminal: WorthQueryDirectConvergenceTerminal<WorthQueryIndeterminate>,
) {
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    assert!(terminal
        .domain_failure()
        .is_some_and(|detail| detail.contains("oscillation posture")));
    assert!(terminal.latest_report().is_none());
    assert!(terminal.incumbents().is_empty());
    if terminal.cleanup().is_err() {
        panic!("rejected oscillation report must retain cleanup authority");
    }
}

fn indeterminate_terminal(
    disposition: FixtureDisposition,
    contract: FixtureConvergenceContract,
) -> WorthQueryDirectConvergenceTerminal<WorthQueryIndeterminate> {
    match terminal_outcome(disposition, contract) {
        WorthQueryDirectConvergenceIterationOutcome::Indeterminate(terminal) => terminal,
        _ => panic!("oscillation policy fixture must be indeterminate"),
    }
}

fn oscillating_terminal(
    disposition: FixtureDisposition,
    contract: FixtureConvergenceContract,
) -> WorthQueryDirectConvergenceTerminal<WorthQueryOscillating> {
    match terminal_outcome(disposition, contract) {
        WorthQueryDirectConvergenceIterationOutcome::Oscillating(terminal) => terminal,
        _ => panic!("oscillation policy fixture must be oscillating"),
    }
}

fn terminal_outcome(
    disposition: FixtureDisposition,
    contract: FixtureConvergenceContract,
) -> WorthQueryDirectConvergenceIterationOutcome {
    let epoch = direct_admission_fixture_with_contract(disposition, contract).admit();
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        "oscillation-policy",
    )) {
        Ok(started) => started,
        Err(_) => panic!("oscillation policy fixture iteration must start"),
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("oscillation policy fixture provider must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("oscillation policy completion must rejoin its epoch"),
    };
    outcome
}
