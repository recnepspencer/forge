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
    pub(super) interaction: crate::runtime::interaction::UiInteractionRuntimeState,
    pub(super) intent_application_facts: crate::runtime::intent::UiIntentApplicationFactState,
    pub(super) visual_inspection:
        crate::inspection::visual_snapshot::WorthUiVisualInspectionAuthority,
    pub(super) next_visual_capture_identity: u64,
    pub(super) next_visual_overlay_identity: u64,
    pub(super) visual_captures: crate::inspection::visual_snapshot::UiVisualCaptureRegistry,
    pub(super) visual_overlays: crate::inspection::visual_snapshot::UiVisualOverlayRegistry,
    pub(super) rebind: crate::runtime::rebind::UiRebindRuntimeState,
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
        let identity = WorthUiActiveApplicationSessionIdentity::from_host_session_value(
            host_session.identity().as_u64(),
        );
        let mounted_frame_retention_budget = app.mounted_frame_retention_budget();
        let host_observation_capacity = app.host_observation_capacity();
        let visual_policy = app.visual_inspection_policy();
        let rebind_profile = app.prepared_authority().change_profile().rebind();
        let intent_application_facts =
            crate::runtime::intent::UiIntentApplicationFactState::activate(
                app.prepared_authority().intent_application_fact_plan(),
                app.generation_identity().clone(),
            );
        let application =
            crate::runtime::session::WorthUiApplicationSessionState::new(app, runtime);
        let mounted = crate::mounting::WorthUiMountedSessionState::new(
            host_session.identity(),
            mounted_frame_retention_budget,
        )
        .map_err(|_| crate::runtime::WorthUiRuntimeLaunchDenial::MountedIdentityExhausted)?;
        let visual_inspection =
            crate::inspection::visual_snapshot::WorthUiVisualInspectionAuthority::seal(
                identity,
                visual_policy,
            );
        Ok(Self {
            identity,
            application,
            host_session,
            mounted,
            host_exchange: crate::host_exchange::WorthUiHostExchangeSessionState::new(
                host_observation_capacity,
            ),
            interaction: crate::runtime::interaction::UiInteractionRuntimeState::new(),
            intent_application_facts,
            visual_inspection,
            next_visual_capture_identity: 1,
            next_visual_overlay_identity: 1,
            visual_captures: crate::inspection::visual_snapshot::UiVisualCaptureRegistry::new(
                visual_policy,
            ),
            visual_overlays: crate::inspection::visual_snapshot::UiVisualOverlayRegistry::new(),
            rebind: crate::runtime::rebind::UiRebindRuntimeState::new(rebind_profile),
        })
    }

    pub fn session_identity(&self) -> WorthUiActiveApplicationSessionIdentity {
        self.identity
    }

    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        self.application.generation_identity()
    }

    pub const fn rebind_deadline_at(
        &self,
        tick: u64,
    ) -> crate::runtime::rebind::UiRebindSessionDeadline {
        crate::runtime::rebind::UiRebindSessionDeadline::new(self.identity, tick)
    }

    pub const fn rebind_cancellation_request(
        &self,
    ) -> crate::runtime::rebind::UiRebindCancellationRequest {
        crate::runtime::rebind::UiRebindCancellationRequest::new(self.identity)
    }

    pub fn capabilities(&self) -> &crate::facade::registry::snapshot::CapabilitySnapshot {
        self.application.capabilities()
    }

    pub fn classify_observations(
        &self,
        observations: crate::facade::observation::UiAdmittedObservationSet,
    ) -> Result<
        crate::facade::observation::UiChangeClassificationOutcome,
        crate::facade::observation::UiChangeClassificationDenial,
    > {
        self.application
            .classify_observations(self.identity, observations)
    }

    pub fn resolve_affected_scope(
        &self,
        change: crate::facade::observation::UiClassifiedChange,
    ) -> Result<
        crate::runtime::rebind::UiResolvedAffectedScope,
        crate::runtime::rebind::UiAffectedScopeDenial,
    > {
        self.application
            .resolve_affected_scope(self.identity, change)
    }

    pub fn compile_rebind_plan(
        &self,
        lifecycle: crate::runtime::rebind::UiResolvedIdentityLifecycle,
        policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    ) -> Result<crate::runtime::rebind::UiRebindPlan, crate::runtime::rebind::UiRebindPlanningDenial>
    {
        self.application
            .compile_rebind_plan(self.identity, lifecycle, policy)
    }

    pub fn compile_preservation_rebind(
        &self,
        evidence: crate::facade::observation::UiEvidenceOnlySourceChange,
        policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    ) -> Result<crate::runtime::rebind::UiRebindPlan, crate::runtime::rebind::UiRebindPlanningDenial>
    {
        self.application
            .compile_preservation_rebind(self.identity, evidence, policy)
    }

    /// Borrow the graph authority for the generation this session is
    /// currently executing.
    pub fn graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.application.graph()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn lookup_consumed_fact_for_certification(
        &self,
        fact: &crate::fact_contract::UiProducedFact,
    ) -> Result<crate::graph::UiGraphFactLookupReceipt, crate::graph::UiGraphFactLookupDenial> {
        self.application.lookup_consumed_fact(fact)
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

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn refresh_query_change_for_certification(
        &mut self,
        request: worth_ui_query_binding::WorthUiOperationLiveRefreshRequest<'_>,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome,
        worth_ui_query_binding::WorthUiOperationLiveRefreshError,
    > {
        self.application
            .refresh_query_change_for_certification(request)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn measurement_basis_sources_for_certification(
        &self,
    ) -> Box<[crate::declaration::UiDeclaredMeasurementBasisSource]> {
        self.application
            .measurement_basis_sources_for_certification()
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
        let (generation_identity, visual_trace_source, graph, active_plan_digest, completion) =
            turn.into_parts();
        Ok(WorthUiActiveFrameworkTurnCompletion {
            generation_identity,
            visual_trace_source,
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
        let rebind = self.rebind.shutdown();
        let visual_capture = self.visual_captures.shutdown();
        let visual_overlay = self.visual_overlays.shutdown();
        let interaction = self.interaction.shutdown();
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
            .bind_visual_capture(visual_capture)
            .bind_visual_overlay(visual_overlay)
            .bind_mounted_presentation(mounted_presentation)
            .bind_host_session_release(host_session_release)
            .bind_interaction(interaction)
            .bind_rebind(rebind)
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
