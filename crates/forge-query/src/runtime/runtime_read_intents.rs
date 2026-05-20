use super::read_composition_runtime::{
    execute_runtime_basis_context_read_graph, execute_runtime_current_read_graph,
};
use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryLiveReadExecutionBinding,
    ForgeQueryLiveReadExecutionHandoff, ForgeQueryLiveReadIntentSeed,
    ForgeQueryReadExecutionBinding, ForgeQueryReadExecutionHandoff,
    ForgeQueryReadExecutionIntentSeed,
};
use crate::query_context::AdmittedQueryBasisContext;

impl ForgeQueryWorkspace {
    pub fn read_family_intent(
        &mut self,
        family: &ForgeQueryReadFamily,
    ) -> crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring::new(
            self,
            family.clone(),
            None,
        )
    }

    pub fn read_family_in_basis_context_intent(
        &mut self,
        family: &ForgeQueryReadFamily,
        context: &AdmittedQueryBasisContext,
    ) -> crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring::new(
            self,
            family.clone(),
            Some(context.clone()),
        )
    }

    pub fn read_live_intent<T>(
        &mut self,
        live_view: &ForgeQueryLiveView<T>,
    ) -> crate::intent_admission::ForgeQueryWorkspaceLiveReadIntentAuthoring<'_, T> {
        crate::intent_admission::ForgeQueryWorkspaceLiveReadIntentAuthoring::new(
            self,
            live_view.clone(),
        )
    }

    pub(crate) fn review_read_execution(
        &self,
        family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.runtime
            .review_runtime_read_execution(family, basis_context)
    }

    pub(crate) fn resolve_reviewed_admitted_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryReadExecutionHandoff, ForgeQueryRuntimeError> {
        self.runtime
            .resolve_reviewed_admitted_read_execution_handoff(review)
    }

    pub(crate) fn read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime.read_execution_non_admitted_error(review)
    }

    pub(crate) fn into_runtime_read_execution_binding(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
    ) -> ForgeQueryReadExecutionBinding {
        self.runtime.prepare_read_execution_binding(handoff)
    }

    pub(crate) fn execute_bound_read_execution(
        &mut self,
        binding: ForgeQueryReadExecutionBinding,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime.execute_read_execution_binding(binding)
    }

    pub(crate) fn review_live_read_execution<T>(
        &self,
        live_view: ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.runtime
            .review_runtime_live_read_execution(live_view.subscription_installation().clone())
    }

    pub(crate) fn resolve_reviewed_admitted_live_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryLiveReadExecutionHandoff, ForgeQueryRuntimeError> {
        self.runtime
            .resolve_reviewed_admitted_live_read_execution_handoff(review)
    }

    pub(crate) fn live_read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime.live_read_execution_non_admitted_error(review)
    }

    pub(crate) fn into_runtime_live_read_execution_binding(
        &self,
        handoff: ForgeQueryLiveReadExecutionHandoff,
    ) -> ForgeQueryLiveReadExecutionBinding {
        self.runtime.prepare_live_read_execution_binding(handoff)
    }

    pub(crate) fn execute_bound_live_read_execution(
        &mut self,
        binding: ForgeQueryLiveReadExecutionBinding,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.runtime.execute_live_read_execution_binding(binding)
    }
}

impl ForgeQueryRuntime {
    pub(crate) fn review_runtime_read_execution(
        &self,
        family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let request = match basis_context {
            Some(context) => {
                let seed = ForgeQueryReadExecutionIntentSeed::in_basis_context(family, context);
                crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(seed)
            }
            None => {
                let seed = ForgeQueryReadExecutionIntentSeed::current_runtime(family);
                crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(seed)
            }
        }
        .map_err(|violation| ForgeQueryRuntimeError::ReadCompositionDenied(
            ForgeQueryReadDenial::new(ForgeQueryReadDenialKind::AuthoringDenied, violation.message()),
        ))?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryReadExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::ReadExecution(plan),
            ) => Ok(ForgeQueryReadExecutionHandoff::from_plan(plan)),
            ForgeQueryIntentAdmissionDecision::Admitted(_)
            | ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => {
                Err(self.read_execution_non_admitted_error(&review))
            }
        }
    }

    pub(crate) fn read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let violation = non_admitted_runtime_violation(review);
        ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisPreflightDenied,
            violation.message(),
        ))
    }

    pub(crate) fn prepare_read_execution_binding(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
    ) -> ForgeQueryReadExecutionBinding {
        ForgeQueryReadExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_read_execution_binding(
        &mut self,
        binding: ForgeQueryReadExecutionBinding,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let mut result = match binding.basis_context() {
            Some(context) => execute_runtime_basis_context_read_graph(
                self,
                binding.read_family().read_graph(),
                context,
            ),
            None => execute_runtime_current_read_graph(self, binding.read_family().read_graph()),
        }
        .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.read_family().family_name(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.read_family().family_name(),
                result.receipt().result_digest(),
                binding.execution_seam().as_str(),
            );
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_shared_execution_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            result.receipt().result_digest(),
            result.receipt().snapshot_token(),
        );
        result.attach_intent_admission_evidence(decision_trace_envelope, execution_provenance);
        Ok(result)
    }

    pub(crate) fn review_runtime_live_read_execution(
        &self,
        installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let seed = ForgeQueryLiveReadIntentSeed::from_installation(&installation);
        let request =
            crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::live_read_entrypoint(
                seed,
            )
            .map_err(|violation| {
                ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
                    ForgeQueryReadDenialKind::AuthoringDenied,
                    violation.message(),
                ))
            })?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_live_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryLiveReadExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::LiveReadExecution(plan),
            ) => Ok(ForgeQueryLiveReadExecutionHandoff::from_plan(plan)),
            ForgeQueryIntentAdmissionDecision::Admitted(_)
            | ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => {
                Err(self.live_read_execution_non_admitted_error(&review))
            }
        }
    }

    pub(crate) fn live_read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let violation = non_admitted_runtime_violation(review);
        ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisPreflightDenied,
            violation.message(),
        ))
    }

    pub(crate) fn prepare_live_read_execution_binding(
        &self,
        handoff: ForgeQueryLiveReadExecutionHandoff,
    ) -> ForgeQueryLiveReadExecutionBinding {
        ForgeQueryLiveReadExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_live_read_execution_binding(
        &mut self,
        binding: ForgeQueryLiveReadExecutionBinding,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let rows = self
            .backend
            .live_entities(binding.installation().view_name());
        let snapshot_token = self.backend.snapshot_token();
        let receipt =
            ForgeQueryLiveReadReceipt::from_rows(binding.installation(), snapshot_token, &rows);
        let mut result = ForgeQueryLiveReadResult::new(rows, receipt);
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.installation().view_name(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.installation().view_name(),
                result.receipt().result_digest(),
                "live-view-read",
            );
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_shared_execution_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            result.receipt().result_digest(),
            result.receipt().snapshot_token(),
        );
        result.attach_intent_admission_evidence(decision_trace_envelope, execution_provenance);
        Ok(result)
    }
}
