#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

const RUNTIME_LANE_PASS_CASES: &[&str] = &[
    "tests/ui/runtime_authority/pass/runtime_canvas_spatial_lane_facade_types.rs",
    "tests/ui/runtime_authority/pass/runtime_ordinary_lane_facade_types.rs",
    "tests/ui/runtime_authority/pass/runtime_realtime_overlay_lane_facade_types.rs",
    "tests/ui/runtime_authority/pass/runtime_virtualized_data_lane_facade_types.rs",
];

const RUNTIME_LANE_FAIL_CASES: &[&str] = &[
    "tests/ui/runtime_authority/fail/canvas_spatial_plan_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_domain_geometry_cannot_execute_canvas_lane.rs",
    "tests/ui/runtime_authority/fail/ordinary_lane_plan_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_plan_input_cannot_execute_ordinary_lane.rs",
    "tests/ui/runtime_authority/fail/realtime_overlay_hook_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_renderer_pointer_cannot_execute_realtime_lane.rs",
    "tests/ui/runtime_authority/fail/virtualized_data_plan_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/raw_query_string_cannot_execute_virtualized_data_lane.rs",
    "tests/ui/runtime_authority/fail/lane_admission_fields_not_public.rs",
    "tests/ui/runtime_authority/fail/private_component_lane_string_cannot_enter_lane_admission.rs",
];

#[test]
fn runtime_lane_public_types_compile() {
    trybuild_helpers::run_pass_cases(RUNTIME_LANE_PASS_CASES);
}

#[test]
fn runtime_lane_boundaries_stay_sealed() {
    trybuild_helpers::run_compile_fail_cases(RUNTIME_LANE_FAIL_CASES);
}
