#[test]
fn history_and_comparison_typestate_boundaries_are_compile_time_enforced() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/declarative_history_comparison/*.rs");
}
