use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryLiveReadExecutionBinding,
    ForgeQueryLiveReadExecutionHandoff,
};
use crate::runtime::{
    ForgeQueryIntentConsumerInspection, ForgeQueryLiveReadResult, ForgeQueryLiveView,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

pub struct ForgeQueryWorkspaceLiveReadIntentAuthoring<'a, T> {
    workspace: &'a mut ForgeQueryWorkspace,
    live_view: ForgeQueryLiveView<T>,
}

impl<'a, T> ForgeQueryWorkspaceLiveReadIntentAuthoring<'a, T> {
    pub(crate) fn new(
        workspace: &'a mut ForgeQueryWorkspace,
        live_view: ForgeQueryLiveView<T>,
    ) -> Self {
        Self {
            workspace,
            live_view,
        }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryWorkspaceLiveReadIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self.workspace.review_live_read_execution(self.live_view)?;
        Ok(ForgeQueryWorkspaceLiveReadIntentAdmissionReview {
            workspace: self.workspace,
            review,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedWorkspaceLiveReadIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryWorkspaceLiveReadIntentAdmissionReview<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryWorkspaceLiveReadIntentAdmissionReview<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryLiveReadExecutionHandoff> {
        self.workspace
            .resolve_reviewed_admitted_live_read_execution_handoff(self.review.clone())
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
    ) -> Result<ForgeQueryAdmittedWorkspaceLiveReadIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_live_read_execution_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .live_read_execution_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .workspace
            .into_runtime_live_read_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedWorkspaceLiveReadIntent {
            workspace: self.workspace,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedWorkspaceLiveReadIntent<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryLiveReadExecutionHandoff,
    execution_binding: ForgeQueryLiveReadExecutionBinding,
}

impl<'a> ForgeQueryAdmittedWorkspaceLiveReadIntent<'a> {
    pub fn request(&self) -> &crate::intent_admission::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryLiveReadExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryLiveReadExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.workspace
            .execute_bound_live_read_execution(self.execution_binding)
    }
}
