use std::sync::Arc;

use super::invariant_execution::{execute_invariant, locator, requirement, state_with_outcome};
use super::provisional_attempt_fixture::InvariantFixtureOutcome;
use crate::domain_computation::{
    WorthQueryInvariantExecutionDenialKind, WorthQueryInvariantExecutionFailurePosture,
};
use worth_query_installation::facade::WorthQueryInvariantEnforcement;

#[test]
fn provider_cannot_replay_a_retained_verdict_admission_into_another_attempt() {
    let hostile = state_with_outcome(InvariantFixtureOutcome::RetainThenSubstitute);
    let first = execute_invariant(
        Arc::clone(&hostile),
        vec![requirement(
            "closed-loop",
            WorthQueryInvariantEnforcement::Blocking,
            4,
        )],
        "closed-loop",
        [locator("base")],
    )
    .err()
    .expect("hostile provider must retain the first admission");
    assert_eq!(
        first.kind(),
        WorthQueryInvariantExecutionDenialKind::ProviderRejected
    );

    let substituted = execute_invariant(
        hostile,
        vec![requirement(
            "closed-loop",
            WorthQueryInvariantEnforcement::Blocking,
            4,
        )],
        "closed-loop",
        [locator("base")],
    )
    .err()
    .expect("retained admission must not authorize another attempt");
    assert_eq!(
        substituted.kind(),
        WorthQueryInvariantExecutionDenialKind::EvidenceSubstitution
    );
}

#[test]
fn provider_panics_during_load_and_execution_are_typed() {
    for outcome in [
        InvariantFixtureOutcome::PanicDuringLoad,
        InvariantFixtureOutcome::PanicDuringExecution,
    ] {
        let failure = execute_invariant(
            state_with_outcome(outcome),
            vec![requirement(
                "closed-loop",
                WorthQueryInvariantEnforcement::Blocking,
                4,
            )],
            "closed-loop",
            [locator("base")],
        )
        .err()
        .expect("provider panic must become a typed invariant failure");
        assert_eq!(
            failure.kind(),
            WorthQueryInvariantExecutionDenialKind::ProviderPanicked
        );
        assert_eq!(
            failure.posture(),
            WorthQueryInvariantExecutionFailurePosture::Denied
        );
    }
}
