use crate::runtime::{
    WorthUiPrimitiveDrawPlan, WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitiveProofReceipt, WorthUiRuntimeHost,
};

impl WorthUiRuntimeHost {
    pub fn plan_mounted_primitive_event_dispatch(
        &self,
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
        inner_primitive: Option<&WorthUiPrimitiveProofReceipt>,
        inner_draw_plan: Option<&WorthUiPrimitiveDrawPlan>,
        root_offset_x: f32,
        root_offset_y: f32,
        inner_offset_x: f32,
        inner_offset_y: f32,
    ) -> WorthUiPrimitiveEventDispatchPlan {
        let mut regions = Vec::new();
        if let Some(region) = WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan_at(
            primitive,
            draw_plan,
            WorthUiPrimitiveEventRegionOrder::new(0, 0),
            root_offset_x,
            root_offset_y,
        ) {
            regions.push(region);
        }
        if let (Some(inner), Some(plan)) = (inner_primitive, inner_draw_plan) {
            if let Some(region) =
                WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
                    inner,
                    plan,
                    WorthUiPrimitiveEventRegionOrder::new(1, 0),
                    primitive.surface_id(),
                    inner_offset_x,
                    inner_offset_y,
                )
            {
                regions.push(region);
            }
        }
        WorthUiPrimitiveEventDispatchPlan::from_regions(regions)
    }
}
