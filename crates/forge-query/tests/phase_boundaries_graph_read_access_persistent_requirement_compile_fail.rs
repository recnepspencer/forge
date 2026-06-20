#[test]
fn graph_read_access_persistent_requirement_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/declaration_constructor_private.rs",
    );
    t.compile_fail("tests/ui/graph_read_access_persistent_requirement/row_constructor_private.rs");
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/receipt_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/counters_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/family_index_contract_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/raw_index_name_on_declaration_forbidden.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_persistent_requirement/raw_index_name_on_family_contract_forbidden.rs",
    );
}
