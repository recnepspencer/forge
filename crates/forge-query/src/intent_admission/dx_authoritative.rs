use crate::intent_admission::{
    ForgeQueryAuthoritativeIntentExecutionBinding, ForgeQueryAuthoritativeIntentExecutionHandoff,
};
use crate::runtime::{
    ForgeQueryIntentDeclaration, ForgeQueryIntentReceipt, ForgeQueryRuntime, ForgeQueryRuntimeError,
};

use super::{
    non_admitted_runtime_violation, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryRawIntentAdmissionRequest, ForgeQueryRuntimeIntentAdmissionReviewData,
};

pub struct ForgeQueryRuntimeIntentAuthoring<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    declaration: ForgeQueryIntentDeclaration,
}

impl<'a> ForgeQueryRuntimeIntentAuthoring<'a> {
    pub(crate) fn new(
        runtime: &'a mut ForgeQueryRuntime,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Self {
        Self {
            runtime,
            declaration,
        }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self
            .runtime
            .review_authoritative_runtime_intent(self.declaration)?;
        Ok(ForgeQueryRuntimeIntentAdmissionReview {
            runtime: self.runtime,
            review,
        })
    }

    pub fn admit(self) -> Result<ForgeQueryAdmittedRuntimeIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryRuntimeIntentAdmissionReview<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryRuntimeIntentAdmissionReview<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_plan(&self) -> Option<&super::ForgeQueryAdmittedIntentPlan> {
        self.review.admitted_plan()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryAuthoritativeIntentExecutionHandoff> {
        self.runtime
            .resolve_reviewed_admitted_authoritative_intent_handoff(self.review.clone())
            .ok()
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }

    pub fn admit(self) -> Result<ForgeQueryAdmittedRuntimeIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_authoritative_intent_handoff(self.review.clone())
            .map_err(|_| {
                let violation = non_admitted_runtime_violation(&self.review);
                self.runtime.intent_violation_error(
                    self.review.request().declaration(),
                    violation,
                    None,
                    self.review.decision_trace_envelope().cloned(),
                    None,
                )
            })?;
        let execution_binding = self
            .runtime
            .prepare_authoritative_intent_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedRuntimeIntent {
            runtime: self.runtime,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedRuntimeIntent<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryAuthoritativeIntentExecutionHandoff,
    execution_binding: ForgeQueryAuthoritativeIntentExecutionBinding,
}

impl<'a> ForgeQueryAdmittedRuntimeIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryAuthoritativeIntentExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryAuthoritativeIntentExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime
            .execute_authoritative_intent_execution_binding(self.execution_binding)
    }
}
