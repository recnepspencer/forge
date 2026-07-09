#[test]
fn query_phase_boundaries_enforce_absent_internal_runtime_builder_methods() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
