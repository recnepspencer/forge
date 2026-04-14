#[test]
fn query_phase_boundaries_are_compile_time_private() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
