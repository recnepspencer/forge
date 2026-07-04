#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_authority_compile_failures_prevent_forged_artifacts_and_receipts() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail("tests/ui/declaration/external_callers_cannot_mint_declaration_artifact.rs");
    tests.compile_fail("tests/ui/declaration/external_callers_cannot_mint_declaration_identity.rs");
    tests.compile_fail(
        "tests/ui/declaration/external_callers_cannot_construct_semantic_artifact.rs",
    );
    tests.compile_fail("tests/ui/declaration/external_callers_cannot_mint_dsl_lowering_receipt.rs");
    tests.compile_fail(
        "tests/ui/declaration/external_callers_cannot_seed_dsl_package_with_semantic_artifact.rs",
    );
    tests.compile_fail("tests/ui/declaration/facade_does_not_export_declaration_lowering.rs");
    tests.compile_fail("tests/ui/declaration/facade_does_not_export_semantic_artifact.rs");
}
