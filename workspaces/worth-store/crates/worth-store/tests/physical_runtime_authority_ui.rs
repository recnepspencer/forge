#[test]
fn external_consumers_cannot_forge_or_duplicate_runtime_authority() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/physical_runtime_authority/supported_admission.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/runtime_duplication_and_reconstruction_are_sealed.rs",
    );
    cases.compile_fail(
        "tests/physical_runtime_authority/internal_composition_construction_is_sealed.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/internal_runtime_topology_is_sealed.rs");
    cases.compile_fail("tests/physical_runtime_authority/non_authority_values_cannot_admit.rs");
    cases.compile_fail(
        "tests/physical_runtime_authority/wrong_phase_and_physical_operations_are_absent.rs",
    );
    cases.compile_fail("tests/physical_runtime_authority/maximal_feature_profile_cannot_admit.rs");
}
