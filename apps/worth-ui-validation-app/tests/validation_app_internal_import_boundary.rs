#[test]
fn validation_app_public_facade_compile_passes() {
    trybuild::TestCases::new().pass("tests/ui/pass/validation_app_public_facade_launch.rs");
}

#[test]
fn validation_app_cannot_import_internal_runtime_or_registry_modules() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/validation_app_internal_runtime_import.rs");
    tests.compile_fail("tests/ui/fail/validation_app_internal_registry_import.rs");
    tests.compile_fail("tests/ui/fail/validation_app_internal_active_plan_import.rs");
    tests.compile_fail("tests/ui/fail/validation_app_internal_diagnostics_import.rs");
}

#[test]
fn validation_app_cannot_premint_receipts_or_visual_success() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/validation_app_preminted_run_receipt.rs");
    tests.compile_fail("tests/ui/fail/validation_app_visual_only_success.rs");
    tests.compile_fail("tests/ui/fail/validation_app_local_shell_state_injection.rs");
}
