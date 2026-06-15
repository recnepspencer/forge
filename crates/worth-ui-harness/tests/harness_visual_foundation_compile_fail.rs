#[test]
fn raw_visual_values_are_rejected_from_harness_shell_source() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/raw_visual_values_forbidden.rs");
}

#[test]
fn raw_measurement_values_are_rejected_from_harness_density() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/raw_measurement_values_forbidden.rs");
}

#[test]
fn local_runtime_status_visuals_cannot_replace_query_outcome_projection() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/fail/local_runtime_status_visual_forbidden.rs");
}

#[test]
fn default_visual_foundation_uses_public_facade() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/pass/default_visual_foundation_uses_public_facade.rs");
}
