#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn inspection_boundary_compile_failures_prevent_forged_receipts_and_postures() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_inspection_receipts.rs");
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_inspection_receipts_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_witnesses.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_via_variant_literal.rs",
    );
}

