const FIXTURES: &[&str] = &[
    "tests/fixtures/replay_undo_semantic_graph/topology_replay_scope_not_from_spatial_lookup_receipt.rs",
    "tests/fixtures/replay_undo_semantic_graph/spatial_replay_scope_not_from_raw_stage_index_identity.rs",
    "tests/fixtures/replay_undo_semantic_graph/spatial_replay_prepared_request_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/lookup_consumed_workload_handoff_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_forbidden_surface_firewall_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_forbidden_surface_firewall_struct_literals_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/packetless_chain_helper_not_public_authority.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_hard_deletion_closeout_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_public_closeout_not_hand_filled.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_public_closeout_rejects_raw_digest.rs",
    "tests/fixtures/replay_undo_semantic_graph/replay_undo_milestone_thirteen_seed_not_hand_filled.rs",
];

#[test]
fn replay_undo_semantic_graph_public_lowering_surfaces_reject_wrong_authority() {
    let trybuild = trybuild::TestCases::new();
    for fixture in FIXTURES {
        trybuild.compile_fail(fixture);
    }
}
