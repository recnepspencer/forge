#[path = "trybuild_support.rs"]
mod trybuild_support;
fn lane_frame_cost_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

#[test]
fn lane_frame_cost_certification_facade_types_compile() {
    lane_frame_cost_compile_pass(
        "tests/ui/runtime_lane_frame_cost_certification/pass/lane_frame_cost_certification_facade_types.rs",
    );
}
