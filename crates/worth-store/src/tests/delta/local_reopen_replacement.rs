use super::*;

#[test]
fn local_file_reopen_rejects_replacement_lineage_proof_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-replacement-proof-mismatch");

    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store.append_canonical_commit(main_head.clone()).unwrap();
        store
            .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
                feature_branch.clone(),
                main_branch.clone(),
            ))
            .unwrap();
        runtime
            .history_authority()
            .create_branch(feature_branch.clone(), &main_branch)
            .unwrap();

        let mut target_commit_id = main_head.commit.commit_id;
        for index in 0..RECOMMENDED_REWRITE_LAYER_WIDTH {
            update_entity_on_branch(
                &mut runtime,
                entity_id,
                &format!("replacement-proof-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_proof_mismatch(&path);
    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}

#[test]
fn local_file_reopen_rejects_replacement_lineage_proof_length_drift() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-replacement-proof-length-drift");

    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(root).unwrap();
        store.append_canonical_commit(main_head.clone()).unwrap();
        store
            .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
                feature_branch.clone(),
                main_branch.clone(),
            ))
            .unwrap();
        runtime
            .history_authority()
            .create_branch(feature_branch.clone(), &main_branch)
            .unwrap();

        let mut target_commit_id = main_head.commit.commit_id;
        for index in 0..RECOMMENDED_REWRITE_LAYER_WIDTH {
            update_entity_on_branch(
                &mut runtime,
                entity_id,
                &format!("replacement-proof-length-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_proof_length_drift(&path);
    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}
