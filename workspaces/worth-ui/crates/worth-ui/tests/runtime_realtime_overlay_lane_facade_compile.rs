#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_realtime_overlay_lane_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_realtime_overlay_lane_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn realtime_overlay_lane_facade_types_are_visible() {
    runtime_realtime_overlay_lane_compile_pass(
        "tests/ui/runtime_authority/pass/runtime_realtime_overlay_lane_facade_types.rs",
    );
}

#[test]
fn hud_plan_fields_are_not_publicly_mintable() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/hud_plan_fields_not_public.rs",
    );
}

#[test]
fn realtime_frame_receipt_fields_are_not_publicly_mintable() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/realtime_frame_receipt_fields_not_public.rs",
    );
}

#[test]
fn realtime_counters_are_not_publicly_mintable() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/realtime_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn realtime_hook_fields_are_not_publicly_mintable() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/realtime_overlay_hook_fields_not_public.rs",
    );
}

#[test]
fn raw_renderer_pointer_cannot_execute_realtime_lane() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/raw_renderer_pointer_cannot_execute_realtime_lane.rs",
    );
}

#[test]
fn ordinary_widget_fallback_cannot_execute_realtime_lane() {
    runtime_realtime_overlay_lane_compile_fail(
        "tests/ui/runtime_authority/fail/ordinary_widget_fallback_cannot_execute_realtime_lane.rs",
    );
}
