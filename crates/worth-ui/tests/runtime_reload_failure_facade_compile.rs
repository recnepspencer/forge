#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

#[test]
fn runtime_reload_failure_public_types_compile() {
    trybuild_helpers::run_pass_cases(&[
        "tests/ui/runtime_reload_failure/pass/reload_failure_facade_types.rs",
    ]);
}

#[test]
fn runtime_reload_failure_boundary_stays_sealed() {
    trybuild_helpers::run_compile_fail_cases(&[
        "tests/ui/runtime_reload_failure/fail/reload_failure_fields_not_public.rs",
        "tests/ui/runtime_reload_failure/fail/preservation_receipt_fields_not_public.rs",
        "tests/ui/runtime_reload_failure/fail/fallback_runtime_constructor_not_public.rs",
    ]);
}
