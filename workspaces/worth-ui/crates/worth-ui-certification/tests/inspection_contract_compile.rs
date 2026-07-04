#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn inspection_contract_enforces_shared_callers_and_sealed_receipts() {
    let tests = trybuild_support::new_test_cases();
    tests.pass("tests/ui/inspection/ai_and_human_callers_share_inspection_contract.rs");
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_inspection_receipts.rs");
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_inspection_receipts_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/exhaustive_matching_over_public_inspection_contract_enums_is_forbidden.rs",
    );
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_import_receipt_projection_helper.rs",
    );
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_import_runtime_evidence_authority_from_inspection.rs");
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_obligation_reason_projection.rs",
    );
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_via_struct_literal.rs");
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_via_variant_literal.rs");
    tests.compile_fail(
        "tests/ui/inspection/external_callers_cannot_mint_unsupported_posture_witnesses.rs",
    );
    tests.compile_fail("tests/ui/inspection/facade_callers_cannot_mint_evidence_identity.rs");
}
