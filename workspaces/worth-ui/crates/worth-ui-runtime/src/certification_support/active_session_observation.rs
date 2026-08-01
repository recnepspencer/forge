use crate::facade::{WorthUiActiveApplicationSession, WorthUiHostMeasurementCapability};
use crate::fact_contract::UiProducedFact;
use crate::graph::{UiGraphFactLookupDenial, UiGraphFactLookupReceipt};
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiCanvasSpatialInspectionDenial,
    WorthUiCanvasSpatialPlanAvailability, WorthUiCanvasSpatialTargetSummary, WorthUiLaneHandle,
    WorthUiOrdinaryPlanAvailability, WorthUiOrdinaryPlanSummary, WorthUiOrdinaryPlanSummaryDenial,
    WorthUiOrdinaryPlanSummaryRequest, WorthUiQueryLaneFactLink, WorthUiRealtimeInspectionDenial,
    WorthUiRealtimePlanAvailability, WorthUiRealtimeTargetSummary, WorthUiRendererSurfaceHandle,
    WorthUiStateQueryResidueScan, WorthUiVirtualizedPlanAvailability,
    WorthUiVirtualizedPlanSummary, WorthUiVirtualizedPlanSummaryDenial,
    WorthUiVirtualizedPlanSummaryRequest,
};

/// Certification-only observation of raw runtime plans and host capability authority.
pub trait WorthUiActiveSessionCertificationExt {
    fn inspect_runtime(&self) -> WorthUiActiveRuntimeObservation;

    fn inspect_query_state_residue(&self) -> WorthUiStateQueryResidueScan;

