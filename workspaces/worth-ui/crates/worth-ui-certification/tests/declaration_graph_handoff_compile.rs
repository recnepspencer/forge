#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_graph_handoff_compile_failures_prevent_raw_or_support_substitutes() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_graph_handoff/external_callers_cannot_construct_or_substitute_graph_handoff.rs",
    );
}
