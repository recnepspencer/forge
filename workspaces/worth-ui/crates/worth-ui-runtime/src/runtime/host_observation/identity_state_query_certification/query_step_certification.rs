use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason, WorthUiQueryLiveRebindOutcome,
};

pub(crate) fn certify_query_rebind_step(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    active_observation: &WorthUiActiveRuntimeObservation,
    counters: &mut WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    reject_query_plan_active_runtime_mismatch(step, active_observation, *counters)?;
    reject_empty_query_plan_digest(step, *counters)?;
    let typed_denial_match = classify_query_step_denial_expectation(step, counters);
    reject_missing_expected_query_denial(step, typed_denial_match, *counters)?;
    reject_undeclared_query_denial(step, *counters)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthUiQueryStepDenialExpectationMatch {
    saw_expected_denial: bool,
}

fn classify_query_step_denial_expectation(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    counters: &mut WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiQueryStepDenialExpectationMatch {
    let mut saw_expected_denial = false;
    for entry in step.rebind_plan().entries() {
        counters.record_query_binding(entry.outcome());
        if let WorthUiQueryLiveRebindOutcome::Deny(denial_reason) = entry.outcome() {
            saw_expected_denial |= step
                .expected_denial()
                .map(|expected| denial_reason.reason() == expected)
                .unwrap_or(false);
        }
    }
    WorthUiQueryStepDenialExpectationMatch {
        saw_expected_denial,
    }
}

fn reject_query_plan_active_runtime_mismatch(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    active_observation: &WorthUiActiveRuntimeObservation,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.rebind_plan().active_artifact_digest() == active_observation.artifact_digest() {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::QueryPlanActiveRuntimeMismatch {
            label: step.label().to_owned(),
            active_runtime_artifact_digest: active_observation.artifact_digest(),
            plan_active_artifact_digest: step.rebind_plan().active_artifact_digest(),
        },
        counters,
    ))
}

fn reject_empty_query_plan_digest(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.rebind_plan().active_artifact_digest() != 0
        && step.rebind_plan().candidate_artifact_digest() != 0
    {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::QueryPlanDigestMismatch {
            label: step.label().to_owned(),
            active_artifact_digest: step.rebind_plan().active_artifact_digest(),
            candidate_artifact_digest: step.rebind_plan().candidate_artifact_digest(),
        },
        counters,
    ))
}

fn reject_missing_expected_query_denial(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    typed_denial_match: WorthUiQueryStepDenialExpectationMatch,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.expected_denial().is_none() || typed_denial_match.saw_expected_denial {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::UnexpectedTypedQueryDriftDenial {
            label: step.label().to_owned(),
            expected: step.expected_denial().expect("checked expected denial"),
        },
        counters,
    ))
}

fn reject_undeclared_query_denial(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.expected_denial().is_some() || step.rebind_plan().counters().denied_binding_count() == 0
    {
        return Ok(());
    }
    Err(missing_typed_query_drift_denial(step, counters))
}

fn missing_typed_query_drift_denial(
    step: &crate::runtime::WorthUiQueryDriftCertificationScenarioStep,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiIdentityStateQueryCertificationDenial {
    denial(
        WorthUiIdentityStateQueryCertificationDenialReason::MissingTypedQueryDriftDenial {
            label: step.label().to_owned(),
        },
        counters,
    )
}

fn denial(
    reason: WorthUiIdentityStateQueryCertificationDenialReason,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiIdentityStateQueryCertificationDenial {
    WorthUiIdentityStateQueryCertificationDenial::new(reason, counters)
}
