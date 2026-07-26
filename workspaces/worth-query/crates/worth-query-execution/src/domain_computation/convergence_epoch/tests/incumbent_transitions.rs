use super::fixture::{
    direct_admission_fixture_with_contract, FixtureConvergenceContract, FixtureDisposition,
};
use crate::domain_computation::{
    WorthQueryConvergenceTerminalKind, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectGraphStepOutcome, WorthQueryGraphProviderCallKind,
    WorthQueryIteratingDirectConvergenceEpoch, WorthQueryManagedGraphCallRequest,
};

#[test]
fn pareto_remove_and_add_commits_only_after_the_installed_transition_validates() {
    let epoch = direct_admission_fixture_with_contract(
        FixtureDisposition::ParetoReplacement,
        FixtureConvergenceContract::Pareto,
    )
    .admit();
    let epoch = match advance(epoch, "pareto-add") {
        WorthQueryDirectConvergenceIterationOutcome::Continue(epoch) => epoch,
        _ => panic!("first Pareto candidate must continue with retained incumbent authority"),
    };
    let terminal = match advance(epoch, "pareto-replace") {
        WorthQueryDirectConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("valid Pareto replacement must preserve the converged terminal"),
    };

    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Converged
    );
    assert_eq!(terminal.counters().iteration_count(), 2);
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.incumbents()[0].occurrence_identity(),
        "candidate-2"
    );
    assert_eq!(
        terminal
            .latest_report()
            .expect("valid replacement must admit its report")
            .decision()
            .candidate_occurrence_identity(),
        "candidate-2"
    );
    if terminal.cleanup().is_err() {
        panic!("valid Pareto replacement must retain cleanup authority");
    }
}

#[test]
fn pareto_duplicate_candidate_denies_without_mutating_retained_incumbent_authority() {
    let epoch = direct_admission_fixture_with_contract(
        FixtureDisposition::ParetoCollision,
        FixtureConvergenceContract::Pareto,
    )
    .admit();
    let epoch = match advance(epoch, "pareto-first") {
        WorthQueryDirectConvergenceIterationOutcome::Continue(epoch) => epoch,
        _ => panic!("first Pareto candidate must be retained"),
    };
    let terminal = match advance(epoch, "pareto-duplicate") {
        WorthQueryDirectConvergenceIterationOutcome::Indeterminate(terminal) => terminal,
        _ => panic!("duplicate Pareto candidate must not become an admitted convergence report"),
    };

    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Indeterminate
    );
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.incumbents()[0].occurrence_identity(),
        "candidate-pareto"
    );
    assert_eq!(
        terminal
            .latest_report()
            .expect("the last valid report must remain retained")
            .iteration_ordinal(),
        1
    );
    assert!(terminal
        .domain_failure()
        .is_some_and(|detail| detail.contains("incumbent transition")));
    if terminal.cleanup().is_err() {
        panic!("denied Pareto replacement must retain cleanup authority");
    }
}

fn advance(
    epoch: WorthQueryIteratingDirectConvergenceEpoch,
    call_identity: &str,
) -> WorthQueryDirectConvergenceIterationOutcome {
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        call_identity,
    )) {
        Ok(started) => started,
        Err(_) => panic!("installed Pareto epoch must begin its next iteration"),
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("Pareto fixture graph provider must complete in one step"),
    };
    match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("exact Pareto completion must rejoin its pending epoch"),
    }
}
