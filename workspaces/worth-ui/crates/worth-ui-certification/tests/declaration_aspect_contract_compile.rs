#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_aspect_contract_compile_failures_prevent_stringly_authority_bypass() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_aspect/external_callers_cannot_construct_aspect_contracts.rs",
    );
}
