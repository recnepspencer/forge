#[test]
fn workflow_authority_artifacts_are_sealed() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/declarative_workflow/*.rs");
}
