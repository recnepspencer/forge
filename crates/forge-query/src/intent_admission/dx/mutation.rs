use crate::intent_admission::{
    ForgeQueryAuthoritativeMutationExecutionBinding,
    ForgeQueryAuthoritativeMutationExecutionHandoff,
};
use crate::runtime::{
    ForgeQueryIntentConsumerInspection, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

use super::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryRuntimeIntentAdmissionReviewData,
};

pub struct ForgeQueryRuntimeWriteIntentAuthoring<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    command: ForgeQueryWriteCommand,
}

impl<'a> ForgeQueryRuntimeWriteIntentAuthoring<'a> {
    pub(crate) fn new(runtime: &'a mut ForgeQueryRuntime, command: ForgeQueryWriteCommand) -> Self {
        Self { runtime, command }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeWriteIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self
            .runtime
            .review_authoritative_runtime_write(self.command)?;
        Ok(ForgeQueryRuntimeWriteIntentAdmissionReview {
            runtime: self.runtime,
            review,
        })
    }

    pub fn admit(self) -> Result<ForgeQueryAdmittedRuntimeWriteIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryRuntimeWriteIntentAdmissionReview<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryRuntimeWriteIntentAdmissionReview<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryAuthoritativeMutationExecutionHandoff> {
        self.runtime
            .resolve_reviewed_admitted_authoritative_write_handoff(self.review.clone())
            .ok()
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }

    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        ForgeQueryIntentConsumerInspection::from_review(
            self.review.request().intent_name(),
            self.review.decision(),
            self.review.request().family(),
            self.review.request().entrypoint(),
            self.review.decision_trace_envelope(),
        )
    }

    pub fn admit(self) -> Result<ForgeQueryAdmittedRuntimeWriteIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_authoritative_write_handoff(self.review.clone())?;
        let execution_binding = self
            .runtime
            .prepare_authoritative_mutation_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedRuntimeWriteIntent {
            runtime: self.runtime,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedRuntimeWriteIntent<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryAuthoritativeMutationExecutionHandoff,
    execution_binding: ForgeQueryAuthoritativeMutationExecutionBinding,
}

impl<'a> ForgeQueryAdmittedRuntimeWriteIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryAuthoritativeMutationExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryAuthoritativeMutationExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.runtime
            .execute_authoritative_mutation_execution_binding(self.execution_binding)
    }
}
