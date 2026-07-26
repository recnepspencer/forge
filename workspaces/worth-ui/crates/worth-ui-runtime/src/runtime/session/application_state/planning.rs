use super::WorthUiApplicationSessionState;
use crate::runtime::{
    WorthUiCanvasSpatialInspectionDenial, WorthUiCanvasSpatialPlanAvailability,
    WorthUiCanvasSpatialTargetSummary, WorthUiLaneHandle, WorthUiOrdinaryPlanAvailability,
    WorthUiOrdinaryPlanSummary, WorthUiOrdinaryPlanSummaryDenial,
    WorthUiOrdinaryPlanSummaryRequest, WorthUiQueryLaneFactLink, WorthUiRealtimeInspectionDenial,
    WorthUiRealtimePlanAvailability, WorthUiRealtimeTargetSummary, WorthUiRendererSurfaceHandle,
    WorthUiVirtualizedPlanAvailability, WorthUiVirtualizedPlanSummary,
    WorthUiVirtualizedPlanSummaryDenial, WorthUiVirtualizedPlanSummaryRequest,
};

impl WorthUiApplicationSessionState {
    pub(crate) fn ordinary_plan_availability(&self) -> WorthUiOrdinaryPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .ordinary_availability()
    }

    pub(crate) fn virtualized_plan_availability(&self) -> WorthUiVirtualizedPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .virtualized_availability()
    }

    pub(crate) fn query_fact_link(&self, binding_id: &str) -> Option<WorthUiQueryLaneFactLink> {
        let binding_id = crate::capability::ViewBindingId::new(binding_id).ok()?;
        self.runtime
            .active
            .active_plan_ref()
            .query_fact_link_for_binding_id(&binding_id)
    }

    pub(crate) fn canvas_spatial_plan_availability(&self) -> WorthUiCanvasSpatialPlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .canvas_spatial_availability()
    }

    pub(crate) fn first_canvas_spatial_handle(&self) -> Option<WorthUiLaneHandle> {
        self.runtime
            .active
            .active_plan_ref()
            .first_canvas_spatial_handle()
    }

    pub(crate) fn inspect_canvas_spatial_target(
        &self,
        handle: WorthUiLaneHandle,
    ) -> Result<WorthUiCanvasSpatialTargetSummary, WorthUiCanvasSpatialInspectionDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .canvas_spatial_summary(handle)
    }

    pub(crate) fn realtime_plan_availability(&self) -> WorthUiRealtimePlanAvailability {
        self.runtime
            .active
            .active_plan_ref()
            .realtime_availability()
    }

    pub(crate) fn first_realtime_renderer_surface(&self) -> Option<WorthUiRendererSurfaceHandle> {
        self.runtime
            .active
            .active_plan_ref()
            .first_realtime_handle()
    }

    pub(crate) fn inspect_realtime_target(
        &self,
        handle: WorthUiRendererSurfaceHandle,
    ) -> Result<WorthUiRealtimeTargetSummary, WorthUiRealtimeInspectionDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .realtime_summary(handle)
    }

    pub(crate) fn inspect_virtualized_plan(
        &self,
        request: WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<WorthUiVirtualizedPlanSummary, WorthUiVirtualizedPlanSummaryDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .virtualized_summary(&self.runtime.query_binding, request)
    }

    pub(crate) fn inspect_ordinary_plan(
        &self,
        request: WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<WorthUiOrdinaryPlanSummary, WorthUiOrdinaryPlanSummaryDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .ordinary_summary(request)
    }
}
