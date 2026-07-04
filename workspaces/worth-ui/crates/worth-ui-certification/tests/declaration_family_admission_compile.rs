#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_family_compile_failures_prevent_family_wrapper_forgery() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_family/external_callers_cannot_construct_family_wrappers.rs",
    );
}
