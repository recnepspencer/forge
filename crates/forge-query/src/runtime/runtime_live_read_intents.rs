use super::materialized_fact_posture::materialized_fact_posture_from_live_subscription_state;
use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryLiveReadExecutionBinding,
    ForgeQueryLiveReadExecutionHandoff, ForgeQueryLiveReadIntentSeed,
};

impl ForgeQueryWorkspace {
    pub fn read_live_intent<T>(
        &mut self,
        live_view: &ForgeQueryLiveView<T>,
    ) -> crate::intent_admission::ForgeQueryWorkspaceLiveReadIntentAuthoring<'_, T> {
        crate::intent_admission::ForgeQueryWorkspaceLiveReadIntentAuthoring::new(
            self,
            live_view.clone(),
        )
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

    pub(crate) fn into_runtime_live_read_execution_binding(
        &self,
        handoff: ForgeQueryLiveReadExecutionHandoff,
    ) -> Result<ForgeQueryLiveReadExecutionBinding, ForgeQueryRuntimeError> {
        self.runtime.prepare_live_read_execution_binding(handoff)
    }

    pub(crate) fn execute_bound_live_read_execution(
        &mut self,
        binding: ForgeQueryLiveReadExecutionBinding,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.runtime.execute_live_read_execution_binding(binding)
    }

    pub(crate) fn live_read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime.live_read_execution_non_admitted_error(review)
    }
}

impl ForgeQueryRuntime {
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
    ) -> Result<ForgeQueryLiveReadExecutionBinding, ForgeQueryRuntimeError> {
        let graph_obligation_dispatch = self.live_read_obligation_dispatch(&handoff)?;
        let live_graph_read_access_plan =
            self.plan_live_graph_read_access_for_installation(handoff.installation())?;
        Ok(ForgeQueryLiveReadExecutionBinding::from_handoff(
            handoff,
            graph_obligation_dispatch,
            live_graph_read_access_plan,
        ))
    }

    pub(crate) fn execute_live_read_execution_binding(
        &mut self,
        binding: ForgeQueryLiveReadExecutionBinding,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let rows = self
            .backend
            .live_entities(binding.installation().view_name());
        let snapshot_identity = self.current_snapshot_identity();
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let materialized_fact_posture = self.materialized_fact_posture_for_live_read(
            binding.installation().view_name(),
            &snapshot_evidence_identity,
        );
        let receipt = ForgeQueryLiveReadReceipt::from_rows(
            binding.installation(),
            snapshot_identity,
            materialized_fact_posture,
            &rows,
        );
        let mut result = ForgeQueryLiveReadResult::new(rows, receipt);
        let live_graph_access_counters =
            ForgeQueryLiveGraphReadMaintenanceCounters::observed_live_read(
                binding
                    .live_graph_read_access_plan()
                    .mutation_delta_scope()
                    .affected_requirement_row_count(),
                usize::from(!result.rows().is_empty()),
            );
        let live_graph_access_receipt =
            ForgeQueryLiveGraphReadAccessReceipt::from_plan_and_counters(
                binding.live_graph_read_access_plan(),
                live_graph_access_counters,
            );
        result.attach_live_graph_read_access(live_graph_access_receipt);
        result.attach_graph_obligation_dispatch(binding.graph_obligation_dispatch().cloned());
        let obligation_dispatch_envelope_digest = binding
            .graph_obligation_dispatch()
            .and_then(|dispatch| dispatch.envelope_digest());
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts_with_obligation_dispatch(
                binding.family(),
                binding.entrypoint(),
                binding.installation().view_name(),
                binding.handoff().request_digest(),
                binding.handoff().eligibility_trace().clone(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.execution_seam(),
                obligation_dispatch_envelope_digest,
                binding.installation().view_name(),
                result.receipt().result_digest(),
                "live-view-read",
            );
        let snapshot_evidence_identity = self.current_snapshot_identity().evidence_identity();
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

    fn materialized_fact_posture_for_live_read(
        &self,
        view_name: &str,
        basis_identity: &crate::ForgeQueryEvidenceIdentity,
    ) -> Option<crate::projection_consumption::ProjectionMaterializedFactPosture> {
        let state = self.live_subscriptions.get(view_name)?;
        Some(materialized_fact_posture_from_live_subscription_state(
            state,
            basis_identity,
        ))
    }

    fn plan_live_graph_read_access_for_installation(
        &self,
        installation: &ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Result<ForgeQueryLiveGraphReadAccessPlan, ForgeQueryRuntimeError> {
        ForgeQueryLiveGraphReadAccessPlan::from_live_installation(
            installation,
            ForgeQueryLiveGraphReadMaintenanceBudget::bounded_with_snapshot_refresh(),
        )
        .map_err(|denial| {
            ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
                ForgeQueryReadDenialKind::BasisPreflightDenied,
                denial.message(),
            ))
        })
    }
}
