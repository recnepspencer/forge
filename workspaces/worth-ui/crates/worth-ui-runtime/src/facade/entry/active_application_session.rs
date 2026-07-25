use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiFrameworkTurn, WorthUiRuntime,
    WorthUiRuntimeShutdownReceipt,
};
use worth_ui_inspection::UiInspectionQuery;

use super::{
    WorthUiActiveApplicationSessionIdentity, WorthUiActiveFrameworkTurnCompletion, WorthUiApp,
};

/// The one ordinary owner of a running Worth UI application generation.
pub struct WorthUiActiveApplicationSession {
    pub(super) identity: WorthUiActiveApplicationSessionIdentity,
    pub(super) app: WorthUiApp,
    pub(super) runtime: WorthUiRuntime,
    pub(super) host_session: crate::facade::WorthUiHostSessionAuthority,
    pub(super) mounted_identity: crate::mounting::UiMountedIdentityState,
    pub(super) mounted_retention: crate::mounting::UiMountedFrameRetentionCoordinator,
    pub(super) mounted_presentation: crate::mounting::UiMountedPresentationCoordinator,
    pub(super) mounted_publication_reservations: std::collections::BTreeMap<
        worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        crate::mounting::UiMountedFramePublicationCandidate,
    >,
    pub(super) mounted_reconciliation_reservations: std::collections::BTreeMap<
        worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        crate::mounting::UiMountedFrameReconciliationCandidate,
    >,
    pub(super) host_observations:
        crate::host_exchange::observation_report_validation::UiHostObservationReportValidation,
    pub(super) host_measurements:
        crate::host_exchange::measurement_admission::UiHostMeasurementAdmission,
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
        let mounted_identity = crate::mounting::UiMountedIdentityState::new(
            host_session.identity(),
        )
        .map_err(|_| crate::runtime::WorthUiRuntimeLaunchDenial::MountedIdentityExhausted)?;
        Ok(Self {
            identity,
            app,
            runtime,
            host_session,
            mounted_identity,
            mounted_retention: crate::mounting::UiMountedFrameRetentionCoordinator::with_budget(
                mounted_frame_retention_budget,
            ),
            mounted_presentation: crate::mounting::UiMountedPresentationCoordinator::default(),
            mounted_publication_reservations: std::collections::BTreeMap::new(),
            mounted_reconciliation_reservations: std::collections::BTreeMap::new(),
            host_observations: Default::default(),
            host_measurements: Default::default(),
        })
    }

    pub fn session_identity(&self) -> WorthUiActiveApplicationSessionIdentity {
        self.identity
    }

    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        self.app.generation_identity()
    }

    pub fn capabilities(&self) -> &crate::facade::registry::snapshot::CapabilitySnapshot {
        self.app.capabilities()
    }

    /// Borrow the graph authority for the generation this session is
    /// currently executing.
    pub fn graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.app.graph()
    }

    pub fn source_event_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> crate::runtime::WorthUiSourceEventIngress {
        self.runtime.source_event_ingress(provider)
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

    pub fn inspect_query_state_residue(&self) -> crate::runtime::WorthUiStateQueryResidueScan {
        self.runtime.inspect_query_state_residue()
    }

    pub fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> Result<
        WorthUiActiveFrameworkTurnCompletion<'_>,
        crate::mounting::UiMountedPublicationLeaseDenial,
    > {
        if self.mounted_presentation.has_active_attempt() {
            return Err(crate::mounting::UiMountedPublicationLeaseDenial::PresentationInFlight);
        }
        let generation_identity = self.generation_identity().clone();
        let graph = self.app.graph();
        let active_plan_digest = self.runtime.active.active_plan_ref().digest().as_u64();
        let host_session_identity = self.host_session.identity();
        let completion = self.runtime.execute_framework_turn(collect_sources);
        Ok(WorthUiActiveFrameworkTurnCompletion {
            generation_identity,
            graph,
            active_plan_digest,
            host_session_identity,
            completion,
            mounted_identity: &mut self.mounted_identity,
            mounted_retention: &mut self.mounted_retention,
            host_session: &self.host_session,
            mounted_presentation: &mut self.mounted_presentation,
            mounted_publication_reservations: &mut self.mounted_publication_reservations,
            host_observations: &mut self.host_observations,
        })
    }

    pub fn ordinary_plan_availability(&self) -> crate::runtime::WorthUiOrdinaryPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .ordinary_availability()
    }

    pub fn virtualized_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiVirtualizedPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .virtualized_availability()
    }

    pub fn query_fact_link(
        &self,
        binding_id: &str,
    ) -> Option<crate::runtime::WorthUiQueryLaneFactLink> {
        let binding_id = crate::capability::ViewBindingId::new(binding_id).ok()?;
        self.runtime
            .active
            .active_plan_ref()
            .query_fact_link_for_binding_id(&binding_id)
    }

    pub fn canvas_spatial_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiCanvasSpatialPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .canvas_spatial_availability()
    }

    pub fn first_canvas_spatial_handle(&self) -> Option<crate::runtime::WorthUiLaneHandle> {
        self.runtime
            .active
            .active_plan_ref()
            .first_canvas_spatial_handle()
    }

    pub fn inspect_canvas_spatial_target(
        &self,
        handle: crate::runtime::WorthUiLaneHandle,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialTargetSummary,
        crate::runtime::WorthUiCanvasSpatialInspectionDenial,
    > {
        self.runtime
            .active
            .active_plan_ref()
            .canvas_spatial_summary(handle)
    }

    pub fn realtime_plan_availability(&self) -> crate::runtime::WorthUiRealtimePlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .realtime_availability()
    }

    pub fn first_realtime_renderer_surface(
        &self,
    ) -> Option<crate::runtime::WorthUiRendererSurfaceHandle> {
        self.runtime
            .active
            .active_plan_ref()
            .first_realtime_handle()
    }

    pub fn inspect_realtime_target(
        &self,
        handle: crate::runtime::WorthUiRendererSurfaceHandle,
    ) -> Result<
        crate::runtime::WorthUiRealtimeTargetSummary,
        crate::runtime::WorthUiRealtimeInspectionDenial,
    > {
        self.runtime
            .active
            .active_plan_ref()
            .realtime_summary(handle)
    }

    pub fn inspect_virtualized_plan(
        &self,
        request: crate::runtime::WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedPlanSummary,
        crate::runtime::WorthUiVirtualizedPlanSummaryDenial,
    > {
        self.runtime
            .active
            .active_plan_ref()
            .virtualized_summary(&self.runtime.query_binding, request)
    }

    pub fn inspect_ordinary_plan(
        &self,
        request: crate::runtime::WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiOrdinaryPlanSummary,
        crate::runtime::WorthUiOrdinaryPlanSummaryDenial,
    > {
        self.runtime
            .active
            .active_plan_ref()
            .ordinary_summary(request)
    }

    pub fn host_session_identity(&self) -> crate::facade::WorthUiHostSessionIdentity {
        self.host_session.identity()
    }

    pub fn host_measurement_capability(&self) -> crate::facade::WorthUiHostMeasurementCapability {
        self.host_session.measurement_capability()
    }

    pub fn shutdown(mut self) -> WorthUiRuntimeShutdownReceipt {
        let (mounted_presentation, outcomes) = self
            .mounted_presentation
            .shutdown(self.host_session.effect_port());
        for outcome in outcomes {
            let _ = self.finish_mounted_presentation(outcome);
        }
        assert!(
            self.mounted_publication_reservations.is_empty(),
            "shutdown resolves every retained mounted publication reservation"
        );
        assert!(
            self.mounted_reconciliation_reservations.is_empty(),
            "shutdown resolves every retained mounted reconciliation reservation"
        );
        self.host_observations.shutdown();
        self.host_measurements.shutdown();
        let host_session_release = self.host_session.release_adapter_session();
        self.runtime
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
