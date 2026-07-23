use super::realtime_overlay_lane_test_support::{
    realtime_launch_denial, realtime_overlay_fixture, RealtimeOverlayFixture,
};
use crate::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiHandleSlotGeneration, WorthUiHighFrequencyFramePolicy,
    WorthUiHighFrequencyFramePolicyDenialReason, WorthUiHudPlanDenialReason,
    WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFramePriority, WorthUiRealtimeFrameTarget,
    WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceHandle, WorthUiRuntimeHandleLocator,
};

#[test]
fn equivalent_realtime_meaning_produces_equivalent_compact_receipts() {
    let mut left = realtime_overlay_fixture();
    let mut right = realtime_overlay_fixture();
    let left_receipt = left
        .execute(WorthUiRealtimeFrameTarget::renderer_surface(left.handle()))
        .expect("left active realtime target executes");
    let right_receipt = right
        .execute(WorthUiRealtimeFrameTarget::renderer_surface(right.handle()))
        .expect("right active realtime target executes");

    assert_eq!(left_receipt.lane(), WorthUiRealtimeOverlayLane::HudOverlay);
    assert_eq!(left_receipt.touched_overlay_row_count(), 8);
    assert_eq!(
        left_receipt.touched_overlay_row_count(),
        right_receipt.touched_overlay_row_count()
    );
    assert_eq!(
        left_receipt.certification().hud_plan_digest(),
        right_receipt.certification().hud_plan_digest()
    );
    assert_eq!(left_receipt.touched_plan_indexes().len(), 1);
    assert_eq!(left_receipt.counters().allocation_count(), 0);
}

#[test]
fn foreign_renderer_surface_denies_before_frame_work_and_inspection_authority() {
    let first = realtime_overlay_fixture();
    let foreign = first.handle();
    let mut second = realtime_overlay_fixture();
    let inspection = second
        .session
        .inspect_realtime_target(foreign)
        .expect_err("foreign renderer surface cannot be inspected as active");
    assert_eq!(
        inspection.outcome(),
        WorthUiHandleResolutionOutcome::ForeignSessionArena
    );
    let denial = second
        .execute(WorthUiRealtimeFrameTarget::renderer_surface(foreign))
        .expect_err("foreign renderer surface cannot execute");
    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::TargetArenaMismatch
    );
    assert_eq!(denial.counters().frame_synchronized_pass_count(), 0);
    assert_eq!(denial.counters().renderer_surface_handoff_count(), 0);
}

#[test]
fn stale_renderer_surface_denies_before_frame_work() {
    let mut fixture = realtime_overlay_fixture();
    let fresh = fixture.handle();
    let locator = fresh.locator();
    let stale = WorthUiRendererSurfaceHandle::new(WorthUiRuntimeHandleLocator::new(
        locator.arena_identity(),
        locator.plan_index(),
        WorthUiHandleSlotGeneration::new(locator.slot_generation().as_u64() + 1),
    ));
    let denial = fixture
        .execute(WorthUiRealtimeFrameTarget::renderer_surface(stale))
        .expect_err("stale renderer surface cannot execute");
    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::TargetSlotGenerationMismatch
    );
    assert_eq!(denial.counters().frame_synchronized_pass_count(), 0);
}

#[test]
fn zero_and_overflowing_frame_budgets_are_typed_contract_denials() {
    let zero = WorthUiHighFrequencyFramePolicy::frame_budgeted(
        0,
        WorthUiRealtimeFramePriority::HudOverlay,
    )
    .expect_err("zero budget denies");
    assert_eq!(
        zero.reason(),
        WorthUiHighFrequencyFramePolicyDenialReason::ZeroFrameBudgetMillis
    );
    let overflow = WorthUiHighFrequencyFramePolicy::frame_budgeted(
        u32::from(u16::MAX) + 1,
        WorthUiRealtimeFramePriority::CriticalOverlay,
    )
    .expect_err("overflowing budget denies");
    assert_eq!(
        overflow.reason(),
        WorthUiHighFrequencyFramePolicyDenialReason::FrameBudgetOverflow
    );
}

#[test]
fn declared_frame_cost_above_budget_denies_before_active_publication() {
    let denial = realtime_launch_denial(8, 17, 16);
    assert!(matches!(
        denial,
        crate::runtime::WorthUiRuntimeLaunchDenial::RealtimeOverlayPlan(ref plan)
            if plan.reason() == WorthUiHudPlanDenialReason::FrameBudgetExhausted {
                budget_millis: 16,
                declared_cost_millis: 17,
            }
    ));
}

#[test]
fn successful_summary_is_compact_and_generation_bound() {
    let fixture = RealtimeOverlayFixture::new(32, 5, 16);
    let summary = fixture
        .session
        .inspect_realtime_target(fixture.handle())
        .expect("active target summary resolves directly");
    assert_eq!(summary.overlay_row_limit(), 32);
    assert_eq!(summary.declared_frame_cost_millis(), 5);
    assert_eq!(
        summary.host_session_identity(),
        fixture.session.host_session_identity().as_u64()
    );
}
