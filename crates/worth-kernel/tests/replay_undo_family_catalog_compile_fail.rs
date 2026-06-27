const COMPILE_FAIL_FIXTURES: &[&str] = &[
    "tests/fixtures/replay_undo_family_catalog/kernel_replay_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/kernel_undo_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/kernel_replay_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/kernel_undo_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_replay_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_replay_declaration_input_missing_scope_product.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_replay_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_replay_identity_not_from_wrong_authority.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_undo_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_undo_declaration_input_missing_scope_product.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_undo_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/spatial_undo_identity_not_from_wrong_authority.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_replay_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_replay_declaration_input_missing_scope_product.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_replay_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_replay_identity_not_from_wrong_authority.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_undo_declaration_not_hand_filled.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_undo_declaration_input_missing_scope_product.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_undo_identity_not_from_raw_string.rs",
    "tests/fixtures/replay_undo_family_catalog/topology_undo_identity_not_from_wrong_authority.rs",
];

#[test]
fn replay_undo_family_catalog_public_surfaces_reject_forgery() {
    let test_cases = trybuild::TestCases::new();

    for fixture in COMPILE_FAIL_FIXTURES {
        test_cases.compile_fail(*fixture);
    }
}