    fn refresh_query_change(
        &mut self,
        request: worth_ui_query_binding::WorthUiOperationLiveRefreshRequest<'_>,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome,
        worth_ui_query_binding::WorthUiOperationLiveRefreshError,
    >;

    fn query_change_state(
        &self,
        reference: &worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveChangeObservation,
        worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial,
    >;

    fn measurement_basis_sources(
        &self,
    ) -> Box<[crate::declaration::UiDeclaredMeasurementBasisSource]>;

    fn ordinary_plan_availability(&self) -> WorthUiOrdinaryPlanAvailability;

    fn virtualized_plan_availability(&self) -> WorthUiVirtualizedPlanAvailability;

    fn query_fact_link(&self, binding_id: &str) -> Option<WorthUiQueryLaneFactLink>;

    fn canvas_spatial_plan_availability(&self) -> WorthUiCanvasSpatialPlanAvailability;

    fn first_canvas_spatial_handle(&self) -> Option<WorthUiLaneHandle>;

    fn inspect_canvas_spatial_target(
        &self,
        handle: WorthUiLaneHandle,
    ) -> Result<WorthUiCanvasSpatialTargetSummary, WorthUiCanvasSpatialInspectionDenial>;

    fn realtime_plan_availability(&self) -> WorthUiRealtimePlanAvailability;

    fn first_realtime_renderer_surface(&self) -> Option<WorthUiRendererSurfaceHandle>;

    fn inspect_realtime_target(
        &self,
        handle: WorthUiRendererSurfaceHandle,
    ) -> Result<WorthUiRealtimeTargetSummary, WorthUiRealtimeInspectionDenial>;

    fn inspect_virtualized_plan(
        &self,
        request: WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<WorthUiVirtualizedPlanSummary, WorthUiVirtualizedPlanSummaryDenial>;

    fn inspect_ordinary_plan(
        &self,
        request: WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<WorthUiOrdinaryPlanSummary, WorthUiOrdinaryPlanSummaryDenial>;

    fn host_measurement_capability(&self) -> WorthUiHostMeasurementCapability;

    fn lookup_consumed_fact(
        &self,
        fact: &UiProducedFact,
    ) -> Result<UiGraphFactLookupReceipt, UiGraphFactLookupDenial>;
}

impl WorthUiActiveSessionCertificationExt for WorthUiActiveApplicationSession {
    fn inspect_runtime(&self) -> WorthUiActiveRuntimeObservation {
        WorthUiActiveApplicationSession::inspect_runtime(self)
    }

    fn inspect_query_state_residue(&self) -> WorthUiStateQueryResidueScan {
        WorthUiActiveApplicationSession::inspect_query_state_residue(self)
    }

    fn refresh_query_change(
        &mut self,
        request: worth_ui_query_binding::WorthUiOperationLiveRefreshRequest<'_>,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome,
        worth_ui_query_binding::WorthUiOperationLiveRefreshError,
    > {
        WorthUiActiveApplicationSession::refresh_query_change_for_certification(self, request)
    }

    fn query_change_state(
        &self,
        reference: &worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveChangeObservation,
        worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial,
    > {
        WorthUiActiveApplicationSession::query_change_state_for_certification(self, reference)
    }

    fn measurement_basis_sources(
        &self,
    ) -> Box<[crate::declaration::UiDeclaredMeasurementBasisSource]> {
        WorthUiActiveApplicationSession::measurement_basis_sources_for_certification(self)
    }

    fn ordinary_plan_availability(&self) -> WorthUiOrdinaryPlanAvailability {
        WorthUiActiveApplicationSession::ordinary_plan_availability(self)
    }

    fn virtualized_plan_availability(&self) -> WorthUiVirtualizedPlanAvailability {
        WorthUiActiveApplicationSession::virtualized_plan_availability(self)
    }

    fn query_fact_link(&self, binding_id: &str) -> Option<WorthUiQueryLaneFactLink> {
        WorthUiActiveApplicationSession::query_fact_link(self, binding_id)
    }

    fn canvas_spatial_plan_availability(&self) -> WorthUiCanvasSpatialPlanAvailability {
        WorthUiActiveApplicationSession::canvas_spatial_plan_availability(self)
    }

    fn first_canvas_spatial_handle(&self) -> Option<WorthUiLaneHandle> {
        WorthUiActiveApplicationSession::first_canvas_spatial_handle(self)
    }

    fn inspect_canvas_spatial_target(
        &self,
        handle: WorthUiLaneHandle,
    ) -> Result<WorthUiCanvasSpatialTargetSummary, WorthUiCanvasSpatialInspectionDenial> {
        WorthUiActiveApplicationSession::inspect_canvas_spatial_target(self, handle)
    }

    fn realtime_plan_availability(&self) -> WorthUiRealtimePlanAvailability {
        WorthUiActiveApplicationSession::realtime_plan_availability(self)
    }

    fn first_realtime_renderer_surface(&self) -> Option<WorthUiRendererSurfaceHandle> {
        WorthUiActiveApplicationSession::first_realtime_renderer_surface(self)
    }

    fn inspect_realtime_target(
        &self,
        handle: WorthUiRendererSurfaceHandle,
    ) -> Result<WorthUiRealtimeTargetSummary, WorthUiRealtimeInspectionDenial> {
        WorthUiActiveApplicationSession::inspect_realtime_target(self, handle)
    }

    fn inspect_virtualized_plan(
        &self,
        request: WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<WorthUiVirtualizedPlanSummary, WorthUiVirtualizedPlanSummaryDenial> {
        WorthUiActiveApplicationSession::inspect_virtualized_plan(self, request)
    }

    fn inspect_ordinary_plan(
        &self,
        request: WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<WorthUiOrdinaryPlanSummary, WorthUiOrdinaryPlanSummaryDenial> {
        WorthUiActiveApplicationSession::inspect_ordinary_plan(self, request)
    }

    fn host_measurement_capability(&self) -> WorthUiHostMeasurementCapability {
        WorthUiActiveApplicationSession::host_measurement_capability(self)
    }

    fn lookup_consumed_fact(
        &self,
        fact: &UiProducedFact,
    ) -> Result<UiGraphFactLookupReceipt, UiGraphFactLookupDenial> {
        WorthUiActiveApplicationSession::lookup_consumed_fact_for_certification(self, fact)
    }
}
