use crate::intent_admission::{
    ForgeQueryEffectTriggeredIntentExecutionBinding,
    ForgeQueryEffectTriggeredIntentExecutionHandoff,
};
use crate::runtime::{
    ForgeQueryEffectDelivery, ForgeQueryEffectHandle, ForgeQueryEffectIntentReceipt,
    ForgeQueryRuntime, ForgeQueryRuntimeError,
};

use super::{
    non_admitted_runtime_violation, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryRawIntentAdmissionRequest, ForgeQueryRuntimeIntentAdmissionReviewData,
};

pub struct ForgeQueryRuntimeEffectWriteIntentAuthoring<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    effect_name: String,
    strategy_version: String,
    input_contract: String,
}

impl<'a> ForgeQueryRuntimeEffectWriteIntentAuthoring<'a> {
    pub(crate) fn new<T>(
        runtime: &'a mut ForgeQueryRuntime,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: String,
        input_contract: String,
    ) -> Self {
        Self {
            runtime,
            effect_name: effect.name().to_string(),
            strategy_version,
            input_contract,
        }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeEffectWriteIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let (pending_delivery, review) = self.runtime.review_next_effect_write_runtime_intent(
            &self.effect_name,
            self.strategy_version,
            self.input_contract,
        )?;
        Ok(ForgeQueryRuntimeEffectWriteIntentAdmissionReview {
            runtime: self.runtime,
            review,
            pending_delivery,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeEffectWriteIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryRuntimeEffectWriteIntentAdmissionReview<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    pending_delivery: ForgeQueryEffectDelivery,
}

impl<'a> ForgeQueryRuntimeEffectWriteIntentAdmissionReview<'a> {
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

    pub fn admitted_handoff(&self) -> Option<ForgeQueryEffectTriggeredIntentExecutionHandoff> {
        self.runtime
            .resolve_reviewed_admitted_effect_intent_handoff(self.review.clone())
            .ok()
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }

    pub fn pending_delivery(&self) -> &ForgeQueryEffectDelivery {
        &self.pending_delivery
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeEffectWriteIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_effect_intent_handoff(self.review.clone())
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
            .prepare_effect_intent_execution_binding(handoff.clone(), &self.pending_delivery);
        Ok(ForgeQueryAdmittedRuntimeEffectWriteIntent {
            runtime: self.runtime,
            review: self.review,
            handoff,
            execution_binding,
            pending_delivery: self.pending_delivery,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedRuntimeEffectWriteIntent<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryEffectTriggeredIntentExecutionHandoff,
    execution_binding: ForgeQueryEffectTriggeredIntentExecutionBinding,
    pending_delivery: ForgeQueryEffectDelivery,
}

impl<'a> ForgeQueryAdmittedRuntimeEffectWriteIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryEffectTriggeredIntentExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryEffectTriggeredIntentExecutionBinding {
        &self.execution_binding
    }

    pub fn pending_delivery(&self) -> &ForgeQueryEffectDelivery {
        &self.pending_delivery
    }

    pub fn execute(self) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime
            .execute_effect_intent_execution_binding(self.execution_binding)
    }
}
