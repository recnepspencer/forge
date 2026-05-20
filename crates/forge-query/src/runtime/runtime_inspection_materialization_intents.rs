use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryDerivedInspectionExecutionBinding, ForgeQueryDerivedInspectionExecutionHandoff,
    ForgeQueryDerivedMaterializationExecutionBinding,
    ForgeQueryDerivedMaterializationExecutionHandoff, ForgeQueryDerivedViewIntentSeed,
    ForgeQueryIntentAdmissionDecision,
};

impl ForgeQueryWorkspace {
    pub fn materialize_intent<T>(
        &mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> crate::intent_admission::ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceDerivedMaterializationIntentAuthoring::new(
            self,
            view.name().to_string(),
        )
    }

    pub fn inspect_derived_intent<T>(
        &mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> crate::intent_admission::ForgeQueryWorkspaceDerivedInspectionIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceDerivedInspectionIntentAuthoring::new(
            self,
            view.name().to_string(),
        )
    }

    pub(crate) fn review_derived_materialization(
        &self,
        view_name: String,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.runtime
            .review_runtime_derived_materialization(view_name)
    }

    pub(crate) fn resolve_reviewed_admitted_derived_materialization_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryDerivedMaterializationExecutionHandoff, ForgeQueryRuntimeError> {
        self.runtime
            .resolve_reviewed_admitted_derived_materialization_handoff(review)
    }

    pub(crate) fn derived_materialization_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime
            .derived_materialization_non_admitted_error(review)
    }

    pub(crate) fn into_runtime_derived_materialization_binding(
        &self,
        handoff: ForgeQueryDerivedMaterializationExecutionHandoff,
    ) -> ForgeQueryDerivedMaterializationExecutionBinding {
        self.runtime
            .prepare_derived_materialization_execution_binding(handoff)
    }

    pub(crate) fn execute_bound_derived_materialization(
        &self,
        binding: ForgeQueryDerivedMaterializationExecutionBinding,
    ) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.runtime
            .execute_derived_materialization_execution_binding(binding)
    }

    pub(crate) fn review_derived_inspection(
        &self,
        view_name: String,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.runtime.review_runtime_derived_inspection(view_name)
    }

    pub(crate) fn resolve_reviewed_admitted_derived_inspection_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryDerivedInspectionExecutionHandoff, ForgeQueryRuntimeError> {
        self.runtime
            .resolve_reviewed_admitted_derived_inspection_handoff(review)
    }

    pub(crate) fn derived_inspection_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime.derived_inspection_non_admitted_error(review)
    }

    pub(crate) fn into_runtime_derived_inspection_binding(
        &self,
        handoff: ForgeQueryDerivedInspectionExecutionHandoff,
    ) -> ForgeQueryDerivedInspectionExecutionBinding {
        self.runtime
            .prepare_derived_inspection_execution_binding(handoff)
    }

    pub(crate) fn execute_bound_derived_inspection(
        &self,
        binding: ForgeQueryDerivedInspectionExecutionBinding,
    ) -> Result<ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
        self.runtime
            .execute_derived_inspection_execution_binding(binding)
    }
}

impl ForgeQueryRuntime {
    pub(crate) fn review_runtime_derived_materialization(
        &self,
        view_name: String,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)?;
        let seed = self.derived_view_intent_seed(&view_name)?;
        let request =
            crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(seed)
                .map_err(|violation| {
                    ForgeQueryRuntimeError::MissingDerivedView(violation.message().to_string())
                })?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn review_runtime_derived_inspection(
        &self,
        view_name: String,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let seed = self.derived_view_intent_seed(&view_name)?;
        let request =
            crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(seed)
                .map_err(|violation| {
                    ForgeQueryRuntimeError::MissingDerivedView(violation.message().to_string())
                })?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_derived_materialization_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryDerivedMaterializationExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::DerivedMaterialization(plan),
            ) => Ok(ForgeQueryDerivedMaterializationExecutionHandoff::from_plan(
                plan,
            )),
            _ => Err(self.derived_materialization_non_admitted_error(&review)),
        }
    }

    pub(crate) fn resolve_reviewed_admitted_derived_inspection_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryDerivedInspectionExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::DerivedInspection(plan),
            ) => Ok(ForgeQueryDerivedInspectionExecutionHandoff::from_plan(plan)),
            _ => Err(self.derived_inspection_non_admitted_error(&review)),
        }
    }

    pub(crate) fn derived_materialization_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        ForgeQueryRuntimeError::MissingDerivedView(
            non_admitted_runtime_violation(review).message().to_string(),
        )
    }

    pub(crate) fn derived_inspection_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        ForgeQueryRuntimeError::MissingDerivedView(
            non_admitted_runtime_violation(review).message().to_string(),
        )
    }

    pub(crate) fn prepare_derived_materialization_execution_binding(
        &self,
        handoff: ForgeQueryDerivedMaterializationExecutionHandoff,
    ) -> ForgeQueryDerivedMaterializationExecutionBinding {
        ForgeQueryDerivedMaterializationExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn prepare_derived_inspection_execution_binding(
        &self,
        handoff: ForgeQueryDerivedInspectionExecutionHandoff,
    ) -> ForgeQueryDerivedInspectionExecutionBinding {
        ForgeQueryDerivedInspectionExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_derived_materialization_execution_binding(
        &self,
        binding: ForgeQueryDerivedMaterializationExecutionBinding,
    ) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        let evidence = self.derived_view_evidence(binding.view_name())?;
        let rows = self
            .derived_views
            .get(binding.view_name())
            .map(|runtime| runtime.materialization.rows().to_vec())
            .ok_or_else(|| {
                ForgeQueryRuntimeError::MissingDerivedView(binding.view_name().to_string())
            })?;
        let receipt = ForgeQueryDerivedMaterializationReceipt::from_evidence(
            &evidence,
            self.backend.snapshot_token(),
        );
        let mut result = ForgeQueryDerivedMaterializationResult::new(rows, receipt);
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.view_name(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.view_name(),
                result.receipt().result_digest(),
                "derived-view-materialization",
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

    pub(crate) fn execute_derived_inspection_execution_binding(
        &self,
        binding: ForgeQueryDerivedInspectionExecutionBinding,
    ) -> Result<ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
        let evidence = self.derived_view_evidence(binding.view_name())?;
        let receipt = ForgeQueryDerivedInspectionReceipt::from_evidence(
            &evidence,
            self.backend.snapshot_token(),
        );
        let mut result = ForgeQueryDerivedInspectionResult::new(evidence, receipt);
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.view_name(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.view_name(),
                result.receipt().result_digest(),
                "derived-view-inspection",
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

    fn derived_view_intent_seed(
        &self,
        view_name: &str,
    ) -> Result<ForgeQueryDerivedViewIntentSeed, ForgeQueryRuntimeError> {
        let handle = ForgeQueryDerivedViewHandle::<Value>::new(view_name);
        let evidence = self.derived_view_evidence(view_name)?;
        Ok(ForgeQueryDerivedViewIntentSeed::new(
            &handle,
            evidence.authority_lane(),
            evidence.dependency_digest(),
            evidence.materialization_digest(),
            evidence.inspection_digest(),
            evidence.materialized_row_count(),
        ))
    }

    fn derived_view_evidence(
        &self,
        view_name: &str,
    ) -> Result<ForgeQueryComputedInspectionEvidence, ForgeQueryRuntimeError> {
        self.derived_views
            .get(view_name)
            .map(ForgeQueryComputedInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingDerivedView(view_name.to_string()))
    }
}
