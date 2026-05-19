use super::authoritative_declaration;
use crate::facade::runtime::{
    admit_runtime_intent_request, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentViolationDecision,
    ForgeQueryRawIntentAdmissionRequest,
};
use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedDeferredIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) decision: ForgeQueryIntentAdmissionDecision,
    pub(in crate::intent_admission::certification) trace: ForgeQueryIntentDecisionTraceEnvelope,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedUnsupportedIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) decision: ForgeQueryIntentAdmissionDecision,
    pub(in crate::intent_admission::certification) trace: ForgeQueryIntentDecisionTraceEnvelope,
}

pub(in crate::intent_admission::certification) fn certified_deferred_intent_fixture(
) -> CertifiedDeferredIntentFixture {
    let declaration = authoritative_declaration("certification-deferred-intent");
    let request = ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        declaration,
    )
    .expect("deferred certification request should build");
    let decision = admit_runtime_intent_request(request.clone());
    let trace = review_trace(&request, &decision, "deferred");
    CertifiedDeferredIntentFixture {
        request,
        decision,
        trace,
    }
}

pub(in crate::intent_admission::certification) fn certified_unsupported_intent_fixture(
) -> CertifiedUnsupportedIntentFixture {
    let declaration = authoritative_declaration("certification-unsupported-intent");
    let request = ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        declaration,
    )
    .expect("unsupported certification request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let violation = ForgeQueryIntentViolationDecision::from_eligibility_denial(
        &eligibility,
        "support-unsupported",
        "neighbor-unsupported-until-coverage",
    );
    let decision = ForgeQueryIntentAdmissionDecision::Violation(violation);
    let trace = review_trace(&request, &decision, "unsupported");
    CertifiedUnsupportedIntentFixture {
        request,
        decision,
        trace,
    }
}

fn review_trace(
    request: &ForgeQueryRawIntentAdmissionRequest,
    decision: &ForgeQueryIntentAdmissionDecision,
    lane: &str,
) -> ForgeQueryIntentDecisionTraceEnvelope {
    ForgeQueryRuntimeIntentAdmissionReviewData::from_decision(request.clone(), decision.clone())
        .decision_trace_envelope()
        .unwrap_or_else(|| panic!("{lane} certification review should preserve a trace"))
        .clone()
}
