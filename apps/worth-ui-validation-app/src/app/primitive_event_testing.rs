use worth_ui::facade::{
    SurfaceId, WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitivePointerFrameInput,
    WorthUiPrimitivePointerFrameReceipt, WorthUiPrimitiveProofDenial, WorthUiPrimitiveProofReceipt,
};

use super::ValidationWorkbenchApp;

const OUTER_SURFACE: &str = "worth.surface.preview.primitive.proof";
const INNER_SURFACE: &str = "worth.surface.preview.primitive.inner";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidationMountedPrimitiveEventViewport {
    width: f32,
    height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationMountedPrimitiveEventFrameReceipt {
    pointer_frame: WorthUiPrimitivePointerFrameReceipt,
    event_plan: WorthUiPrimitiveEventDispatchPlan,
}

#[derive(Debug)]
pub enum ValidationMountedPrimitiveEventFrameDenial {
    OuterPrimitive(WorthUiPrimitiveProofDenial),
    InnerPrimitive(WorthUiPrimitiveProofDenial),
}

impl ValidationMountedPrimitiveEventViewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn width(self) -> f32 {
        self.width
    }

    pub fn height(self) -> f32 {
        self.height
    }
}

impl ValidationMountedPrimitiveEventFrameReceipt {
    pub fn pointer_frame(&self) -> &WorthUiPrimitivePointerFrameReceipt {
        &self.pointer_frame
    }

    pub fn event_plan(&self) -> &WorthUiPrimitiveEventDispatchPlan {
        &self.event_plan
    }
}

impl ValidationWorkbenchApp {
    pub fn mounted_primitive_event_frame_for_proof(
        &self,
        viewport: ValidationMountedPrimitiveEventViewport,
        input: WorthUiPrimitivePointerFrameInput,
    ) -> Result<
        ValidationMountedPrimitiveEventFrameReceipt,
        ValidationMountedPrimitiveEventFrameDenial,
    > {
        let outer = self
            .workbench
            .runtime()
            .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
            .map_err(ValidationMountedPrimitiveEventFrameDenial::OuterPrimitive)?;
        let inner = self
            .workbench
            .runtime()
            .resolve_primitive_proof(&surface_id(INNER_SURFACE))
            .map_err(ValidationMountedPrimitiveEventFrameDenial::InnerPrimitive)?;
        let event_plan = mounted_primitive_event_plan(&outer, &inner, viewport);
        let pointer_frame =
            WorthUiPrimitivePointerFrameReceipt::from_dispatch_plan(&event_plan, input);
        Ok(ValidationMountedPrimitiveEventFrameReceipt {
            pointer_frame,
            event_plan,
        })
    }
}

fn mounted_primitive_event_plan(
    outer: &WorthUiPrimitiveProofReceipt,
    inner: &WorthUiPrimitiveProofReceipt,
    viewport: ValidationMountedPrimitiveEventViewport,
) -> WorthUiPrimitiveEventDispatchPlan {
    let outer_plan = outer.draw_plan(viewport.width(), viewport.height());
    let outer_frame = outer_plan.frame();
    let inner_plan = inner.draw_plan(outer_frame.width(), outer_frame.height());
    let mut regions = Vec::new();
    if let Some(region) = WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan(
        outer,
        &outer_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
    ) {
        regions.push(region);
    }
    if let Some(region) = WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
        inner,
        &inner_plan,
        WorthUiPrimitiveEventRegionOrder::new(1, 0),
        outer.surface_id(),
        outer_frame.x(),
        outer_frame.y(),
    ) {
        regions.push(region);
    }
    WorthUiPrimitiveEventDispatchPlan::from_regions(regions)
}

fn surface_id(surface_id: &str) -> SurfaceId {
    SurfaceId::new(surface_id).expect("validation primitive surface ids are static")
}
