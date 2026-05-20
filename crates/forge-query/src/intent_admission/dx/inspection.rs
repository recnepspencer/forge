use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryDerivedInspectionExecutionBinding, ForgeQueryDerivedInspectionExecutionHandoff,
    ForgeQueryDerivedMaterializationExecutionBinding,
    ForgeQueryDerivedMaterializationExecutionHandoff, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryRawIntentAdmissionRequest,
};
use crate::runtime::{
    ForgeQueryDerivedInspectionResult, ForgeQueryDerivedMaterializationResult,
    ForgeQueryIntentConsumerInspection, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

pub struct ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    view_name: String,
}

impl<'a> ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring<'a> {
    pub(crate) fn new(workspace: &'a mut ForgeQueryWorkspace, view_name: String) -> Self {
        Self {
            workspace,
            view_name,
        }
    }

    pub fn review(
        self,
    ) -> Result<
        ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview<'a>,
        ForgeQueryRuntimeError,
    > {
        let review = self
            .workspace
            .review_derived_materialization(self.view_name)?;
        Ok(
            ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview {
                workspace: self.workspace,
                review,
            },
        )
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent<'a>, ForgeQueryRuntimeError>
    {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryWorkspaceDerivedMaterializationIntentAdmissionReview<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryDerivedMaterializationExecutionHandoff> {
        self.workspace
            .resolve_reviewed_admitted_derived_materialization_handoff(self.review.clone())
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
    ) -> Result<ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent<'a>, ForgeQueryRuntimeError>
    {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_derived_materialization_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .derived_materialization_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .workspace
            .into_runtime_derived_materialization_binding(handoff.clone());
        Ok(ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent {
            workspace: self.workspace,
            review: self.review,
            handoff,
            execution_binding,
        })
    }
}

pub struct ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryDerivedMaterializationExecutionHandoff,
    execution_binding: ForgeQueryDerivedMaterializationExecutionBinding,
}

impl<'a> ForgeQueryAdmittedWorkspaceDerivedMaterializationIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryDerivedMaterializationExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryDerivedMaterializationExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.workspace
            .execute_bound_derived_materialization(self.execution_binding)
    }
}

pub struct ForgeQueryWorkspaceDerivedInspectionIntentAuthoring<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    view_name: String,
}

impl<'a> ForgeQueryWorkspaceDerivedInspectionIntentAuthoring<'a> {
    pub(crate) fn new(workspace: &'a mut ForgeQueryWorkspace, view_name: String) -> Self {
        Self {
            workspace,
            view_name,
        }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview<'a>, ForgeQueryRuntimeError>
    {
        let review = self.workspace.review_derived_inspection(self.view_name)?;
        Ok(ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview {
            workspace: self.workspace,
            review,
        })
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedWorkspaceDerivedInspectionIntent<'a>, ForgeQueryRuntimeError>
    {
        self.review()?.admit()
    }

    pub fn execute(self) -> Result<ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
        self.admit()?.execute()
    }
}

pub struct ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl<'a> ForgeQueryWorkspaceDerivedInspectionIntentAdmissionReview<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn admitted_handoff(&self) -> Option<ForgeQueryDerivedInspectionExecutionHandoff> {
        self.workspace
            .resolve_reviewed_admitted_derived_inspection_handoff(self.review.clone())
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
    ) -> Result<ForgeQueryAdmittedWorkspaceDerivedInspectionIntent<'a>, ForgeQueryRuntimeError>
    {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_derived_inspection_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .derived_inspection_non_admitted_error(&self.review)
            })?;
        let execution_binding = self
            .workspace
            .into_runtime_derived_inspection_binding(handoff.clone());
        Ok(ForgeQueryAdmittedWorkspaceDerivedInspectionIntent {
            workspace: self.workspace,
            review: self.review,
            handoff,
            execution_binding,
        })
    }
}

pub struct ForgeQueryAdmittedWorkspaceDerivedInspectionIntent<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
    handoff: ForgeQueryDerivedInspectionExecutionHandoff,
    execution_binding: ForgeQueryDerivedInspectionExecutionBinding,
}

impl<'a> ForgeQueryAdmittedWorkspaceDerivedInspectionIntent<'a> {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn handoff(&self) -> &ForgeQueryDerivedInspectionExecutionHandoff {
        &self.handoff
    }

    pub fn execution_binding(&self) -> &ForgeQueryDerivedInspectionExecutionBinding {
        &self.execution_binding
    }

    pub fn execute(self) -> Result<ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
        self.workspace
            .execute_bound_derived_inspection(self.execution_binding)
    }
}
