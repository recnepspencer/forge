use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryReadExecutionBinding, ForgeQueryReadExecutionHandoff,
};
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{
    ForgeQueryIntentConsumerInspection, ForgeQueryReadFamily, ForgeQueryReadResult,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

pub struct ForgeQueryWorkspaceReadIntentAuthoring<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    read_family: ForgeQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
}

impl<'a> ForgeQueryWorkspaceReadIntentAuthoring<'a> {
    pub(crate) fn new(
        workspace: &'a mut ForgeQueryWorkspace,
        read_family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
    ) -> Self {
        Self {
            workspace,
            read_family,
            basis_context,
        }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryWorkspaceReadIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self
            .workspace
            .review_read_execution(self.read_family, self.basis_context)?;
        Ok(ForgeQueryWorkspaceReadIntentAdmissionReview {
            workspace: self.workspace,
            review,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedWorkspaceReadIntent<'a>, ForgeQueryRuntimeError> {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryWorkspaceReadIntentAdmissionReview<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryWorkspaceReadIntentAdmissionReview<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryReadExecutionHandoff> {
        self.workspace
            .resolve_reviewed_admitted_read_execution_handoff(self.review.clone())
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
    ) -> Result<ForgeQueryAdmittedWorkspaceReadIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_read_execution_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .read_execution_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .workspace
            .into_runtime_read_execution_binding(handoff.clone());
        Ok(ForgeQueryAdmittedWorkspaceReadIntent {
            workspace: self.workspace,
            review: self.review,
            handoff,
            execution_binding,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryAdmittedWorkspaceReadIntent<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryReadExecutionHandoff,
    execution_binding: ForgeQueryReadExecutionBinding,
}

impl<'a> ForgeQueryAdmittedWorkspaceReadIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryReadExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryReadExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.workspace
            .execute_bound_read_execution(self.execution_binding)
    }
}
