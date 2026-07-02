#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_structural_semantics_compile_failures_prevent_handoff_authority_bypass() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_structural/external_callers_cannot_construct_structural_semantics_or_handoff.rs",
    );
}
