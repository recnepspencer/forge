#[test]
fn invariant_ui_boundaries_are_sealed() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/invariants/*.rs");
}
