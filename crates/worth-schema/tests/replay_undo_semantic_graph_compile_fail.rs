const FIXTURES: &[&str] = &[
    "tests/fixtures/replay_undo_semantic_graph/public_replay_undo_not_from_raw_prior_proof_identity.rs",
    "tests/fixtures/replay_undo_semantic_graph/public_replay_undo_not_from_raw_stage_index_identity.rs",
];

#[test]
fn replay_undo_semantic_graph_public_facade_rejects_raw_identity_minting() {
    let trybuild = trybuild::TestCases::new();
    for fixture in FIXTURES {
        trybuild.compile_fail(fixture);
    }
}
