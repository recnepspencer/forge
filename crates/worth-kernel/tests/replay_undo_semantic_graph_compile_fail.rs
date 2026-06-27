const FIXTURES: &[&str] = &[
    "tests/fixtures/replay_undo_semantic_graph/topology_replay_scope_not_from_spatial_lookup_receipt.rs",
    "tests/fixtures/replay_undo_semantic_graph/spatial_replay_scope_not_from_raw_stage_index_identity.rs",
    "tests/fixtures/replay_undo_semantic_graph/spatial_replay_prepared_request_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/lookup_consumed_workload_handoff_not_hand_filled.rs",
];

#[test]
fn replay_undo_semantic_graph_public_lowering_surfaces_reject_wrong_authority() {
    let trybuild = trybuild::TestCases::new();
    for fixture in FIXTURES {
        trybuild.compile_fail(fixture);
    }
}
