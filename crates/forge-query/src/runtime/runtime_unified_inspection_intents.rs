use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryGenericInspectionIntentSeed, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryUnifiedInspectionExecutionBinding,
    ForgeQueryUnifiedInspectionExecutionHandoff,
};

impl ForgeQueryRuntime {
    pub(crate) fn inspect_live_view_name_result(
        &self,
        view_name: &str,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        let seed = ForgeQueryGenericInspectionIntentSeed::from_target(
            ForgeQueryInspectionTarget::LiveView { name: view_name },
        )
        .expect("live view names should always lower into unified inspection seeds");
        let review = self.review_unified_inspection(seed)?;
        let handoff = self.resolve_reviewed_admitted_unified_inspection_handoff(review)?;
        let binding = self.prepare_unified_inspection_execution_binding(handoff);
        self.execute_unified_inspection_execution_binding(binding)
    }

    pub(crate) fn review_unified_inspection(
        &self,
        seed: ForgeQueryGenericInspectionIntentSeed,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let request =
            crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(seed)
                .expect("generic inspection seed should always author successfully");
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_unified_inspection_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryUnifiedInspectionExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::UnifiedInspection(plan),
            ) => Ok(ForgeQueryUnifiedInspectionExecutionHandoff::from_plan(plan)),
            _ => Err(self.unified_inspection_non_admitted_error(&review)),
        }
    }

    pub(crate) fn unified_inspection_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let violation = non_admitted_runtime_violation(review);
        ForgeQueryRuntimeError::MissingLiveView(violation.message().to_string())
    }

    pub(crate) fn prepare_unified_inspection_execution_binding(
        &self,
        handoff: ForgeQueryUnifiedInspectionExecutionHandoff,
    ) -> ForgeQueryUnifiedInspectionExecutionBinding {
        ForgeQueryUnifiedInspectionExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_unified_inspection_execution_binding(
        &self,
        binding: ForgeQueryUnifiedInspectionExecutionBinding,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        let inspection = self.inspect_from_generic_seed(binding.seed())?;
        let snapshot_identity = self.current_snapshot_identity();
        let receipt = ForgeQueryUnifiedInspectionReceipt::from_inspection(
            binding.seed().request_label().clone(),
            &inspection,
            snapshot_identity.clone(),
        );
        let mut result = ForgeQueryUnifiedInspectionResult::new(inspection, receipt);
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.seed().request_label().as_str(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.seed().request_input_digest(),
                result.receipt().result_digest(),
                "unified-inspection",
            );
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let execution_provenance =
            ForgeQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                binding.family(),
                binding.entrypoint(),
                binding.execution_seam(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.binding_digest(),
                result.receipt().result_digest(),
                &snapshot_evidence_identity,
            );
        result.attach_intent_admission_evidence(decision_trace_envelope, execution_provenance);
        Ok(result)
    }

    pub(crate) fn inspect_from_generic_seed(
        &self,
        seed: &ForgeQueryGenericInspectionIntentSeed,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError> {
        match seed.target() {
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::LiveView {
                view_name,
            } => {
                let state = self
                    .live_subscriptions
                    .get(&ForgeQueryLiveArtifactTarget::from_view_name(view_name))
                    .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.clone()))?;
                Ok(ForgeQueryInspection::LiveView(
                    ForgeQueryLiveViewInspection::from_state(state),
                ))
            }
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::Effect {
                effect_name,
            } => Ok(ForgeQueryInspection::Effect(
                self.inspect_effect_by_name(effect_name)?,
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::WriteReceipt(
                receipt,
            ) => {
                let runtime_evidence = self
                    .backend
                    .inspect_write_receipt(receipt, &self.evidence_authority)?;
                Ok(ForgeQueryInspection::WriteReceipt(
                    ForgeQueryWriteReceiptInspection::new(receipt, runtime_evidence),
                ))
            }
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::BatchWriteReceipt(
                receipt,
            ) => Ok(ForgeQueryInspection::BatchWriteReceipt(
                ForgeQueryBatchWriteReceiptInspection::new(receipt),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::IntentReceipt(
                receipt,
            ) => Ok(ForgeQueryInspection::IntentReceipt(
                ForgeQueryIntentReceiptInspection::from_receipt(receipt),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::IntentDenial(
                evidence,
            ) => Ok(ForgeQueryInspection::IntentDenial(
                ForgeQueryIntentDenialInspection::from_evidence(evidence),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::EffectIntentReceipt(
                receipt,
            ) => Ok(ForgeQueryInspection::EffectIntentReceipt(
                ForgeQueryEffectIntentReceiptInspection::from_receipt(receipt),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::PreviewBinding(
                binding,
            ) => Ok(ForgeQueryInspection::PreviewBinding(
                ForgeQueryPreviewBindingInspection::from_binding(binding),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::PreviewOutcome(
                outcome,
            ) => Ok(ForgeQueryInspection::PreviewOutcome(
                ForgeQueryPreviewOutcomeInspection::from_outcome(outcome),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::PreviewIntentReceipt(
                receipt,
            ) => Ok(ForgeQueryInspection::PreviewIntentReceipt(
                ForgeQueryPreviewIntentReceiptInspection::from_receipt(receipt),
            )),
            crate::intent_admission::ForgeQueryGenericInspectionIntentTargetSeed::BranchIntentReceipt(
                receipt,
            ) => Ok(ForgeQueryInspection::BranchIntentReceipt(
                ForgeQueryBranchIntentReceiptInspection::from_receipt(receipt),
            )),
        }
    }
}
