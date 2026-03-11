use crate::tests::support::*;

#[test]
fn branch_creation_and_branch_targeted_commits_build_a_version_graph() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let main_second =
        create_entity_outcome_on_branch(&mut runtime, "main-b", BranchId("main".to_string()));
    let graph = runtime.version_graph();

    assert_eq!(
        runtime
            .branch_head(&BranchId("feature".to_string()))
            .unwrap(),
        &feature_outcome.commit
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())).unwrap(),
        &main_second.commit
    );
    assert_eq!(
        feature_outcome.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(
        main_second.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(graph.branches.len(), 2);
    assert_eq!(graph.commits.len(), 3);
}

#[test]
fn merge_commit_uses_deterministic_parent_order_and_advances_target_branch() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let merge_outcome = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    assert_eq!(
        merge_outcome.commit.parents,
        vec![
            main_outcome.commit.commit_id,
            feature_outcome.commit.commit_id
        ]
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())),
        Some(&merge_outcome.commit)
    );
    assert_eq!(
        runtime.branch_head(&BranchId("feature".to_string())),
        Some(&feature_outcome.commit)
    );
    let envelope = runtime
        .canonical_commit_envelope(merge_outcome.commit.commit_id)
        .unwrap();
    assert_eq!(
        envelope.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        envelope.merge_base_commits,
        vec![main_outcome.commit.commit_id]
    );
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeCommitPublished));
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeBaseResolved));
}

#[test]
fn merge_commit_requires_existing_parent_branch_heads() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let txn = runtime.begin_transaction(
        TransactionOptions::default().merge_from_branches(vec![BranchId("missing".to_string())]),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::InvalidMergeParent
    ));
}

#[test]
fn branch_history_helpers_expose_ancestor_and_merge_base_reasoning() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let chain = runtime.ancestor_chain(feature.commit.commit_id);
    let merge_base = runtime.latest_common_ancestor_between_branches(
        &BranchId("main".to_string()),
        &BranchId("feature".to_string()),
    );

    assert_eq!(chain, vec![main.commit.commit_id, feature.commit.commit_id]);
    assert_eq!(merge_base, Some(main.commit.commit_id));
    assert!(runtime.can_merge_branch_into(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string())
    ));
}

#[test]
fn merge_inspection_reports_overlapping_authority() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.inspect_merge(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string()),
    );

    assert_eq!(inspection.merge_base, Some(base.commit.commit_id));
    assert!(!inspection.can_merge);
    assert_eq!(
        inspection.conflicting_records,
        vec![crate::facade::MergeConflictRecord::Entity(shared)]
    );
}

#[test]
fn merge_commit_rejects_overlapping_authority_since_merge_base() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("main".to_string())),
        merge_parent_branches: vec![BranchId("feature".to_string())],
        ..TransactionOptions::default()
    });
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::MergeConflictOverlap
    ));
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::History)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeConflictOverlap));
}

#[test]
fn duplicate_branch_creation_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let error = runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap_err();

    assert_eq!(error, BranchCreateError::BranchAlreadyExists);
}

#[test]
fn chunked_storage_summary_tracks_visibility_boundaries() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let entity_a = changed_entities(&first)[0];
    let _second = create_entity_outcome(&mut runtime, "e1");
    let snapshot = runtime.snapshot();
    let _third = create_entity_outcome(&mut runtime, "e2");
    let _update = update_entity(&mut runtime, entity_a, "e0-updated");

    let summary_before_update = runtime.chunked_storage_summary(snapshot.version_id);
    let summary_current =
        runtime.chunked_storage_summary(runtime.latest_commit().unwrap().version_id);

    assert_eq!(summary_before_update.entity_chunks.len(), 2);
    assert_eq!(summary_before_update.entity_chunks[0].visible_records, 2);
    assert_eq!(summary_before_update.entity_chunks[1].visible_records, 0);
    assert_eq!(summary_current.entity_chunks[1].visible_records, 1);
    assert_eq!(summary_current.entity_chunks[0].slot_len, 2);
}

#[test]
fn chunk_diagnostics_and_packet_plans_are_public_and_stable() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let second = create_entity_outcome(&mut runtime, "e1");
    let entity_a = changed_entities(&first)[0];
    let entity_b = changed_entities(&second)[0];
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk(
        "pair",
        vec![RecordRef::Entity(entity_a), RecordRef::Entity(entity_b)],
    );

    let plan = runtime.plan_read_packet(&snapshot, &packet).unwrap();
    let diagnostics = runtime.chunk_diagnostics(snapshot.version_id);

    assert_eq!(plan.target_count, 2);
    assert_eq!(plan.entity_chunk_indexes, vec![0]);
    assert_eq!(diagnostics.entity_chunks_total, 1);
    assert_eq!(diagnostics.entity_chunks_with_visible_records, 1);
}
