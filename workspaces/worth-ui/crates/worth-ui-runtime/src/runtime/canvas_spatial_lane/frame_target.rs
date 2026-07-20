use crate::runtime::{
    WorthUiCanvasOverlayPlan, WorthUiCanvasViewportPlan, WorthUiLaneHandle,
    WorthUiSpatialHitTestPlan,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialFrameTarget {
    kind: WorthUiCanvasSpatialFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiCanvasSpatialFrameTargetKind {
    Viewport(WorthUiCanvasViewportPlan),
    Draw(WorthUiLaneHandle),
    HitTest(WorthUiSpatialHitTestPlan),
    Overlay(WorthUiCanvasOverlayPlan),
    ToolState(WorthUiLaneHandle),
}

impl WorthUiCanvasSpatialFrameTarget {
    pub fn viewport(viewport_plan: WorthUiCanvasViewportPlan) -> Self {
        Self {
            kind: WorthUiCanvasSpatialFrameTargetKind::Viewport(viewport_plan),
        }
    }

    pub fn draw(lane_handle: WorthUiLaneHandle) -> Self {
        Self {
            kind: WorthUiCanvasSpatialFrameTargetKind::Draw(lane_handle),
        }
    }

    pub fn hit_test(hit_test_plan: WorthUiSpatialHitTestPlan) -> Self {
        Self {
            kind: WorthUiCanvasSpatialFrameTargetKind::HitTest(hit_test_plan),
        }
    }

    pub fn overlay(overlay_plan: WorthUiCanvasOverlayPlan) -> Self {
        Self {
            kind: WorthUiCanvasSpatialFrameTargetKind::Overlay(overlay_plan),
        }
    }

    pub fn tool_state(lane_handle: WorthUiLaneHandle) -> Self {
        Self {
            kind: WorthUiCanvasSpatialFrameTargetKind::ToolState(lane_handle),
        }
    }

    pub(crate) fn kind(self) -> WorthUiCanvasSpatialFrameTargetKind {
        self.kind
    }
}
