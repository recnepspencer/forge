use worth_ui::facade::{
    WorthUiCanvasDrawHook, WorthUiCanvasOverlayPlan, WorthUiCanvasSpatialCertification,
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameDenial,
    WorthUiCanvasSpatialFrameDenialReason, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiCanvasSpatialNode,
    WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial,
    WorthUiCanvasSpatialPlanDenialReason, WorthUiCanvasViewportPlan,
    WorthUiCanvasViewportPlanDenial, WorthUiCanvasViewportPlanDenialReason, WorthUiLaneHandle,
    WorthUiSpatialHitTestHook, WorthUiSpatialHitTestPlan, WorthUiSpatialToolStateHook,
    WorthUiSpatialViewportPoint,
};

fn accepts_canvas_spatial_types(
    _draw_hook: Option<WorthUiCanvasDrawHook>,
    _overlay_plan: Option<WorthUiCanvasOverlayPlan>,
    _certification: Option<WorthUiCanvasSpatialCertification>,
    _counters: Option<WorthUiCanvasSpatialCounters>,
    _frame_denial: Option<WorthUiCanvasSpatialFrameDenial>,
    _frame_reason: Option<WorthUiCanvasSpatialFrameDenialReason>,
    _frame_receipt: Option<WorthUiCanvasSpatialFrameReceipt>,
    _target: Option<WorthUiCanvasSpatialFrameTarget>,
    _viewport_plan: Option<WorthUiCanvasViewportPlan>,
    _viewport_denial: Option<WorthUiCanvasViewportPlanDenial>,
    _viewport_denial_reason: Option<WorthUiCanvasViewportPlanDenialReason>,
    _lane: Option<WorthUiCanvasSpatialLane>,
    _node: Option<WorthUiCanvasSpatialNode>,
    _plan: Option<WorthUiCanvasSpatialPlan>,
    _plan_denial: Option<WorthUiCanvasSpatialPlanDenial>,
    _plan_reason: Option<WorthUiCanvasSpatialPlanDenialReason>,
    _hit_test_hook: Option<WorthUiSpatialHitTestHook>,
    _hit_test_plan: Option<WorthUiSpatialHitTestPlan>,
    _tool_state_hook: Option<WorthUiSpatialToolStateHook>,
    _point: Option<WorthUiSpatialViewportPoint>,
    _lane_handle: Option<WorthUiLaneHandle>,
) {
}

fn main() {
    let point = WorthUiSpatialViewportPoint::viewport(144, 96);
    let _ = point.x();
    let _ = WorthUiCanvasSpatialLane::Draw.canonical_tag();
    let _ = WorthUiCanvasSpatialPlanDenialReason::NoCanvasSpatialRows;
    let _ = WorthUiCanvasSpatialPlanDenialReason::HandleAllocationPlanMismatch;
    let _ = WorthUiCanvasSpatialFrameDenialReason::TargetFamilyMismatch;
    let _ = WorthUiCanvasViewportPlanDenialReason::ZeroZoomFactor;
    accepts_canvas_spatial_types(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(WorthUiCanvasSpatialLane::HitTest),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(point),
        None,
    );
}
