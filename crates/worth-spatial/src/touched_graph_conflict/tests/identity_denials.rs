#[test]
fn raw_strings_and_operation_labels_cannot_mint_spatial_conflict_family_identity() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail(
        "src/touched_graph_conflict/tests/ui/spatial_conflict_family_identity_denials.rs",
    );
}
