use super::{
    WorthUiCanvasSpatialCounters, WorthUiRealtimeLaneCounters, WorthUiSteadyFrameCounterBoundary,
    WorthUiSteadyFrameCounterDenialReason, WorthUiVirtualizedDataCounters, WorthUiVisibleRange,
};

#[test]
fn steady_frame_rejects_virtualized_broad_collection_work() {
    let mut virtualized = WorthUiVirtualizedDataCounters::default();
    virtualized.record_visible_range(WorthUiVisibleRange::rows(30, 10).expect("range is valid"));
    virtualized.record_full_collection_scan();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(31)
        .record_virtualized_counters_for_test(virtualized)
        .seal()
        .expect_err("full collection scans must not hide in virtualized steady frames");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}

#[test]
fn steady_frame_rejects_canvas_truth_or_renderer_internal_reads() {
    let mut canvas = WorthUiCanvasSpatialCounters::default();
    canvas.record_spatial_hit_test();
    canvas.record_domain_geometry_truth_read();

    let geometry_denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(32)
        .record_canvas_counters_for_test(canvas)
        .seal()
        .expect_err("domain geometry truth reads must not hide in canvas steady frames");

    assert_eq!(
        geometry_denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );

    let mut renderer = WorthUiCanvasSpatialCounters::default();
    renderer.record_spatial_hit_test();
    renderer.record_renderer_internal_read();

    let renderer_denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(33)
        .record_canvas_counters_for_test(renderer)
        .seal()
        .expect_err("renderer internals must not be read as steady-frame authority");

    assert_eq!(
        renderer_denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}

#[test]
fn steady_frame_rejects_realtime_ordinary_layout_work() {
    let mut realtime = WorthUiRealtimeLaneCounters::default();
    realtime.record_frame_synchronized_pass();
    realtime.record_ordinary_layout_pass();

    let denial = WorthUiSteadyFrameCounterBoundary::for_active_plan(34)
        .record_realtime_counters_for_test(realtime)
        .seal()
        .expect_err("ordinary layout recompute work must not hide in realtime overlays");

    assert_eq!(
        denial.reason(),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}
