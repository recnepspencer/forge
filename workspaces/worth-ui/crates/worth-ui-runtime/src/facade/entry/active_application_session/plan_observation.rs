use super::super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(crate) fn ordinary_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiOrdinaryPlanAvailability {
        self.application.ordinary_plan_availability()
    }

    pub(crate) fn virtualized_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiVirtualizedPlanAvailability {
        self.application.virtualized_plan_availability()
    }

    pub(crate) fn query_fact_link(
        &self,
        binding_id: &str,
    ) -> Option<crate::runtime::WorthUiQueryLaneFactLink> {
        self.application.query_fact_link(binding_id)
    }

    pub(crate) fn canvas_spatial_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiCanvasSpatialPlanAvailability {
        self.application.canvas_spatial_plan_availability()
    }

    pub(crate) fn first_canvas_spatial_handle(&self) -> Option<crate::runtime::WorthUiLaneHandle> {
        self.application.first_canvas_spatial_handle()
    }

    pub(crate) fn inspect_canvas_spatial_target(
        &self,
        handle: crate::runtime::WorthUiLaneHandle,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialTargetSummary,
        crate::runtime::WorthUiCanvasSpatialInspectionDenial,
    > {
        self.application.inspect_canvas_spatial_target(handle)
    }

    pub(crate) fn realtime_plan_availability(
        &self,
    ) -> crate::runtime::WorthUiRealtimePlanAvailability {
        self.application.realtime_plan_availability()
    }

    pub(crate) fn first_realtime_renderer_surface(
        &self,
    ) -> Option<crate::runtime::WorthUiRendererSurfaceHandle> {
        self.application.first_realtime_renderer_surface()
    }

    pub(crate) fn inspect_realtime_target(
        &self,
        handle: crate::runtime::WorthUiRendererSurfaceHandle,
    ) -> Result<
        crate::runtime::WorthUiRealtimeTargetSummary,
        crate::runtime::WorthUiRealtimeInspectionDenial,
    > {
        self.application.inspect_realtime_target(handle)
    }

    pub(crate) fn inspect_virtualized_plan(
        &self,
        request: crate::runtime::WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedPlanSummary,
        crate::runtime::WorthUiVirtualizedPlanSummaryDenial,
    > {
        self.application.inspect_virtualized_plan(request)
    }

    pub(crate) fn inspect_ordinary_plan(
        &self,
        request: crate::runtime::WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiOrdinaryPlanSummary,
        crate::runtime::WorthUiOrdinaryPlanSummaryDenial,
    > {
        self.application.inspect_ordinary_plan(request)
    }
}
