fn runtime_reload_counter_boundary_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn runtime_reload_counter_boundary_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn reload_counter_boundary_facade_types_are_importable() {
    runtime_reload_counter_boundary_pass(
        "tests/ui/runtime_reload_counter_boundary/pass/reload_counter_boundary_facade_types.rs",
    );
}

#[test]
fn reload_counter_receipt_fields_are_not_publicly_mintable() {
    runtime_reload_counter_boundary_fail(
        "tests/ui/runtime_reload_counter_boundary/fail/reload_counter_receipt_fields_not_public.rs",
    );
}

#[test]
fn raw_reload_counter_receipt_cannot_lower_to_foundational() {
    runtime_reload_counter_boundary_fail(
        "tests/ui/runtime_reload_counter_boundary/fail/raw_reload_counter_receipt_cannot_lower_to_foundational.rs",
    );
}
