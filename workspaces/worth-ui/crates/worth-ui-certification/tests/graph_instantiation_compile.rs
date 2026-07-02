#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn graph_instantiation_compile_failures_keep_admission_boundary_narrow() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/graph_instantiation/external_callers_cannot_construct_or_substitute_graph_instantiation_plan.rs",
    );
}
