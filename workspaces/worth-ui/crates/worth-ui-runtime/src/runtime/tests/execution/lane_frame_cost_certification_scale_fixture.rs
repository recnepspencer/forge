use super::realtime_overlay_lane_test_support::RealtimeOverlayFixture;
use super::virtualized_data_lane_test_support::virtualized_data_fixture;
use super::{
    WorthUiFrameExecutionReceipt, WorthUiRealtimeFrameTarget, WorthUiSteadyFrameCounterBoundary,
    WorthUiVisibleRange,
};

pub(super) fn virtualized_data_scale_sample(
    active_plan_digest: u64,
    rows: u32,
    columns: u32,
) -> WorthUiFrameExecutionReceipt {
    let mut virtualized = virtualized_data_fixture();
    let range = WorthUiVisibleRange::grid(rows, rows, 0, columns).expect("range is valid");
    let target = virtualized.summary().target(range);
    let receipt = virtualized
        .execute(target)
        .expect("runtime frame execution succeeds");

    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_virtualized_data_frame(receipt)
        .seal()
        .expect("virtualized scale sample seals")
}

pub(super) fn realtime_overlay_scale_sample(
    active_plan_digest: u64,
    wider_overlay: bool,
) -> WorthUiFrameExecutionReceipt {
    let row_limit = if wider_overlay { 32 } else { 8 };
    let mut fixture = RealtimeOverlayFixture::new(row_limit, 4, 16);
    let handle = fixture.handle();
    let receipt = fixture
        .execute(WorthUiRealtimeFrameTarget::renderer_surface(handle))
        .expect("active realtime target executes");

    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_realtime_overlay_frame(receipt)
        .seal()
        .expect("realtime scale sample seals")
}
