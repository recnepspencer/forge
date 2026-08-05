#[test]
fn mutation_precondition_compiler_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail(
        "tests/ui/application_schema/mutation_precondition_requires_declared_family.rs",
    );
    cases.compile_fail(
        "tests/ui/application_schema/mutation_precondition_requires_matching_scope.rs",
    );
}
