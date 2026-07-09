use super::*;

pub(super) fn run_strategy_merge_certification() -> StrategyCertificationBundle {
    let root_path = unique_test_store_path("worth-relational-strategy-cert");
    let recovered_root = root_path.clone();
    let mut runtime = persisted_strategy_runtime(root_path);
    let entity = create_entity(&mut runtime, "service");
    let feature_branch = create_branch_from_main(&mut runtime, "strategy-feature");
    let aspect_overlap_branch = create_branch_from_main(&mut runtime, "aspect-overlap-feature");
    let aspect_disjoint_branch = create_branch_from_main(&mut runtime, "aspect-disjoint-feature");

    let primary_conflict = merge_strategy_conflict::certify_primary_strategy_conflict(
        &mut runtime,
        entity,
        &feature_branch,
    );
    merge_aspect_conflicts::certify_overlapping_aspect_strategy_conflict(
        &mut runtime,
        &aspect_overlap_branch,
    );
    merge_aspect_conflicts::certify_disjoint_aspect_strategy_truth(
        &mut runtime,
        &aspect_disjoint_branch,
    );
    let controller_sequence =
        controller_sequence::certify_controller_sequence_shared_truth(&mut runtime);

    let main_commit = primary_conflict.main_commit;
    let feature_commit = primary_conflict.feature_commit;
    let controller_sequence_branch = controller_sequence.branch;
    let controller_feature_idempotent_commit = controller_sequence.idempotent_commit;

    let replacement_certification = run_replacement_strategy_certification();

    replay_recovery::certify_replay_recovery(replay_recovery::StrategyReplayRecoveryInput {
        runtime,
        recovered_root,
        entity,
        feature_branch,
        aspect_overlap_branch,
        aspect_disjoint_branch,
        controller_sequence_branch,
        main_commit,
        feature_commit,
        controller_feature_idempotent_commit,
        replacement_certification,
    })
}
