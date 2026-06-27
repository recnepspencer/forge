#[path = "trybuild_support.rs"]
mod trybuild_support;
fn steady_frame_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn steady_frame_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn steady_frame_counter_boundary_facade_types_compile() {
    steady_frame_compile_pass(
        "tests/ui/runtime_steady_frame_counter_boundary/pass/steady_frame_counter_boundary_facade_types.rs",
    );
}

#[test]
fn steady_frame_receipt_fields_not_public() {
    steady_frame_compile_fail(
        "tests/ui/runtime_steady_frame_counter_boundary/fail/steady_frame_receipt_fields_not_public.rs",
    );
}

#[test]
fn raw_steady_frame_receipt_cannot_lower_to_foundational() {
    steady_frame_compile_fail(
        "tests/ui/runtime_steady_frame_counter_boundary/fail/raw_steady_frame_receipt_cannot_lower_to_foundational.rs",
    );
}

