use super::realtime_overlay_lane_test_support::{
    realtime_denial_for_mismatched_handle_allocation, realtime_denial_for_missing_hook,
    realtime_denial_for_missing_realtime_support, realtime_denial_for_no_hud_rows,
    realtime_denial_for_stale_lane_admission,
    realtime_denial_for_stale_lane_admission_without_render_rows,
    realtime_denial_for_unsupported_hook, realtime_overlay_fixture,
};
use crate::runtime::{
    WorthUiHandlePlanGeneration, WorthUiHighFrequencyFramePolicy,
    WorthUiHighFrequencyFramePolicyDenialReason, WorthUiHudPlanDenialReason,
    WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFramePriority, WorthUiRealtimeFrameTarget,
    WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceHandle,
};

#[test]
fn equivalent_realtime_plans_produce_equivalent_hud_receipts() {
    let left = realtime_overlay_fixture();
    let right = realtime_overlay_fixture();
    let left_surface = left.hud_plan.renderer_surfaces()[0].handle();
    let right_surface = right.hud_plan.renderer_surfaces()[0].handle();

    let left_receipt = left
        .runtime
        .execute_realtime_frame(
            &left.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(left_surface),
        )
        .expect("runtime frame execution succeeds");
    let right_receipt = right
        .runtime
        .execute_realtime_frame(
            &right.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(right_surface),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(
        left.hud_plan.hud_plan_digest(),
        right.hud_plan.hud_plan_digest()
    );
    assert_eq!(left_receipt.lane(), WorthUiRealtimeOverlayLane::HudOverlay);
    assert_eq!(
        left_receipt.touched_plan_indexes(),
        right_receipt.touched_plan_indexes()
    );
    assert_eq!(
        left_receipt.certification().hud_plan_digest(),
        right_receipt.certification().hud_plan_digest()
    );
    assert_eq!(
        left_receipt.counters().frame_synchronized_pass_count(),
        right_receipt.counters().frame_synchronized_pass_count()
    );
}

#[test]
fn renderer_surface_handle_does_not_bypass_platform_identity() {
    let fixture = realtime_overlay_fixture();
    let surface = fixture.hud_plan.renderer_surfaces()[0].handle();
    let receipt = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(surface),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(receipt.renderer_surface_admission().handle(), surface);
    assert_eq!(
        receipt.touched_runtime_handles()[0].plan_index(),
        surface.plan_index()
    );
    assert_eq!(
        receipt
            .renderer_surface_admission()
            .command_identity_count(),
        receipt.command_plan_indexes().len()
    );
    assert_eq!(
        receipt
            .renderer_surface_admission()
            .accessibility_posture_count(),
        receipt.accessibility_plan_indexes().len()
    );
    assert_eq!(
        receipt
            .renderer_surface_admission()
            .diagnostics_posture_count(),
        receipt.diagnostics_plan_indexes().len()
    );
    assert!(!receipt.command_plan_indexes().is_empty());
    assert!(!receipt.accessibility_plan_indexes().is_empty());
    assert!(!receipt.diagnostics_plan_indexes().is_empty());
    assert_eq!(receipt.counters().renderer_surface_handoff_count(), 1);
    assert_eq!(receipt.counters().accessibility_posture_count(), 1);
    assert_eq!(receipt.counters().ordinary_layout_pass_count(), 0);
}

#[test]
fn realtime_plan_rejects_missing_or_wrong_overlay_hooks_before_rows() {
    let missing_hook = realtime_denial_for_missing_hook();
    assert_eq!(
        missing_hook.reason(),
        WorthUiHudPlanDenialReason::MissingRealtimeOverlayHook
    );
    assert_eq!(missing_hook.counters().denial_count(), 1);
    assert_eq!(missing_hook.counters().hud_plan_row_count(), 0);

    let unsupported_hook = realtime_denial_for_unsupported_hook();
    assert_eq!(
        unsupported_hook.reason(),
        WorthUiHudPlanDenialReason::UnsupportedRealtimeOverlayHook
    );
    assert_eq!(unsupported_hook.counters().denial_count(), 1);
    assert_eq!(unsupported_hook.counters().overlay_hook_count(), 0);
}

#[test]
fn realtime_plan_rejects_missing_realtime_support_before_rows() {
    let missing_realtime_support = realtime_denial_for_missing_realtime_support();
    assert_eq!(
        missing_realtime_support.reason(),
        WorthUiHudPlanDenialReason::LaneAdmissionMissingRealtimeSupport
    );
    assert_eq!(missing_realtime_support.counters().denial_count(), 1);
    assert_eq!(missing_realtime_support.counters().hud_plan_row_count(), 0);
}

#[test]
fn realtime_plan_rejects_stale_lane_admission_before_rows() {
    let stale_lane_admission = realtime_denial_for_stale_lane_admission();
    assert_eq!(
        stale_lane_admission.reason(),
        WorthUiHudPlanDenialReason::LaneAdmissionPlanMismatch
    );
    assert_eq!(
        stale_lane_admission
            .counters()
            .certification_failure_count(),
        1
    );
    assert_eq!(stale_lane_admission.counters().hud_plan_row_count(), 0);

    let stale_admission_without_render_rows =
        realtime_denial_for_stale_lane_admission_without_render_rows();
    assert_eq!(
        stale_admission_without_render_rows.reason(),
        WorthUiHudPlanDenialReason::LaneAdmissionPlanMismatch
    );
    assert_eq!(
        stale_admission_without_render_rows
            .counters()
            .certification_failure_count(),
        1
    );
    assert_eq!(
        stale_admission_without_render_rows
            .counters()
            .renderer_surface_admission_count(),
        0
    );
}

#[test]
fn realtime_plan_rejects_mismatched_handle_allocation_before_rows() {
    let mismatched_handle_allocation = realtime_denial_for_mismatched_handle_allocation();
    assert_eq!(
        mismatched_handle_allocation.reason(),
        WorthUiHudPlanDenialReason::HandleAllocationPlanMismatch
    );
    assert_eq!(
        mismatched_handle_allocation
            .counters()
            .certification_failure_count(),
        1
    );
    assert_eq!(
        mismatched_handle_allocation.counters().hud_plan_row_count(),
        0
    );
}

#[test]
fn realtime_plan_rejects_no_hud_rows_after_preserving_nonrealtime_posture() {
    let no_hud_rows = realtime_denial_for_no_hud_rows();
    assert_eq!(no_hud_rows.reason(), WorthUiHudPlanDenialReason::NoHudRows);
    assert_eq!(no_hud_rows.counters().denial_count(), 1);
    assert_eq!(no_hud_rows.counters().hud_plan_row_count(), 0);
    assert!(no_hud_rows.counters().skipped_nonrealtime_plan_row_count() > 0);
}

#[test]
fn realtime_lane_rejects_ordinary_widget_fallback() {
    let fixture = realtime_overlay_fixture();
    let component = fixture.allocation.component_handles()[0];
    let denial = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::ordinary_widget_fallback_for_test(component),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::OrdinaryWidgetFallback
    );
}

