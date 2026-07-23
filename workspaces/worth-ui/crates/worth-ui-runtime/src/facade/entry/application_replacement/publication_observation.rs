use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

/// Exact post-commit observation carried by a successful application cutover.
pub struct WorthUiApplicationPublicationObservation {
    application_generation: WorthUiPreparedApplicationGenerationIdentity,
    runtime: crate::runtime::WorthUiActiveRuntimeObservation,
    host_session: crate::facade::WorthUiHostSessionIdentity,
    runtime_host_session: Option<crate::facade::WorthUiHostSessionIdentity>,
    plan_host_session: crate::facade::WorthUiHostSessionIdentity,
    runtime_host_observation:
        Option<worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration>,
    plan_host_observation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    scheduler: crate::runtime::UiAllocationFrameDispatcherState,
}

impl WorthUiApplicationPublicationObservation {
    pub(super) fn capture(
        application: &super::WorthUiApp,
        runtime: &crate::runtime::WorthUiRuntime,
        host: &crate::facade::WorthUiHostSessionAuthority,
    ) -> Self {
        Self {
            application_generation: application.generation_identity().clone(),
            runtime: runtime.inspect_active(),
            host_session: host.identity(),
            runtime_host_session: runtime.host_session_identity,
            plan_host_session: runtime.host_plan_binding.session_identity(),
            runtime_host_observation: runtime.host_observation_generation,
            plan_host_observation: runtime.host_plan_binding.observation_generation(),
            scheduler: runtime.allocation_frame_dispatcher_state(),
        }
    }

    pub fn application_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.application_generation
    }

    pub fn runtime(&self) -> &crate::runtime::WorthUiActiveRuntimeObservation {
        &self.runtime
    }

    pub fn scheduler(&self) -> crate::runtime::UiAllocationFrameDispatcherState {
        self.scheduler
    }

    pub fn generation_is_coherent(&self) -> bool {
        self.application_generation == *self.runtime.generation_identity()
    }

    pub fn host_is_coherent(&self) -> bool {
        self.runtime_host_session == Some(self.host_session)
            && self.plan_host_session == self.host_session
            && self.runtime_host_observation == Some(self.plan_host_observation)
    }
}
