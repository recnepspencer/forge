#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_canvas_spatial_lane_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_canvas_spatial_lane_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn canvas_spatial_lane_facade_types_are_visible() {
    runtime_canvas_spatial_lane_compile_pass(
        "tests/ui/runtime_authority/pass/runtime_canvas_spatial_lane_facade_types.rs",
    );
}

#[test]
fn canvas_spatial_plan_fields_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/canvas_spatial_plan_fields_not_public.rs",
    );
}

#[test]
fn canvas_spatial_frame_receipt_fields_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/canvas_spatial_frame_receipt_fields_not_public.rs",
    );
}

#[test]
fn canvas_spatial_counters_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/canvas_spatial_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn raw_domain_geometry_cannot_execute_canvas_lane() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/raw_domain_geometry_cannot_execute_canvas_lane.rs",
    );
}

#[test]
fn raw_renderer_pointer_cannot_execute_canvas_lane() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/raw_renderer_pointer_cannot_execute_canvas_lane.rs",
    );
}

#[test]
fn canvas_draw_hook_fields_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/canvas_draw_hook_fields_not_public.rs",
    );
}

#[test]
fn spatial_hit_test_hook_fields_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/spatial_hit_test_hook_fields_not_public.rs",
    );
}

#[test]
fn spatial_tool_state_hook_fields_are_not_publicly_mintable() {
    runtime_canvas_spatial_lane_compile_fail(
        "tests/ui/runtime_authority/fail/spatial_tool_state_hook_fields_not_public.rs",
    );
}
