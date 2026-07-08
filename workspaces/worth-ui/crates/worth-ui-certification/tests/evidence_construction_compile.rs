#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn external_callers_cannot_import_evidence_construction_mint_helpers() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/evidence_construction/external_callers_cannot_import_evidence_identity_helper.rs",
    );
    tests.compile_fail(
        "tests/ui/evidence_construction/external_callers_cannot_import_evidence_ref_helper.rs",
    );
    tests.compile_fail(
        "tests/ui/evidence_construction/external_callers_cannot_import_evidence_slice_helper.rs",
    );
}