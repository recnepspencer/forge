#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn external_callers_cannot_mint_measurement_basis_or_import_bypass_helpers() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/measurement_authority/external_callers_cannot_mint_measurement_basis_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/measurement_authority/external_callers_cannot_mint_projection_fact_receipt_via_struct_literal.rs",
    );
    tests.compile_fail(
        "tests/ui/measurement_authority/external_callers_cannot_import_projection_fact_admission_helper.rs",
    );
    tests.compile_fail(
        "tests/ui/measurement_authority/external_callers_cannot_import_measurement_test_support.rs",
    );
}
