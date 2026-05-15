#[path = "dx_authoritative.rs"]
mod dx_authoritative;
#[path = "dx_basis_projection.rs"]
mod dx_basis_projection;
#[path = "dx_effect.rs"]
mod dx_effect;

use super::{
    admit_runtime_intent_request, ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentViolationDecision,
    ForgeQueryRawIntentAdmissionRequest,
};

pub use dx_authoritative::{
    ForgeQueryAdmittedRuntimeIntent, ForgeQueryRuntimeIntentAdmissionReview,
    ForgeQueryRuntimeIntentAuthoring,
};
pub use dx_basis_projection::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryBasisObservationAdmittedIntent, ForgeQueryBasisObservationIntentAuthoring,
    ForgeQueryBasisObservationIntentReview, ForgeQueryProjectionConsumptionAdmittedIntent,
    ForgeQueryProjectionConsumptionIntentAuthoring, ForgeQueryProjectionConsumptionIntentReview,
};
pub use dx_effect::{
    ForgeQueryAdmittedRuntimeEffectWriteIntent, ForgeQueryRuntimeEffectWriteIntentAdmissionReview,
    ForgeQueryRuntimeEffectWriteIntentAuthoring,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryRuntimeIntentAdmissionReviewData {
    request: ForgeQueryRawIntentAdmissionRequest,
    eligibility: ForgeQueryIntentAdmissionEligibility,
    decision: ForgeQueryIntentAdmissionDecision,
    non_admitted_trace: Option<ForgeQueryIntentDecisionTraceEnvelope>,
}

impl ForgeQueryRuntimeIntentAdmissionReviewData {
    pub(crate) fn from_request(request: ForgeQueryRawIntentAdmissionRequest) -> Self {
        let decision = admit_runtime_intent_request(request.clone());
        Self::from_decision(request, decision)
    }

    pub(crate) fn from_decision(
        request: ForgeQueryRawIntentAdmissionRequest,
        decision: ForgeQueryIntentAdmissionDecision,
    ) -> Self {
        let eligibility = ForgeQueryIntentAdmissionEligibility::from_request(request.clone());
        let non_admitted_trace = match &decision {
            ForgeQueryIntentAdmissionDecision::Admitted(_) => None,
            ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
                Some(ForgeQueryIntentDecisionTraceEnvelope::for_request_advisory(
                    &request,
                    &eligibility.trace_evidence(),
                    advisory,
                ))
            }
            ForgeQueryIntentAdmissionDecision::Violation(violation) => Some(
                ForgeQueryIntentDecisionTraceEnvelope::for_request_violation(
                    &request,
                    &eligibility.trace_evidence(),
                    violation,
                ),
            ),
        };
        Self {
            request,
            eligibility,
            decision,
            non_admitted_trace,
        }
    }

    pub(crate) fn admitted_plan(&self) -> Option<&ForgeQueryAdmittedIntentPlan> {
        match &self.decision {
            ForgeQueryIntentAdmissionDecision::Admitted(plan) => Some(plan),
            ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => None,
        }
    }

    pub(crate) fn non_admitted_stop(&self) -> Option<ForgeQueryIntentNonAdmittedStop> {
        self.decision.clone().into_non_admitted_stop()
    }

    pub(crate) fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        &self.request
    }

    pub(crate) fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        &self.decision
    }

    pub(crate) fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        &self.eligibility
    }

    pub(crate) fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.non_admitted_trace.as_ref()
    }
}

pub(crate) fn non_admitted_runtime_violation(
    review: &ForgeQueryRuntimeIntentAdmissionReviewData,
) -> ForgeQueryIntentViolationDecision {
    review
        .non_admitted_stop()
        .expect("non-admitted runtime review should preserve a stop artifact")
        .into_violation_stop()
        .violation()
        .clone()
}
