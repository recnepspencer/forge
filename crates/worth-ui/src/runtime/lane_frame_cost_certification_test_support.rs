use super::canvas_spatial_lane_test_support::canvas_spatial_fixture;
use super::lane_frame_cost_certification_scale_fixture::{
    realtime_overlay_scale_sample, virtualized_data_scale_sample,
};
use super::ordinary_lane_test_support::ordinary_lane_fixture;
use super::realtime_overlay_lane_test_support::realtime_overlay_fixture;
use super::virtualized_data_lane_test_support::virtualized_data_fixture;
use super::{
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane,
    WorthUiFrameExecutionReceipt, WorthUiLaneFrameCostCertificationScenario,
    WorthUiLaneParityCertification, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCounters,
    WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayLane,
    WorthUiRuntimeHandleAllocation, WorthUiSteadyFrameCounterBoundary, WorthUiViewBindingHandle,
    WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane,
    WorthUiVisibleRange,
};

pub(super) fn complete_lane_frame_cost_scenario(
    active_plan_digest: u64,
) -> WorthUiLaneFrameCostCertificationScenario {
    WorthUiLaneFrameCostCertificationScenario::named("lane-frame-cost")
        .with_steady_frame_receipt(complete_platform_lane_frame_receipt(active_plan_digest))
        .with_virtualized_data_scale_sample(virtualized_data_scale_sample(
            active_plan_digest,
            120,
            24,
        ))
        .with_realtime_scale_sample(realtime_overlay_scale_sample(active_plan_digest, true))
        .with_cross_lane_parity(cross_lane_parity(active_plan_digest))
        .require_foundational_readiness()
}

pub(super) fn scenario_without_scale_variation(
    active_plan_digest: u64,
) -> WorthUiLaneFrameCostCertificationScenario {
    WorthUiLaneFrameCostCertificationScenario::named("constant-scale")
        .with_steady_frame_receipt(complete_platform_lane_frame_receipt(active_plan_digest))
        .with_virtualized_data_scale_sample(virtualized_data_scale_sample(
            active_plan_digest,
            120,
            12,
        ))
        .with_realtime_scale_sample(realtime_overlay_scale_sample(active_plan_digest, false))
        .with_cross_lane_parity(cross_lane_parity(active_plan_digest))
        .require_foundational_readiness()
}

pub(super) fn scenario_without_cross_lane_parity(
    active_plan_digest: u64,
) -> WorthUiLaneFrameCostCertificationScenario {
    WorthUiLaneFrameCostCertificationScenario::named("missing-parity")
        .with_steady_frame_receipt(complete_platform_lane_frame_receipt(active_plan_digest))
        .with_virtualized_data_scale_sample(virtualized_data_scale_sample(
            active_plan_digest,
            120,
            24,
        ))
        .with_realtime_scale_sample(realtime_overlay_scale_sample(active_plan_digest, true))
        .require_foundational_readiness()
}

pub(super) fn scenario_without_foundational_readiness(
    active_plan_digest: u64,
) -> WorthUiLaneFrameCostCertificationScenario {
    WorthUiLaneFrameCostCertificationScenario::named("missing-foundational-readiness")
        .with_steady_frame_receipt(complete_platform_lane_frame_receipt(active_plan_digest))
        .with_virtualized_data_scale_sample(virtualized_data_scale_sample(
            active_plan_digest,
            120,
            24,
        ))
        .with_realtime_scale_sample(realtime_overlay_scale_sample(active_plan_digest, true))
        .with_cross_lane_parity(cross_lane_parity(active_plan_digest))
}

pub(super) fn scenario_with_mismatched_cross_lane_parity(
    active_plan_digest: u64,
    parity_plan_digest: u64,
) -> WorthUiLaneFrameCostCertificationScenario {
    WorthUiLaneFrameCostCertificationScenario::named("mismatched-parity")
        .with_steady_frame_receipt(complete_platform_lane_frame_receipt(active_plan_digest))
        .with_virtualized_data_scale_sample(virtualized_data_scale_sample(
            active_plan_digest,
            120,
            24,
        ))
        .with_realtime_scale_sample(realtime_overlay_scale_sample(active_plan_digest, true))
        .with_cross_lane_parity(cross_lane_parity(parity_plan_digest))
        .require_foundational_readiness()
}

pub(super) fn partial_lane_receipt(active_plan_digest: u64) -> WorthUiFrameExecutionReceipt {
    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_ordinary_counters_for_test(ordinary_counters())
        .seal()
        .expect("partial receipt can seal before lane coverage certification")
}

pub(super) fn complete_frame_receipt(
    active_plan_digest: u64,
    data_rows: u32,
    data_columns: u32,
    realtime_passes: usize,
) -> WorthUiFrameExecutionReceipt {
    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_ordinary_counters_for_test(ordinary_counters())
        .record_virtualized_counters_for_test(virtualized_counters(data_rows, data_columns))
        .record_canvas_counters_for_test(canvas_counters())
        .record_realtime_counters_for_test(realtime_counters(realtime_passes))
        .seal()
        .expect("complete frame receipt seals")
}

pub(super) fn complete_synthetic_frame_receipt(
    active_plan_digest: u64,
) -> WorthUiFrameExecutionReceipt {
    complete_frame_receipt(active_plan_digest, 40, 12, 1)
}

