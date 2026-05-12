#[test]
fn phase1_internal_boundaries_are_private() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/phase1/*.rs");
}
