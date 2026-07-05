#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn public_boundaries_reject_forbidden_host_authority_and_runtime_helper_bypasses() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/measurement_boundary_purity/forbidden_measurement_request_family_variant.rs",
    );
    tests.compile_fail(
        "tests/ui/measurement_boundary_purity/forbidden_measurement_request_constructor.rs",
    );
    tests.compile_fail(
        "tests/ui/measurement_boundary_purity/runtime_evidence_root_is_not_public.rs",
    );
}