#[test]
fn realtime_lane_rejects_stale_renderer_surface_generation() {
    let fixture = realtime_overlay_fixture();
    let fresh_surface = fixture.hud_plan.renderer_surfaces()[0].handle();
    let stale_generation =
        WorthUiHandlePlanGeneration::new(fresh_surface.plan_generation().as_u64() ^ 0xfeed);
    let stale_surface =
        WorthUiRendererSurfaceHandle::new(fresh_surface.plan_index(), stale_generation);
    let denial = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(stale_surface),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::TargetGenerationMismatch
    );
    assert_eq!(denial.plan_index(), Some(fresh_surface.plan_index()));
    assert_eq!(denial.counters().certification_failure_count(), 1);
}

#[test]
fn realtime_lane_rejects_renderer_surface_not_in_hud_plan() {
    let fixture = realtime_overlay_fixture();
    let fresh_surface = fixture.hud_plan.renderer_surfaces()[0].handle();
    let absent_surface = WorthUiRendererSurfaceHandle::new(
        fresh_surface.plan_index() + 10_000,
        fresh_surface.plan_generation(),
    );
    let denial = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(absent_surface),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::TargetNotInHudPlan
    );
    assert_eq!(denial.plan_index(), Some(absent_surface.plan_index()));
    assert_eq!(denial.counters().denial_count(), 1);
    assert_eq!(denial.counters().frame_synchronized_pass_count(), 0);
    assert_eq!(denial.counters().renderer_surface_handoff_count(), 0);
}

#[test]
fn realtime_lane_counter_detects_hidden_ordinary_layout_pass() {
    let fixture = realtime_overlay_fixture();
    let surface = fixture.hud_plan.renderer_surfaces()[0].handle();
    let denial = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::hidden_ordinary_layout_pass_for_test(surface),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::HiddenOrdinaryLayoutPass
    );
    assert_eq!(denial.counters().ordinary_layout_pass_count(), 1);
    assert_eq!(denial.counters().certification_failure_count(), 1);
}

#[test]
fn custom_realtime_hook_cannot_suppress_forbidden_work_counters() {
    let fixture = realtime_overlay_fixture();
    let surface = fixture.hud_plan.renderer_surfaces()[0].handle();
    let denial = fixture
        .runtime
        .execute_realtime_frame(
            &fixture.hud_plan,
            WorthUiRealtimeFrameTarget::forbidden_work_suppression_for_test(surface),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::ForbiddenWorkCounterSuppression
    );
    assert_eq!(denial.counters().source_parse_count(), 1);
    assert_eq!(denial.counters().registry_lookup_count(), 1);
    assert_eq!(denial.counters().allocation_count(), 1);
    assert_eq!(denial.counters().diagnostic_materialization_count(), 1);
}

#[test]
fn high_frequency_frame_policy_rejects_zero_budget() {
    let denial = WorthUiHighFrequencyFramePolicy::frame_budgeted(
        0,
        WorthUiRealtimeFramePriority::HudOverlay,
    )
    .expect_err("zero frame budget is invalid");

    assert_eq!(
        denial.reason(),
        WorthUiHighFrequencyFramePolicyDenialReason::ZeroFrameBudgetMillis
    );
}
