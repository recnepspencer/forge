use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::{WorthUiFrameworkTurn, WorthUiRuntime, WorthUiRuntimeShutdownReceipt};

use super::{
    WorthUiActiveApplicationGenerationIdentity, WorthUiActiveApplicationSessionIdentity,
    WorthUiActiveFrameworkTurnCompletion, WorthUiApp,
};
#[path = "active_application_session/focus_observation.rs"]
mod focus_observation;
#[path = "active_application_session/motion_sampling.rs"]
mod motion_sampling;
#[path = "active_application_session/portal_exit_publication.rs"]
mod portal_exit_publication;
pub(in crate::facade::entry) use portal_exit_publication::UiPortalExitTerminalProgress;
pub(in crate::facade::entry) use portal_exit_retention::UiPortalExitTerminalPending;
#[path = "active_application_session/portal_exit_retention.rs"]
mod portal_exit_retention;
#[path = "active_application_session/portal_motion.rs"]
mod portal_motion;
#[path = "active_application_session/portal_observation.rs"]
mod portal_observation;
#[path = "active_application_session/runtime_access.rs"]
mod runtime_access;
#[path = "active_application_session/semantic_text_registration.rs"]
mod semantic_text_registration;
#[path = "active_application_session/service_proposal_observation.rs"]
mod service_proposal_observation;
mod shutdown;
#[path = "active_application_session/theme_values.rs"]
mod theme_values;
/// The one ordinary owner of a running Worth UI application generation.
pub struct WorthUiActiveApplicationSession {
    pub(super) identity: WorthUiActiveApplicationSessionIdentity,
    pub(super) application: crate::runtime::session::WorthUiApplicationSessionState,
    pub(super) host_session: crate::facade::WorthUiHostSessionAuthority,
    pub(super) mounted: crate::mounting::WorthUiMountedSessionState,
    pub(super) host_exchange: crate::host_exchange::WorthUiHostExchangeSessionState,
    pub(super) interaction: crate::runtime::interaction::UiInteractionRuntimeState,
    pub(super) focus: crate::runtime::focus::UiFocusRuntimeState,
    pub(super) portal: crate::runtime::portal::UiPortalRuntimeState,
    pub(super) motion: crate::runtime::motion::UiMotionRuntimeState,
    pub(super) portal_exit_retention: portal_exit_retention::UiPortalExitRetentionCoordinator,
    pub(super) intent_evidence: crate::inspection::intent::UiIntentEvidenceRegistry,
    pub(super) intent_application_facts: crate::runtime::intent::UiIntentApplicationFactState,
    pub(super) intent_execution: crate::runtime::intent_execution::UiIntentExecutionState,
    pub(super) intent_admission: crate::runtime::intent::UiIntentAdmissionState,
    pub(super) intent_confirmation: crate::runtime::intent::UiIntentConfirmationState,
    pub(super) intent_postures: crate::mounting::UiIntentPostureTable,
    pub(super) presentation: crate::runtime::presentation_state::UiApplicationPresentationState,
    pub(super) visual_inspection:
        crate::inspection::visual_snapshot::WorthUiVisualInspectionAuthority,
    pub(super) next_visual_capture_identity: u64,
    pub(super) next_visual_overlay_identity: u64,
    pub(super) next_portal_service_event_identity: u64,
    pub(super) visual_captures: crate::inspection::visual_snapshot::UiVisualCaptureRegistry,
    pub(super) visual_overlays: crate::inspection::visual_snapshot::UiVisualOverlayRegistry,
    pub(super) rebind: crate::runtime::rebind::UiRebindRuntimeState,
}

impl WorthUiActiveApplicationSession {
    pub(super) fn new(
        mut app: WorthUiApp,
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
        let presentation_async = app.take_presentation_async_owner();
        let intent_application_facts =
            crate::runtime::intent::UiIntentApplicationFactState::activate(
                app.prepared_authority().intent_application_fact_plan(),
            );
        let presentation =
            crate::runtime::presentation_state::UiApplicationPresentationState::activate(
                app.capabilities(),
            );
        let application =
            crate::runtime::session::WorthUiApplicationSessionState::new(app, runtime);
        let mounted = crate::mounting::WorthUiMountedSessionState::new(
            host_session.identity(),
            mounted_frame_retention_budget,
            presentation_async,
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
            focus: crate::runtime::focus::UiFocusRuntimeState::new_session_restore_candidate(),
            portal: crate::runtime::portal::UiPortalRuntimeState::new(
                crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate,
            ),
            motion: crate::runtime::motion::UiMotionRuntimeState::new(
                crate::runtime::UiServiceStatePersistencePosture::Ephemeral,
            ),
            portal_exit_retention: portal_exit_retention::UiPortalExitRetentionCoordinator::new(),
            intent_evidence: crate::inspection::intent::UiIntentEvidenceRegistry::new(
                identity.as_u64(),
            ),
            intent_application_facts,
            intent_execution: crate::runtime::intent_execution::UiIntentExecutionState::new(),
            intent_admission: crate::runtime::intent::UiIntentAdmissionState::new(),
            intent_confirmation: crate::runtime::intent::UiIntentConfirmationState::new(),
            intent_postures: crate::mounting::UiIntentPostureTable::new(),
            presentation,
            visual_inspection,
            next_visual_capture_identity: 1,
            next_visual_overlay_identity: 1,
            next_portal_service_event_identity: 1_u64 << 63,
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

    pub fn active_generation_identity(&self) -> WorthUiActiveApplicationGenerationIdentity {
        WorthUiActiveApplicationGenerationIdentity::current(
            self.identity,
            self.application.generation_identity(),
        )
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
        let font_collection = std::sync::Arc::clone(self.application.font_collection());
        let turn = self.application.execute_framework_turn(collect_sources);
        let (generation_identity, visual_trace_source, graph, active_plan_digest, completion) =
            turn.into_parts();
        Ok(WorthUiActiveFrameworkTurnCompletion {
            application_session_identity: self.identity,
            generation_identity,
            visual_trace_source,
            graph,
            font_collection,
            active_plan_digest,
            host_session_identity,
            completion,
            mounted: &mut self.mounted,
            host_session: &self.host_session,
            host_exchange: &mut self.host_exchange,
            focus: &mut self.focus,
            portal: &mut self.portal,
            interaction: &mut self.interaction,
            presentation: &mut self.presentation,
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
}
