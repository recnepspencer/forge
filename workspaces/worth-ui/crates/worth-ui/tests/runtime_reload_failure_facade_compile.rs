fn runtime_reload_failure_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

fn runtime_reload_failure_fail(path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[test]
fn reload_failure_facade_types_are_importable() {
    runtime_reload_failure_pass(
        "tests/ui/runtime_reload_failure/pass/reload_failure_facade_types.rs",
    );
}

#[test]
fn reload_failure_fields_are_not_publicly_mintable() {
    runtime_reload_failure_fail(
        "tests/ui/runtime_reload_failure/fail/reload_failure_fields_not_public.rs",
    );
}

#[test]
fn preservation_receipt_fields_are_not_publicly_mintable() {
    runtime_reload_failure_fail(
        "tests/ui/runtime_reload_failure/fail/preservation_receipt_fields_not_public.rs",
    );
}

#[test]
fn reload_failure_counters_are_not_publicly_mintable() {
    runtime_reload_failure_fail(
        "tests/ui/runtime_reload_failure/fail/reload_failure_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn fallback_runtime_failure_constructor_is_not_public_api() {
    runtime_reload_failure_fail(
        "tests/ui/runtime_reload_failure/fail/fallback_runtime_constructor_not_public.rs",
    );
}
