use super::*;
use crate::intent_admission::{
    ForgeQueryAuthoritativeIntentExecutionHandoff, ForgeQueryEffectTriggeredIntentExecutionHandoff,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentViolationDecision,
};

impl ForgeQueryRuntime {
    pub(crate) fn resolve_reviewed_admitted_authoritative_intent_handoff(
        &self,
        review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryAuthoritativeIntentExecutionHandoff, ForgeQueryRuntimeError> {
        let declaration = review.request().declaration().clone();
        let non_admitted_trace = review.decision_trace_envelope().cloned();
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::Authoritative(plan),
            ) => Ok(
                crate::intent_admission::ForgeQueryAuthoritativeIntentExecutionHandoff::from_plan(
                    plan,
                ),
            ),
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::EffectTriggered(_),
            ) => Err(self.intent_violation_error(
                &declaration,
                phase_three_family_violation(
                    &review,
                    "authoritative-runtime-path-requires-authoritative-handoff-proof",
                ),
                None,
                non_admitted_trace,
                None,
            )),
            ForgeQueryIntentAdmissionDecision::Advisory(advisory) => Err(self
                .intent_violation_error(
                    &declaration,
                    advisory.into_violation(),
                    None,
                    non_admitted_trace,
                    None,
                )),
            ForgeQueryIntentAdmissionDecision::Violation(violation) => Err(self
                .intent_violation_error(&declaration, violation, None, non_admitted_trace, None)),
        }
    }

    pub(crate) fn resolve_reviewed_admitted_effect_intent_handoff(
        &self,
        review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryEffectTriggeredIntentExecutionHandoff, ForgeQueryRuntimeError> {
        let declaration = review.request().declaration().clone();
        let non_admitted_trace = review.decision_trace_envelope().cloned();
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::EffectTriggered(plan),
            ) => Ok(
                crate::intent_admission::ForgeQueryEffectTriggeredIntentExecutionHandoff::from_plan(
                    plan,
                ),
            ),
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::Authoritative(_),
            ) => Err(self.intent_violation_error(
                &declaration,
                phase_three_family_violation(
                    &review,
                    "effect-runtime-path-requires-effect-handoff-proof",
                ),
                None,
                non_admitted_trace,
                None,
            )),
            ForgeQueryIntentAdmissionDecision::Advisory(advisory) => Err(self
                .intent_violation_error(
                    &declaration,
                    advisory.into_violation(),
                    None,
                    non_admitted_trace,
                    None,
                )),
            ForgeQueryIntentAdmissionDecision::Violation(violation) => Err(self
                .intent_violation_error(&declaration, violation, None, non_admitted_trace, None)),
        }
    }
}

fn phase_three_family_violation(
    review: &crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
    message: &'static str,
) -> ForgeQueryIntentViolationDecision {
    ForgeQueryIntentViolationDecision::new(
        review.request().family(),
        review.request().entrypoint(),
        "phase-three-family-proof",
        message,
        review.request().request_digest(),
        review.eligibility().eligibility_digest(),
    )
}
