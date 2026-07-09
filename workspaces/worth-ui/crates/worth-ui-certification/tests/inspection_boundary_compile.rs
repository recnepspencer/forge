#[path = "trybuild_support.rs"]
mod trybuild_support;
#[test]
fn inspection_boundary_compile_failures_prevent_worthd_receipts_postures_and_helper_bypasses() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_inspection_receipts.rs");
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_inspection_receipts_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_import_receipt_projection_helper.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_import_runtime_evidence_authority_from_inspection.rs",
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
    tests.compile_fail("tests/ui/inspection/facade_callers_cannot_mint_evidence_identity.rs");
}
