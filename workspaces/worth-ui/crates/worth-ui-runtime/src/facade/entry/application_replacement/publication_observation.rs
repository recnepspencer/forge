use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

/// Exact post-commit observation carried by a successful application cutover.
pub struct WorthUiApplicationPublicationObservation {
    application_generation: WorthUiPreparedApplicationGenerationIdentity,
    runtime: crate::runtime::WorthUiActiveRuntimeObservation,
    host_session: crate::facade::WorthUiHostSessionIdentity,
    runtime_basis: crate::runtime::session::WorthUiRuntimePublicationBasis,
    scheduler: crate::runtime::UiAllocationFrameDispatcherState,
}

pub(super) struct WorthUiApplicationPublicationPreparation {
    pub(super) application_generation: WorthUiPreparedApplicationGenerationIdentity,
    pub(super) successor_runtime: crate::runtime::WorthUiActiveRuntimeObservation,
    pub(super) runtime_basis: crate::runtime::session::WorthUiRuntimePublicationBasis,
    pub(super) host_session: crate::facade::WorthUiHostSessionIdentity,
    pub(super) successor_scheduler: crate::runtime::UiAllocationFrameDispatcherState,
}

impl WorthUiApplicationPublicationObservation {
    pub(super) fn prepare_successor(preparation: WorthUiApplicationPublicationPreparation) -> Self {
        let WorthUiApplicationPublicationPreparation {
            application_generation,
            successor_runtime,
            runtime_basis,
            host_session,
            successor_scheduler,
        } = preparation;
        Self {
            application_generation,
            runtime: successor_runtime,
            host_session,
            runtime_basis,
            scheduler: successor_scheduler,
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
        self.runtime_basis.is_coherent_with(self.host_session)
    }
}
