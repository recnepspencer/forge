use super::WorthUiApplicationSessionState;
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority;

#[derive(Clone, Copy)]
pub(crate) struct WorthUiRuntimePublicationBasis {
    runtime_host_session: Option<crate::facade::WorthUiHostSessionIdentity>,
    plan_host_session: crate::facade::WorthUiHostSessionIdentity,
    runtime_host_observation:
        Option<worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration>,
    plan_host_observation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
}

impl WorthUiApplicationSessionState {
    pub(crate) fn prepared_authority(&self) -> &WorthUiPreparedApplicationAuthority {
        self.app.prepared_authority()
    }

    pub(crate) fn replacement_admission_basis(
        &self,
    ) -> crate::runtime::replacement::admission::WorthUiActiveReplacementBasis {
        self.runtime.replacement_admission_basis()
    }

    pub(crate) fn prepare_application_replacement_lowering(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        candidate_query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<
        crate::runtime::WorthUiReplacementLoweringReady,
        crate::runtime::replacement::WorthUiReplacementLoweringDenial,
    > {
        self.runtime.prepare_application_replacement_lowering(
            admitted,
            candidate_application_authority,
            candidate_query_binding,
        )
    }

    pub(crate) fn stage_replacement_activation_from_lowering(
        &self,
        lowering: crate::runtime::WorthUiReplacementLoweringReady,
    ) -> Result<
        crate::runtime::WorthUiPendingActivation,
        crate::runtime::WorthUiActivationStagingDenial,
    > {
        self.runtime
            .stage_replacement_activation_from_lowering(lowering)
    }

    pub(crate) fn prepare_admitted_allocation_catalog_delta(
        &mut self,
        pending_activation: crate::runtime::WorthUiPendingActivation,
        input: crate::runtime::UiAllocationCatalogDeltaActivationInput,
    ) -> Result<
        crate::runtime::WorthUiPreparedQueryAwarePlanOutcome,
        crate::runtime::WorthUiAllocationCatalogActivationDenial,
    > {
        self.runtime
            .prepare_admitted_allocation_catalog_delta_with_query_binding(pending_activation, input)
    }

    pub(crate) fn commit_application_activation(
        &mut self,
        activation: crate::runtime::WorthUiPreparedApplicationPlanSwap,
    ) -> crate::runtime::WorthUiApplicationPlanSwap {
        activation.commit_once(&mut self.runtime, &mut self.app)
    }

    pub(crate) fn runtime_publication_basis(&self) -> WorthUiRuntimePublicationBasis {
        WorthUiRuntimePublicationBasis {
            runtime_host_session: self.runtime.host_session_identity,
            plan_host_session: self.runtime.host_plan_binding.session_identity(),
            runtime_host_observation: self.runtime.host_observation_generation,
            plan_host_observation: self.runtime.host_plan_binding.observation_generation(),
        }
    }

    pub(crate) fn frame_epoch(&self) -> crate::runtime::WorthUiRuntimeFrameEpoch {
        self.runtime.frame_epoch()
    }

    #[cfg(test)]
    pub(crate) fn traversal_frame_boundary_for_test(&self) -> crate::runtime::WorthUiFrameBoundary {
        self.runtime.traversal_frame_boundary_for_test()
    }

    #[cfg(test)]
    pub(crate) fn safe_frame_boundary_for_test(&self) -> crate::runtime::WorthUiFrameBoundary {
        self.runtime.safe_frame_boundary()
    }
}

impl WorthUiRuntimePublicationBasis {
    pub(crate) fn is_coherent_with(
        self,
        host_session: crate::facade::WorthUiHostSessionIdentity,
    ) -> bool {
        self.runtime_host_session == Some(host_session)
            && self.plan_host_session == host_session
            && self.runtime_host_observation == Some(self.plan_host_observation)
    }
}
