#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn facade_export_compile_failures_block_host_bypass_and_certify_root_leakage() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/facade_export/crate_root_does_not_bypass_host_observation_facade.rs",
    );
    tests.compile_fail(
        "tests/ui/facade_export/runtime_facade_root_does_not_export_certify_suites.rs",
    );
    tests.compile_fail(
        "tests/ui/facade_export/runtime_facade_root_does_not_export_runtime_host.rs",
    );
}