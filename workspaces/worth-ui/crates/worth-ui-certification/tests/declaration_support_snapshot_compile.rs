#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_support_snapshot_compile_failures_prevent_forgery_or_promotion() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_support/external_callers_cannot_construct_or_promote_declaration_support.rs",
    );
}
