use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
#[cfg(any(test, feature = "certification-support"))]
use crate::runtime::WorthUiActiveRuntimeObservation;
use crate::runtime::{WorthUiFrameworkTurn, WorthUiRuntime, WorthUiRuntimeShutdownReceipt};
use worth_ui_inspection::UiInspectionQuery;

use super::{
    WorthUiActiveApplicationSessionIdentity, WorthUiActiveFrameworkTurnCompletion, WorthUiApp,
};

/// The one ordinary owner of a running Worth UI application generation.
pub struct WorthUiActiveApplicationSession {
    pub(super) identity: WorthUiActiveApplicationSessionIdentity,
    pub(super) application: crate::runtime::session::WorthUiApplicationSessionState,
    pub(super) host_session: crate::facade::WorthUiHostSessionAuthority,
    pub(super) mounted: crate::mounting::WorthUiMountedSessionState,
    pub(super) host_exchange: crate::host_exchange::WorthUiHostExchangeSessionState,
}

/// Inspection evidence bound to the exact generation currently executing.
pub struct WorthUiActiveInspectionReceipt {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    receipt: UiInspectionReceipt,
}

impl WorthUiActiveApplicationSession {
    pub(super) fn new(
        app: WorthUiApp,
        runtime: WorthUiRuntime,
        host_session: crate::facade::WorthUiHostSessionAuthority,
    ) -> Result<Self, crate::runtime::WorthUiRuntimeLaunchDenial> {
        let identity =
            WorthUiActiveApplicationSessionIdentity::from_host_session(host_session.identity());
        let mounted_frame_retention_budget = app.mounted_frame_retention_budget();
        let host_observation_capacity = app.host_observation_capacity();
        let application =
            crate::runtime::session::WorthUiApplicationSessionState::new(app, runtime);
        let mounted = crate::mounting::WorthUiMountedSessionState::new(
            host_session.identity(),
            mounted_frame_retention_budget,
        )
        .map_err(|_| crate::runtime::WorthUiRuntimeLaunchDenial::MountedIdentityExhausted)?;
        Ok(Self {
            identity,
            application,
            host_session,
            mounted,
            host_exchange: crate::host_exchange::WorthUiHostExchangeSessionState::new(
                host_observation_capacity,
            ),
        })
    }

    pub fn session_identity(&self) -> WorthUiActiveApplicationSessionIdentity {
        self.identity
    }

    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        self.application.generation_identity()
    }

    pub fn capabilities(&self) -> &crate::facade::registry::snapshot::CapabilitySnapshot {
        self.application.capabilities()
    }

    /// Borrow the graph authority for the generation this session is
    /// currently executing.
    pub fn graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.application.graph()
    }

    pub(crate) fn source_event_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> crate::runtime::WorthUiSourceEventIngress {
        self.application.source_event_ingress(provider)
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> WorthUiActiveInspectionReceipt {
        WorthUiActiveInspectionReceipt {
            generation_identity: self.generation_identity().clone(),
            receipt: self.application.inspect(query),
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_runtime(&self) -> WorthUiActiveRuntimeObservation {
        self.application.inspect_active_runtime()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_query_state_residue(
        &self,
    ) -> crate::runtime::WorthUiStateQueryResidueScan {
        self.application.inspect_query_state_residue()
    }

    pub(crate) fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<
        WorthUiActiveFrameworkTurnCompletion<'_>,
        crate::mounting::UiMountedPublicationLeaseDenial,
    > {
        if self.mounted.has_active_presentation_attempt() {
            return Err(crate::mounting::UiMountedPublicationLeaseDenial::PresentationInFlight);
        }
        let host_session_identity = self.host_session.identity();
        let turn = self.application.execute_framework_turn(collect_sources);
        let (generation_identity, graph, active_plan_digest, completion) = turn.into_parts();
        Ok(WorthUiActiveFrameworkTurnCompletion {
            generation_identity,
            graph,
            active_plan_digest,
            host_session_identity,
            completion,
            mounted: &mut self.mounted,
            host_session: &self.host_session,
            host_exchange: &mut self.host_exchange,
        })
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn ordinary_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiOrdinaryPlanAvailability {
        self.application.ordinary_plan_availability()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn virtualized_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiVirtualizedPlanAvailability {
        self.application.virtualized_plan_availability()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn query_fact_link(
        &self,
        binding_id: &str,
    ) -> Option<crate::runtime::WorthUiQueryLaneFactLink> {
        self.application.query_fact_link(binding_id)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn canvas_spatial_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiCanvasSpatialPlanAvailability {
        self.application.canvas_spatial_plan_availability()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn first_canvas_spatial_handle(&self) -> Option<crate::runtime::WorthUiLaneHandle> {
        self.application.first_canvas_spatial_handle()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_canvas_spatial_target(
        &self,
        handle: crate::runtime::WorthUiLaneHandle,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialTargetSummary,
        crate::runtime::WorthUiCanvasSpatialInspectionDenial,
    > {
        self.application.inspect_canvas_spatial_target(handle)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn realtime_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiRealtimePlanAvailability {
        self.application.realtime_plan_availability()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn first_realtime_renderer_surface(
        &self,
    ) -> Option<crate::runtime::WorthUiRendererSurfaceHandle> {
        self.application.first_realtime_renderer_surface()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_realtime_target(
        &self,
        handle: crate::runtime::WorthUiRendererSurfaceHandle,
    ) -> Result<
        crate::runtime::WorthUiRealtimeTargetSummary,
        crate::runtime::WorthUiRealtimeInspectionDenial,
    > {
        self.application.inspect_realtime_target(handle)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_virtualized_plan(
        &self,
        request: crate::runtime::WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedPlanSummary,
        crate::runtime::WorthUiVirtualizedPlanSummaryDenial,
    > {
        self.application.inspect_virtualized_plan(request)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_ordinary_plan(
        &self,
        request: crate::runtime::WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiOrdinaryPlanSummary,
        crate::runtime::WorthUiOrdinaryPlanSummaryDenial,
    > {
        self.application.inspect_ordinary_plan(request)
    }

    pub fn host_session_identity(&self) -> crate::facade::WorthUiHostSessionIdentity {
        self.host_session.identity()
    }

    pub(crate) fn host_measurement_capability(
        &self,
    ) -> crate::facade::WorthUiHostMeasurementCapability {
        self.host_session.measurement_capability()
    }

    pub fn shutdown(mut self) -> WorthUiRuntimeShutdownReceipt {
        let (mounted_presentation, outcomes) =
            self.mounted.shutdown_presentation(&self.host_session);
        for outcome in outcomes {
            let _ = self.finish_mounted_presentation(outcome);
        }
        self.mounted.assert_shutdown_resolved();
        self.host_exchange.shutdown();
        let host_session_release = self.host_session.release_adapter_session();
        self.application
            .shutdown()
            .bind_mounted_presentation(mounted_presentation)
            .bind_host_session_release(host_session_release)
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
