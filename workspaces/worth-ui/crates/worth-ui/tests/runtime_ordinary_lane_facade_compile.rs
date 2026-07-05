#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_ordinary_lane_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_ordinary_lane_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn ordinary_lane_facade_types_are_visible() {
    runtime_ordinary_lane_compile_pass(
        "tests/ui/runtime_authority/pass/runtime_ordinary_lane_facade_types.rs",
    );
}

#[test]
fn ordinary_lane_plan_fields_are_not_publicly_mintable() {
    runtime_ordinary_lane_compile_fail(
        "tests/ui/runtime_authority/fail/ordinary_lane_plan_fields_not_public.rs",
    );
}

#[test]
fn ordinary_lane_frame_receipt_fields_are_not_publicly_mintable() {
    runtime_ordinary_lane_compile_fail(
        "tests/ui/runtime_authority/fail/ordinary_lane_frame_receipt_fields_not_public.rs",
    );
}

#[test]
fn ordinary_lane_counters_are_not_publicly_mintable() {
    runtime_ordinary_lane_compile_fail(
        "tests/ui/runtime_authority/fail/ordinary_lane_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn ordinary_frame_target_cannot_be_built_from_component_string() {
    runtime_ordinary_lane_compile_fail(
        "tests/ui/runtime_authority/fail/ordinary_frame_target_string_constructor_missing.rs",
    );
}

#[test]
fn raw_execution_plan_input_cannot_enter_ordinary_lane_execution() {
    runtime_ordinary_lane_compile_fail(
        "tests/ui/runtime_authority/fail/raw_plan_input_cannot_execute_ordinary_lane.rs",
    );
}
