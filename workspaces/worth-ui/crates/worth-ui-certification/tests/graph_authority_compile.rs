#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn graph_authority_compile_failures_prevent_forged_runtime_identity_or_snapshot() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/graph_authority/external_callers_cannot_forge_graph_identity_or_snapshot.rs",
    );
    tests.compile_fail(
        "tests/ui/graph_authority/external_callers_cannot_mint_graph_successor_from_snapshot.rs",
    );
    tests.compile_fail(
        "tests/ui/graph_authority/external_callers_cannot_import_raw_graph_runtime_internals_from_runtime_facade.rs",
    );
}
