#[test]
fn harness_cannot_import_internal_runtime_or_registry_modules() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/internal_runtime_import_forbidden.rs");
    tests.compile_fail("tests/ui/fail/internal_registry_import_forbidden.rs");
    tests.compile_fail("tests/ui/fail/internal_active_plan_import_forbidden.rs");
    tests.compile_fail("tests/ui/fail/internal_diagnostics_import_forbidden.rs");
}

#[test]
fn harness_rejects_app_local_shell_state_injection_by_construction() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/app_local_shell_state_injection_forbidden.rs");
}

#[test]
fn harness_result_requires_runtime_receipts_not_public_evidence_mutation() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/visual_success_without_runtime_receipts_forbidden.rs");
}

#[test]
fn harness_public_facade_launch_compiles() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/pass/public_facade_harness_launch.rs");
}
