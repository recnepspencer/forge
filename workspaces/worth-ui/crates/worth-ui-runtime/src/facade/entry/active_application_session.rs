use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::facade::prepared_application_authority::{
    WorthUiHostSessionPlan, WorthUiPreparedApplicationGenerationIdentity,
};
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiFrameworkTurn, WorthUiFrameworkTurnCompletion,
    WorthUiRuntime, WorthUiRuntimeShutdownReceipt,
};
use worth_ui_inspection::UiInspectionQuery;

use super::{WorthUiActiveApplicationSessionIdentity, WorthUiApp};

/// The one ordinary owner of a running Worth UI application generation.
pub struct WorthUiActiveApplicationSession {
    pub(super) identity: WorthUiActiveApplicationSessionIdentity,
    pub(super) app: WorthUiApp,
    pub(super) runtime: WorthUiRuntime,
    pub(super) host_session: crate::facade::WorthUiHostSessionAuthority,
}

/// Inspection evidence bound to the exact generation currently executing.
pub struct WorthUiActiveInspectionReceipt {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    receipt: UiInspectionReceipt,
}

/// One framework-turn result bound to the active application generation.
pub struct WorthUiActiveFrameworkTurnCompletion<'session> {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    completion: WorthUiFrameworkTurnCompletion<'session>,
}

impl WorthUiActiveApplicationSession {
    pub(super) fn new(
        app: WorthUiApp,
        mut runtime: WorthUiRuntime,
        host_session_plan: WorthUiHostSessionPlan,
    ) -> Self {
        let host_session = crate::facade::WorthUiHostSessionAuthority::activate(&host_session_plan);
        runtime.bind_operational_host_session(
            host_session.identity(),
            host_session.observation_generation(),
        );
        let identity =
            WorthUiActiveApplicationSessionIdentity::from_host_session(host_session.identity());
        Self {
            identity,
            app,
            runtime,
            host_session,
        }
    }

    pub fn session_identity(&self) -> WorthUiActiveApplicationSessionIdentity {
        self.identity
    }

    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        self.app.generation_identity()
    }

    pub fn capabilities(&self) -> &crate::facade::registry::CapabilitySnapshot {
        self.app.capabilities()
    }

    /// Borrow the graph authority for the generation this session is
    /// currently executing.
    pub fn graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.app.graph()
    }

    pub fn source_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> crate::runtime::WorthUiSourceWatcher {
        self.runtime.source_ingress(provider)
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> WorthUiActiveInspectionReceipt {
        WorthUiActiveInspectionReceipt {
            generation_identity: self.generation_identity().clone(),
            receipt: self.app.inspect(query),
        }
    }

    pub fn inspect_runtime(&self) -> WorthUiActiveRuntimeObservation {
        self.runtime.inspect_active()
    }

    pub fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> WorthUiActiveFrameworkTurnCompletion<'_> {
        let generation_identity = self.generation_identity().clone();
        let completion = self.runtime.execute_framework_turn(collect_sources);
        WorthUiActiveFrameworkTurnCompletion {
            generation_identity,
            completion,
        }
    }

    pub fn host_session_identity(&self) -> crate::facade::WorthUiHostSessionIdentity {
        self.host_session.identity()
    }

    pub fn host_measurement_capability(&self) -> crate::facade::WorthUiHostMeasurementCapability {
        self.host_session.measurement_capability()
    }

    pub fn shutdown(self) -> WorthUiRuntimeShutdownReceipt {
        self.runtime.shutdown()
    }

    #[cfg(test)]
    pub(crate) fn replace_host_observation_generation_for_test(
        &mut self,
        observation_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    ) {
        self.runtime
            .replace_host_observation_generation_for_test(observation_generation);
    }

    #[cfg(test)]
    pub(crate) fn allocation_ingress_count_for_test(&self) -> u64 {
        self.runtime
            .allocation_frame_dispatcher_counters()
            .ingress_count()
    }

    #[cfg(test)]
    pub(crate) fn planning_inspection_authority_identity_for_test(&self) -> usize {
        std::rc::Rc::as_ptr(self.app.retained_planning_authority()) as usize
    }

    #[cfg(test)]
    pub(crate) fn planning_inspection_authority_is_runtime_coherent_for_test(&self) -> bool {
        std::rc::Rc::ptr_eq(
            self.app.retained_planning_authority(),
            &self.runtime.retained_allocation_planning_evidence,
        )
    }
}

impl WorthUiActiveInspectionReceipt {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn receipt(&self) -> &UiInspectionReceipt {
        &self.receipt
    }
}

impl<'session> WorthUiActiveFrameworkTurnCompletion<'session> {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn into_completion(self) -> WorthUiFrameworkTurnCompletion<'session> {
        self.completion
    }
}
