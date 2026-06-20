use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionEligibility,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryReadExecutionBinding, ForgeQueryReadExecutionHandoff,
};
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphIndexInventoryMatchReport,
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAuthorityContext,
    ForgeQueryGraphReadAccessPlanExplanation, ForgeQueryIntentConsumerInspection,
    ForgeQueryReadFamily, ForgeQueryReadResult, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

pub struct ForgeQueryWorkspaceReadIntentAuthoring<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    read_family: ForgeQueryReadFamily,
    basis_context: Option<AdmittedQueryBasisContext>,
    graph_read_authority: Option<ForgeQueryGraphReadAccessAuthorityContext>,
}

impl<'a> ForgeQueryWorkspaceReadIntentAuthoring<'a> {
    pub(crate) fn new(
        workspace: &'a mut ForgeQueryWorkspace,
        read_family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
        graph_read_authority: Option<ForgeQueryGraphReadAccessAuthorityContext>,
    ) -> Self {
        Self {
            workspace,
            read_family,
            basis_context,
            graph_read_authority,
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
            graph_read_authority: self.graph_read_authority,
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
    graph_read_authority: Option<ForgeQueryGraphReadAccessAuthorityContext>,
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

    pub fn graph_read_access_admission(
        &self,
    ) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryRuntimeError> {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_read_execution_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .read_execution_non_admitted_error(&self.review)
            })?;
        admit_graph_read_access_for_review_authority(
            self.workspace,
            handoff.read_family(),
            self.graph_read_authority.as_ref(),
        )
        .map_err(|error| {
            ForgeQueryRuntimeError::ReadCompositionDenied(
                crate::runtime::ForgeQueryReadDenial::new(
                    crate::runtime::ForgeQueryReadDenialKind::AuthoringDenied,
                    error.as_str(),
                ),
            )
        })
    }

    pub fn graph_read_access_plan(
        &self,
    ) -> Result<ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryRuntimeError> {
        let handoff = self
            .workspace
            .resolve_reviewed_admitted_read_execution_handoff(self.review.clone())
            .map_err(|_| {
                self.workspace
                    .read_execution_non_admitted_error(&self.review)
            })?;
        let admission = admit_graph_read_access_for_review_authority(
            self.workspace,
            handoff.read_family(),
            self.graph_read_authority.as_ref(),
        )
        .map_err(|error| {
            ForgeQueryRuntimeError::ReadCompositionDenied(
                crate::runtime::ForgeQueryReadDenial::new(
                    crate::runtime::ForgeQueryReadDenialKind::AuthoringDenied,
                    error.as_str(),
                ),
            )
        })?;
        ForgeQueryAdmittedGraphReadAccessPlan::from_admission(admission.clone()).ok_or_else(|| {
            let detail = admission
                .denial()
                .map(|denial| denial.kind().as_str())
                .unwrap_or("graph_read_access_not_admitted");
                ForgeQueryRuntimeError::ReadCompositionDenied(
                    crate::runtime::ForgeQueryReadDenial::new(
                        crate::runtime::ForgeQueryReadDenialKind::BasisPreflightDenied,
                        detail,
                    )
                    .with_graph_read_persistent_artifact_audit_for_admission(&admission)
                    .with_graph_read_access_admission(admission)
                    .with_graph_read_access_execution_counters(
                        crate::runtime::ForgeQueryGraphReadAccessExecutionCounters::pre_execution_denial(
                    ),
                ),
            )
        })
    }

    pub fn graph_index_support(
        &self,
    ) -> Result<ForgeQueryGraphIndexInventoryMatchReport, ForgeQueryRuntimeError> {
        Ok(self
            .graph_read_access_admission()?
            .graph_index_inventory_match_report()
            .clone())
    }

    pub fn graph_read_access_explanation(
        &self,
    ) -> Result<ForgeQueryGraphReadAccessPlanExplanation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryGraphReadAccessPlanExplanation::from_admission(
            &self.graph_read_access_admission()?,
        ))
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
            .into_runtime_read_execution_binding_in_authority(
                handoff.clone(),
                self.graph_read_authority.as_ref(),
            )?;
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

fn admit_graph_read_access_for_review_authority(
    workspace: &ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
    authority: Option<&ForgeQueryGraphReadAccessAuthorityContext>,
) -> Result<
    ForgeQueryGraphReadAccessAdmission,
    crate::runtime::ForgeQueryGraphReadAccessShapeExplanationError,
> {
    match authority {
        Some(authority) => {
            workspace.admit_graph_read_access_for_family_in_authority(family, authority)
        }
        None => workspace.admit_graph_read_access_for_family(family),
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
