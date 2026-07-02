#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn obligation_dispatch_compile_failures_prevent_plan_and_verdict_forgery() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/obligation_dispatch/external_callers_cannot_import_dispatch_boundary.rs",
    );
    tests.compile_fail(
        "tests/ui/obligation_dispatch/external_callers_cannot_mint_dispatch_plans_or_verdicts.rs",
    );
}
