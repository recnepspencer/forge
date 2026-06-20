#[test]
fn graph_read_access_ephemeral_public_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/graph_read_access_ephemeral/scope_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_ephemeral/plan_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_ephemeral/active_index_constructor_private.rs");
    t.compile_fail(
        "tests/ui/graph_read_access_ephemeral/lifecycle_registry_constructor_private.rs",
    );
    t.compile_fail("tests/ui/graph_read_access_ephemeral/allocation_row_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_ephemeral/receipt_constructor_private.rs");
    t.compile_fail("tests/ui/graph_read_access_ephemeral/counters_constructor_private.rs");
}
