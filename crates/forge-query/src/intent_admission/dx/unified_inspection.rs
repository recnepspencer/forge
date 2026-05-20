use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryGenericInspectionIntentSeed, ForgeQueryGenericInspectionIntentTarget,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryUnifiedInspectionExecutionBinding,
    ForgeQueryUnifiedInspectionExecutionHandoff,
};
use crate::runtime::{
    ForgeQueryIntentConsumerInspection, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryUnifiedInspectionResult,
};

pub struct ForgeQueryRuntimeInspectionIntentAuthoring<'a> {
    runtime: &'a ForgeQueryRuntime,
    seed: ForgeQueryGenericInspectionIntentSeed,
}

impl<'a> ForgeQueryRuntimeInspectionIntentAuthoring<'a> {
    pub(crate) fn new(
        runtime: &'a ForgeQueryRuntime,
        seed: ForgeQueryGenericInspectionIntentSeed,
    ) -> Self {
        Self { runtime, seed }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeInspectionIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self.runtime.review_unified_inspection(self.seed)?;
        Ok(ForgeQueryRuntimeInspectionIntentAdmissionReview {
            runtime: self.runtime,
            review,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeInspectionIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryRuntimeInspectionIntentAdmissionReview<'a> {
    runtime: &'a ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryRuntimeInspectionIntentAdmissionReview<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryUnifiedInspectionExecutionHandoff> {
        self.runtime
            .resolve_reviewed_admitted_unified_inspection_handoff(self.review.clone())
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
    ) -> Result<ForgeQueryAdmittedRuntimeInspectionIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_unified_inspection_handoff(self.review.clone())
            .map_err(|_| {
                self.runtime
                    .unified_inspection_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .runtime
            .prepare_unified_inspection_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedRuntimeInspectionIntent {
            runtime: self.runtime,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedRuntimeInspectionIntent<'a> {
    runtime: &'a ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryUnifiedInspectionExecutionHandoff,
    execution_binding: ForgeQueryUnifiedInspectionExecutionBinding,
}

impl<'a> ForgeQueryAdmittedRuntimeInspectionIntent<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryUnifiedInspectionExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryUnifiedInspectionExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.runtime
            .execute_unified_inspection_execution_binding(self.execution_binding)
    }
}

impl ForgeQueryRuntime {
    pub fn inspect_intent<'a, T>(
        &'a self,
        target: T,
    ) -> ForgeQueryRuntimeInspectionIntentAuthoring<'a>
    where
        T: ForgeQueryGenericInspectionIntentTarget<'a>,
    {
        ForgeQueryRuntimeInspectionIntentAuthoring::new(
            self,
            target.into_generic_inspection_intent_seed(),
        )
    }
}
