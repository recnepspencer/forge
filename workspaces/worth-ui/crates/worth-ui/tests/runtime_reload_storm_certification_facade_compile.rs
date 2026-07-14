#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_reload_storm_certification_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn reload_storm_certification_types_are_not_public_facade_api() {
    runtime_reload_storm_certification_compile_fail(
        "tests/ui/runtime_reload_storm_certification/fail/reload_storm_certification_not_public_facade_api.rs",
    );
}
