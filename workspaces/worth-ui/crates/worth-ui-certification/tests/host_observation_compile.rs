#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn host_observation_compile_failures_prevent_direct_measurement_result_minting() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/measurement_authority/external_callers_cannot_mint_measurement_result_via_from_host_observation.rs",
    );
}
