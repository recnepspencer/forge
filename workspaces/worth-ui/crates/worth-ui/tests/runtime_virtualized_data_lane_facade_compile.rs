#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_virtualized_data_lane_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_virtualized_data_lane_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn virtualized_data_lane_facade_types_are_visible() {
    runtime_virtualized_data_lane_compile_pass(
        "tests/ui/runtime_authority/pass/runtime_virtualized_data_lane_facade_types.rs",
    );
}

#[test]
fn virtualized_data_plan_fields_are_not_publicly_mintable() {
    runtime_virtualized_data_lane_compile_fail(
        "tests/ui/runtime_authority/fail/virtualized_data_plan_fields_not_public.rs",
    );
}

#[test]
fn virtualized_data_frame_receipt_fields_are_not_publicly_mintable() {
    runtime_virtualized_data_lane_compile_fail(
        "tests/ui/runtime_authority/fail/virtualized_data_frame_receipt_fields_not_public.rs",
    );
}

#[test]
fn virtualized_data_counters_are_not_publicly_mintable() {
    runtime_virtualized_data_lane_compile_fail(
        "tests/ui/runtime_authority/fail/virtualized_data_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn visible_range_offset_pagination_constructor_is_missing() {
    runtime_virtualized_data_lane_compile_fail(
        "tests/ui/runtime_authority/fail/visible_range_offset_pagination_constructor_missing.rs",
    );
}

#[test]
fn raw_query_string_cannot_execute_virtualized_data_lane() {
    runtime_virtualized_data_lane_compile_fail(
        "tests/ui/runtime_authority/fail/raw_query_string_cannot_execute_virtualized_data_lane.rs",
    );
}

