use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryExistingTruthProbeExecutionBinding, ForgeQueryExistingTruthProbeExecutionHandoff,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope,
};
use crate::runtime::{
    ForgeQueryExistingTruthProbeRequest, ForgeQueryExistingTruthProbeResult,
    ForgeQueryIntentConsumerInspection, ForgeQueryRuntime, ForgeQueryRuntimeError,
};

pub struct ForgeQueryRuntimeExistingTruthProbeIntentAuthoring<'a> {
    runtime: &'a ForgeQueryRuntime,
    request: ForgeQueryExistingTruthProbeRequest,
}

impl<'a> ForgeQueryRuntimeExistingTruthProbeIntentAuthoring<'a> {
    pub(crate) fn new(
        runtime: &'a ForgeQueryRuntime,
        request: ForgeQueryExistingTruthProbeRequest,
    ) -> Self {
        Self { runtime, request }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview<'a>, ForgeQueryRuntimeError>
    {
        let review = self
            .runtime
            .review_existing_truth_probe_routing(self.request)?;
        Ok(ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview {
            runtime: self.runtime,
            review,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeExistingTruthProbeIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview<'a> {
    runtime: &'a ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryRuntimeExistingTruthProbeIntentAdmissionReview<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryExistingTruthProbeExecutionHandoff> {
        self.runtime
            .resolve_reviewed_admitted_existing_truth_probe_handoff(self.review.clone())
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

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeExistingTruthProbeIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_existing_truth_probe_handoff(self.review.clone())
            .map_err(|_| {
                self.runtime
                    .existing_truth_probe_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .runtime
            .prepare_existing_truth_probe_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedRuntimeExistingTruthProbeIntent {
            runtime: self.runtime,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedRuntimeExistingTruthProbeIntent<'a> {
    runtime: &'a ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryExistingTruthProbeExecutionHandoff,
    execution_binding: ForgeQueryExistingTruthProbeExecutionBinding,
}

impl<'a> ForgeQueryAdmittedRuntimeExistingTruthProbeIntent<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryExistingTruthProbeExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryExistingTruthProbeExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
        self.runtime
            .execute_existing_truth_probe_execution_binding(self.execution_binding)
    }
}
