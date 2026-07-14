use super::*;

#[test]
fn local_file_reopen_backfills_legacy_empty_branch_delta_artifacts() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-artifact-backfill");

    let target_commit_id = {
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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "legacy-artifact-backfill",
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        let target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
        target_commit_id
    };

    force_clear_branch_delta_layer_artifacts(&path);
    let store = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let direct = admitted_branch_delta_read(&store, feature_branch, target_commit_id);

    assert_eq!(
        direct.plan.performance.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(direct.plan.used_layer_ids.len(), 1);
}

#[test]
fn local_file_reopen_rejects_branch_delta_artifact_commit_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-artifact-mismatch");

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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "artifact-mismatch",
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        store.append_canonical_commit(feature_commit).unwrap();
    }

    force_branch_delta_artifact_commit_mismatch(&path);
    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaPublicationGap);
}
#[test]
fn local_file_reopen_rejects_replacement_self_reference_corruption() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-replacement-self");

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
                &format!("replacement-self-{index}"),
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

    force_branch_delta_replacement_self_reference(&path);
    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::BranchDeltaShadowAuthorityViolation
    );
}
#[test]
fn local_file_reopen_rejects_replacement_gap_corruption() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-replacement-gap");

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
                &format!("replacement-gap-{index}"),
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

    force_branch_delta_replacement_gap(&path);
    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}
