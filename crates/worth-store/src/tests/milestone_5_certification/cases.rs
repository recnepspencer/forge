use super::*;

#[test]
fn milestone_5_certification_bundle_proves_no_edit_branch_is_near_free() {
    let bundle = no_edit_bundle();

    assert_eq!(
        bundle.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::EmptyBranchReuse
    );
    assert_eq!(
        bundle.delta_storage_report.control_path.strategy,
        BranchDeltaReadStrategy::AuthorityReplayControl
    );
    assert_eq!(bundle.delta_storage_report.live_layer_count, 0);
    assert_eq!(bundle.counter_snapshot.branch_base_reuse_count, 1);
    assert_eq!(bundle.counter_snapshot.branch_base_copy_count, 0);
    assert_eq!(
        bundle
            .counter_snapshot
            .branch_hidden_full_base_materialization_count,
        0
    );
    assert_eq!(
        bundle.delta_storage_report.direct_path.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        bundle.delta_storage_report.control_reference_surface,
        "Milestone7IndependentReference"
    );
}

#[test]
fn milestone_5_certification_bundle_matches_backend_variation_parity() {
    let suite = milestone_5_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
}

#[test]
fn milestone_5_certification_bundle_proves_delta_growth_tracks_semantic_delta() {
    let suite = milestone_5_suite();
    assert_any_not_equal(&suite.canonical_rows()[1]);

    let no_edit = no_edit_bundle();
    let small = small_edit_bundle_in_memory();
    let deep = deep_edit_bundle();

    assert_eq!(no_edit.delta_storage_report.live_layer_count, 0);
    assert_eq!(small.delta_storage_report.live_layer_count, 1);
    assert!(deep.delta_storage_report.live_layer_count >= 3);
    assert_eq!(
        small.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::DirectLayerRead
    );
    assert_eq!(
        deep.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::DirectLayerRead
    );
}

#[test]
fn milestone_5_certification_bundle_proves_rewritten_stack_control_lane_parity() {
    let suite = milestone_5_suite();
    assert_all_equal(&suite.canonical_rows()[2]);

    let rewritten = rewritten_bundle_in_memory();
    assert_eq!(rewritten.delta_storage_report.live_layer_count, 1);
    assert_eq!(
        rewritten
            .counter_snapshot
            .branch_delta_hidden_full_stack_rewrite_count,
        0
    );
    assert_eq!(rewritten.counter_snapshot.branch_delta_rewrite_count, 1);
}

#[test]
fn milestone_5_certification_suite_is_complete() {
    let suite = milestone_5_suite();
    let completeness =
        evaluate_completeness(&suite, &BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}
