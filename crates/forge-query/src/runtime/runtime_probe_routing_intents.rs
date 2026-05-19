use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryExistingTruthProbeExecutionBinding, ForgeQueryExistingTruthProbeExecutionHandoff,
    ForgeQueryExistingTruthProbeIntentSeed, ForgeQueryExistingTruthProbeRoutingPreflight,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentDecisionTraceEnvelope,
};

impl ForgeQueryRuntime {
    pub fn probe_existing_intent(
        &self,
        request: ForgeQueryExistingTruthProbeRequest,
    ) -> crate::intent_admission::ForgeQueryRuntimeExistingTruthProbeIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryRuntimeExistingTruthProbeIntentAuthoring::new(
            self, request,
        )
    }

    pub(crate) fn review_existing_truth_probe_routing(
        &self,
        request: ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        let seed = self.build_existing_truth_probe_intent_seed(request);
        let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint(seed)
            .expect("existing truth probe intent seed should always author successfully");
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_existing_truth_probe_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryExistingTruthProbeExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::ExistingTruthProbeRouting(
                    plan,
                ),
            ) => Ok(ForgeQueryExistingTruthProbeExecutionHandoff::from_plan(
                plan,
            )),
            _ => Err(self.existing_truth_probe_non_admitted_error(&review)),
        }
    }

    pub(crate) fn existing_truth_probe_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let seed = review
            .request()
            .existing_truth_probe_seed()
            .expect("probe routing review must preserve probe routing seed");
        match seed.preflight() {
            ForgeQueryExistingTruthProbeRoutingPreflight::BindingDenied(denial) => {
                ForgeQueryRuntimeError::MutationBindingDenied(denial.clone())
            }
            ForgeQueryExistingTruthProbeRoutingPreflight::ProbeDenied(denial) => {
                ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial.clone())
            }
            ForgeQueryExistingTruthProbeRoutingPreflight::Admitted => {
                let violation = non_admitted_runtime_violation(review);
                ForgeQueryRuntimeError::ExistingTruthProbeDenied(
                    ForgeQueryExistingTruthProbeDenial::new(
                        seed.request().binding(),
                        ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
                        None,
                        violation.message(),
                    ),
                )
            }
        }
    }

    pub(crate) fn prepare_existing_truth_probe_execution_binding(
        &self,
        handoff: ForgeQueryExistingTruthProbeExecutionHandoff,
    ) -> ForgeQueryExistingTruthProbeExecutionBinding {
        ForgeQueryExistingTruthProbeExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_existing_truth_probe_execution_binding(
        &self,
        binding: ForgeQueryExistingTruthProbeExecutionBinding,
    ) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
        let probe = self
            .backend
            .probe_existing_truth(binding.request())
            .map_err(ForgeQueryRuntimeError::ExistingTruthProbeDenied)?;
        let receipt = ForgeQueryExistingTruthProbeReceipt::from_probe(
            binding.request(),
            &probe,
            self.backend.snapshot_token(),
        );
        let mut result = ForgeQueryExistingTruthProbeResult::new(probe, receipt);
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts(
                binding.family(),
                binding.entrypoint(),
                binding.request().binding().authoritative_identity(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                binding.request().binding().binding_digest().as_str(),
                result.receipt().probe_digest(),
                "existing-truth-probe",
            );
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_shared_execution_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            result.receipt().probe_digest(),
            result.receipt().snapshot_token(),
        );
        result.attach_intent_admission_evidence(decision_trace_envelope, execution_provenance);
        Ok(result)
    }

    pub(crate) fn build_existing_truth_probe_intent_seed(
        &self,
        request: ForgeQueryExistingTruthProbeRequest,
    ) -> ForgeQueryExistingTruthProbeIntentSeed {
        let preflight = match self.backend.admit_existing_truth_binding(request.binding()) {
            Err(denial) => ForgeQueryExistingTruthProbeRoutingPreflight::BindingDenied(denial),
            Ok(()) => match self.probe_existing_support_denial(request.binding()) {
                Some(denial) => ForgeQueryExistingTruthProbeRoutingPreflight::ProbeDenied(denial),
                None => ForgeQueryExistingTruthProbeRoutingPreflight::Admitted,
            },
        };
        ForgeQueryExistingTruthProbeIntentSeed::new(request, preflight)
    }

    fn probe_existing_support_denial(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Option<ForgeQueryExistingTruthProbeDenial> {
        let support_profile = self.backend.support_profile();
        let row = support_profile
            .bridge_backed_verification_support_rows()
            .iter()
            .find(|row| {
                row.operation_family() == "probe_existing"
                    && row.target_binding_family()
                        == binding.family().bridge_backed_support_family()
            })?;
        let supported = match support_profile.posture() {
            ForgeQueryRuntimeBackendPosture::Scaffold => row.scaffold_profile_supported(),
            ForgeQueryRuntimeBackendPosture::Primary => {
                row.primary_bridge_backed_runtime_supported()
            }
        };
        if supported {
            return None;
        }
        Some(ForgeQueryExistingTruthProbeDenial::new(
            binding,
            ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
            None,
            row.denial_class_when_primary_unsupported().unwrap_or(
                "this runtime backend does not admit backend-verified existing-truth probes yet",
            ),
        ))
    }
}
