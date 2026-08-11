use crate::runtime::{
    admit_authoritative_intent_execution, WorthQueryIntentAdmissionDenial,
    WorthQueryIntentDeclaration, WorthQueryIntentExecution,
};

use super::{
    WorthQueryAuthoritativeIntentExecutionHandoff, WorthQueryEffectTriggeredIntentExecutionHandoff,
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentViolationDecision,
};

pub(crate) fn admit_authoritative_execution(
    handoff: &WorthQueryAuthoritativeIntentExecutionHandoff,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_execution_with_proof(
        handoff.family(),
        handoff.entrypoint(),
        handoff.request_digest(),
        handoff.eligibility_digest(),
        handoff.declaration(),
        execution,
    )
}

pub(crate) fn admit_effect_execution(
    handoff: &WorthQueryEffectTriggeredIntentExecutionHandoff,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_execution_with_proof(
        handoff.family(),
        handoff.entrypoint(),
        handoff.request_digest(),
        handoff.eligibility_digest(),
        handoff.declaration(),
        execution,
    )
}

fn admit_execution_with_proof(
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    request_digest: &str,
    eligibility_digest: &str,
    declaration: &WorthQueryIntentDeclaration,
    execution: &WorthQueryIntentExecution,
) -> Result<(), WorthQueryIntentViolationDecision> {
    admit_authoritative_intent_execution(declaration, execution).map_err(
        |denial: WorthQueryIntentAdmissionDenial| {
            WorthQueryIntentViolationDecision::new(
                family,
                entrypoint,
                denial.stage(),
                denial.message(),
                request_digest,
                eligibility_digest,
            )
        },
    )
}
