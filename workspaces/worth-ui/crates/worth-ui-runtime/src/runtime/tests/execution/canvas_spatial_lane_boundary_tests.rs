use super::canvas_spatial_lane_test_support::{
    canvas_spatial_denial_for_mismatched_handle_allocation, canvas_spatial_denial_for_missing_hook,
    canvas_spatial_denial_for_missing_support, canvas_spatial_denial_for_stale_lane_admission,
    canvas_spatial_fixture,
};
use crate::runtime::{
    WorthUiCanvasOverlayPlan, WorthUiCanvasSpatialFrameDenialReason,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane,
    WorthUiCanvasSpatialPlanDenialReason, WorthUiCanvasViewportPlan,
    WorthUiCanvasViewportPlanDenialReason, WorthUiSpatialHitTestPlan, WorthUiSpatialViewportPoint,
};

#[test]
fn equivalent_canvas_plans_produce_equivalent_spatial_lane_receipts() {
    let left = canvas_spatial_fixture();
    let right = canvas_spatial_fixture();
    let left_lane = left.canvas_plan.rows()[0].lane_handle();
    let right_lane = right.canvas_plan.rows()[0].lane_handle();

    let left_receipt = left
        .runtime
        .execute_canvas_spatial_frame(
            &left.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::hit_test(
                WorthUiSpatialHitTestPlan::for_viewport_point(
                    left_lane,
                    WorthUiSpatialViewportPoint::viewport(144, 96),
                ),
            ),
        )
        .expect("runtime frame execution succeeds");
    let right_receipt = right
        .runtime
        .execute_canvas_spatial_frame(
            &right.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::hit_test(
                WorthUiSpatialHitTestPlan::for_viewport_point(
                    right_lane,
                    WorthUiSpatialViewportPoint::viewport(144, 96),
                ),
            ),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(
        left.canvas_plan.canvas_plan_digest(),
        right.canvas_plan.canvas_plan_digest()
    );
    assert_eq!(left_receipt.lane(), WorthUiCanvasSpatialLane::HitTest);
    assert_eq!(
        left_receipt.touched_plan_indexes(),
        right_receipt.touched_plan_indexes()
    );
    assert_eq!(
        left_receipt.certification().canvas_plan_digest(),
        right_receipt.certification().canvas_plan_digest()
    );
    assert_eq!(
        left_receipt.counters().spatial_hit_test_count(),
        right_receipt.counters().spatial_hit_test_count()
    );

    let left_overlay = left
        .runtime
        .execute_canvas_spatial_frame(
            &left.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::overlay(WorthUiCanvasOverlayPlan::for_lane(left_lane)),
        )
        .expect("runtime frame execution succeeds");
    let right_overlay = right
        .runtime
        .execute_canvas_spatial_frame(
            &right.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::overlay(WorthUiCanvasOverlayPlan::for_lane(
                right_lane,
            )),
        )
        .expect("runtime frame execution succeeds");
    assert_eq!(
        left_overlay.touched_plan_indexes(),
        right_overlay.touched_plan_indexes()
    );
    assert_eq!(
        left_overlay.counters().overlay_plan_count(),
        right_overlay.counters().overlay_plan_count()
    );
}

#[test]
fn canvas_lane_rejects_domain_truth_or_scene_renderer_ownership() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let domain_denial = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::domain_geometry_truth_owner_for_test(lane),
        )
        .expect_err("runtime frame execution denies");
    let renderer_denial = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::renderer_internal_owner_for_test(lane),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        domain_denial.reason(),
        WorthUiCanvasSpatialFrameDenialReason::DomainGeometryTruthOwnership
    );
    assert_eq!(
        renderer_denial.reason(),
        WorthUiCanvasSpatialFrameDenialReason::RendererInternalOwnership
    );
    assert_eq!(
        domain_denial.counters().domain_geometry_truth_read_count(),
        1
    );
    assert_eq!(renderer_denial.counters().renderer_internal_read_count(), 1);
}

