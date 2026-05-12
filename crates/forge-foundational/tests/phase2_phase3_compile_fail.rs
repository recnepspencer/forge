#[test]
fn phase2_phase3_type_boundaries_are_private() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/phase2_phase3/*.rs");
}
