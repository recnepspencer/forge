#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_declared_posture_compile_failures_prevent_contract_forgery_or_type_promotion() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_posture/external_callers_cannot_construct_declared_posture_contracts.rs",
    );
    // No distinct public service-execution surface is exported yet, so this compile lane
    // proves the strongest real public promotion boundaries that currently exist.
    tests.compile_fail(
        "tests/ui/declaration_posture/declared_posture_cannot_promote_to_runtime_receipts.rs",
    );
}