#[test]
fn spatial_lane_preserves_command_and_selection_identity() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let receipt = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::tool_state(lane),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(receipt.lane(), WorthUiCanvasSpatialLane::ToolState);
    assert!(!receipt.command_plan_indexes().is_empty());
    assert!(!receipt.selection_state_slot_handles().is_empty());
    assert!(!fixture.canvas_plan.tool_state_hooks().is_empty());
    assert_eq!(
        receipt.selection_state_slot_handles(),
        fixture.allocation.state_slot_handles()
    );
    assert_eq!(
        receipt.counters().command_identity_preservation_count(),
        receipt.command_plan_indexes().len()
    );
    assert_eq!(
        receipt.counters().selection_identity_preservation_count(),
        1
    );
}

#[test]
fn canvas_viewport_transform_supports_pan_and_zoom_without_scene_ownership() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let viewport_plan =
        WorthUiCanvasViewportPlan::pan_zoom(lane, 12, -8, 1250).expect("zoom factor is valid");
    let receipt = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::viewport(viewport_plan),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(receipt.lane(), WorthUiCanvasSpatialLane::ViewportTransform);
    assert_eq!(receipt.touched_plan_indexes(), &[lane.plan_index()]);
    assert_eq!(receipt.counters().viewport_transform_count(), 1);
    assert_eq!(receipt.counters().domain_geometry_truth_read_count(), 0);
    assert_eq!(receipt.counters().renderer_internal_read_count(), 0);
}

#[test]
fn canvas_viewport_transform_rejects_zero_zoom_without_hidden_clamp() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let denial =
        WorthUiCanvasViewportPlan::pan_zoom(lane, 0, 0, 0).expect_err("zero zoom is invalid");

    assert_eq!(
        denial.reason(),
        WorthUiCanvasViewportPlanDenialReason::ZeroZoomFactor
    );
}

#[test]
fn canvas_hit_test_cannot_read_domain_geometry_truth_directly() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let denial = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::domain_geometry_hit_test_for_test(lane),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiCanvasSpatialFrameDenialReason::DomainGeometryTruthRead
    );
    assert_eq!(denial.counters().domain_geometry_truth_read_count(), 1);
}

#[test]
fn custom_canvas_draw_hook_preserves_platform_identity_and_counters() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let receipt = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::draw(lane),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(receipt.lane(), WorthUiCanvasSpatialLane::Draw);
    assert_eq!(
        receipt.touched_runtime_handles()[0].plan_index(),
        lane.plan_index()
    );
    assert_eq!(receipt.counters().draw_hook_count(), 1);
    assert_eq!(receipt.counters().draw_pass_count(), 1);
    assert_eq!(receipt.counters().renderer_reference_count(), 1);
    assert!(!receipt.diagnostics_plan_indexes().is_empty());
}

#[test]
fn canvas_overlay_keeps_lane_handle_scope() {
    let fixture = canvas_spatial_fixture();
    let lane = fixture.canvas_plan.rows()[0].lane_handle();
    let receipt = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::overlay(WorthUiCanvasOverlayPlan::for_lane(lane)),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(receipt.lane(), WorthUiCanvasSpatialLane::Overlay);
    assert_eq!(receipt.counters().overlay_plan_count(), 1);
    assert_eq!(receipt.touched_plan_indexes(), &[lane.plan_index()]);
}

#[test]
fn canvas_plan_rejects_missing_hook_support_and_stale_admission() {
    assert_eq!(
        canvas_spatial_denial_for_missing_hook().reason(),
        WorthUiCanvasSpatialPlanDenialReason::MissingCanvasSpatialHook
    );
    assert_eq!(
        canvas_spatial_denial_for_missing_support().reason(),
        WorthUiCanvasSpatialPlanDenialReason::LaneAdmissionMissingCanvasSpatialSupport
    );
    assert_eq!(
        canvas_spatial_denial_for_stale_lane_admission().reason(),
        WorthUiCanvasSpatialPlanDenialReason::LaneAdmissionPlanMismatch
    );
    assert_eq!(
        canvas_spatial_denial_for_mismatched_handle_allocation().reason(),
        WorthUiCanvasSpatialPlanDenialReason::HandleAllocationPlanMismatch
    );
}

#[test]
fn canvas_lane_rejects_noncanvas_frame_claims() {
    let fixture = canvas_spatial_fixture();
    let component = fixture.allocation.component_handles()[0];
    let denial = fixture
        .runtime
        .execute_canvas_spatial_frame(
            &fixture.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::component_for_test(component),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiCanvasSpatialFrameDenialReason::NonCanvasSpatialClaim
    );
}