fn complete_platform_lane_frame_receipt(active_plan_digest: u64) -> WorthUiFrameExecutionReceipt {
    let (ordinary_runtime, ordinary_plan, ordinary_allocation) = ordinary_lane_fixture();
    let ordinary_handle = ordinary_allocation.component_handles()[0];
    let ordinary_receipt = ordinary_runtime
        .execute_ordinary_lane_frame(
            &ordinary_plan,
            WorthUiOrdinaryFrameTarget::component(ordinary_handle),
        )
        .expect("ordinary frame executes");

    let virtualized = virtualized_data_fixture();
    let data_handle = first_view_binding_handle(&virtualized.allocation);
    let range = WorthUiVisibleRange::grid(120, 40, 0, 12).expect("range is valid");
    let virtualized_receipt = virtualized
        .runtime
        .execute_virtualized_data_frame(
            &virtualized.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(data_handle, range),
        )
        .expect("virtualized frame executes");

    let canvas = canvas_spatial_fixture();
    let canvas_lane = canvas.canvas_plan.rows()[0].lane_handle();
    let canvas_receipt = canvas
        .runtime
        .execute_canvas_spatial_frame(
            &canvas.canvas_plan,
            WorthUiCanvasSpatialFrameTarget::hit_test(
                super::WorthUiSpatialHitTestPlan::for_viewport_point(
                    canvas_lane,
                    super::WorthUiSpatialViewportPoint::viewport(144, 96),
                ),
            ),
        )
        .expect("canvas frame executes");

    let realtime = realtime_overlay_fixture();
    let surface = realtime.hud_plan.renderer_surfaces()[0].handle();
    let realtime_receipt = realtime
        .runtime
        .execute_realtime_frame(
            &realtime.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(surface),
        )
        .expect("realtime frame executes");

    assert_eq!(
        virtualized_receipt.lane(),
        WorthUiVirtualizedDataLane::CellGrid
    );
    assert_eq!(canvas_receipt.lane(), WorthUiCanvasSpatialLane::HitTest);
    assert_eq!(
        realtime_receipt.lane(),
        WorthUiRealtimeOverlayLane::HudOverlay
    );

    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_ordinary_lane_frame(ordinary_receipt)
        .record_virtualized_data_frame(virtualized_receipt)
        .record_canvas_spatial_frame(canvas_receipt)
        .record_realtime_overlay_frame(realtime_receipt)
        .seal()
        .expect("steady frame receipt seals")
}

pub(super) fn source_parse_frame_denial(
    active_plan_digest: u64,
) -> super::WorthUiSteadyFrameCounterDenialReason {
    let mut counters = ordinary_counters();
    counters.record_source_parse();
    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_ordinary_counters_for_test(counters)
        .seal()
        .expect_err("source parse cannot enter steady frame certification")
        .reason()
}

pub(super) fn full_collection_frame_denial(
    active_plan_digest: u64,
) -> super::WorthUiSteadyFrameCounterDenialReason {
    let mut counters = virtualized_counters(40, 12);
    counters.record_full_collection_scan();
    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_virtualized_counters_for_test(counters)
        .seal()
        .expect_err("full collection scan cannot enter steady frame certification")
        .reason()
}

pub(super) fn realtime_ordinary_traversal_denial(
    active_plan_digest: u64,
) -> super::WorthUiSteadyFrameCounterDenialReason {
    let mut counters = realtime_counters(1);
    counters.record_ordinary_layout_pass();
    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_realtime_counters_for_test(counters)
        .seal()
        .expect_err("realtime ordinary traversal cannot enter steady frame certification")
        .reason()
}

fn ordinary_counters() -> WorthUiOrdinaryLaneCounters {
    let mut counters = WorthUiOrdinaryLaneCounters::default();
    counters.record_frame_row_touch();
    counters
}

fn virtualized_counters(data_rows: u32, data_columns: u32) -> WorthUiVirtualizedDataCounters {
    let mut counters = WorthUiVirtualizedDataCounters::default();
    let range = WorthUiVisibleRange::grid(data_rows, data_rows, 0, data_columns)
        .expect("scale fixture range is valid");
    counters.record_visible_range(range);
    counters
}

fn canvas_counters() -> WorthUiCanvasSpatialCounters {
    let mut counters = WorthUiCanvasSpatialCounters::default();
    counters.record_spatial_hit_test();
    counters.record_draw_pass();
    counters
}

fn realtime_counters(realtime_passes: usize) -> WorthUiRealtimeLaneCounters {
    let mut counters = WorthUiRealtimeLaneCounters::default();
    for _ in 0..realtime_passes {
        counters.record_frame_synchronized_pass();
        counters.record_renderer_surface_handoff();
    }
    counters
}

fn cross_lane_parity(active_plan_digest: u64) -> WorthUiLaneParityCertification {
    WorthUiLaneParityCertification::new(
        0xA11CE,
        0xA11CE,
        active_plan_digest,
        active_plan_digest,
        0xC1055_1A9E,
    )
}

fn first_view_binding_handle(
    allocation: &WorthUiRuntimeHandleAllocation,
) -> WorthUiViewBindingHandle {
    allocation
        .view_binding_handles()
        .first()
        .copied()
        .expect("fixture has view binding handle")
}
