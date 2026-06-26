fn runtime_reload_storm_certification_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

#[test]
fn reload_storm_certification_facade_types_are_importable() {
    runtime_reload_storm_certification_pass(
        "tests/ui/runtime_reload_storm_certification/pass/reload_storm_certification_facade_types.rs",
    );
}
