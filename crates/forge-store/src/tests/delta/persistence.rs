use super::*;

#[test]
fn sqlite_rewrite_persists_after_reopen() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_sqlite_path("forge-store-delta-rewrite");

    let target_commit_id = {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
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
        for index in 0..2 {
            update_entity_on_branch(
                &mut runtime,
                entity_id,
                &format!("sqlite-rewrite-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }

        store
            .rewrite_branch_delta(
                store
                    .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
                        feature_branch.clone(),
                        target_commit_id,
                    ))
                    .unwrap(),
            )
            .unwrap();
        target_commit_id
    };

    let store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(plan.used_layer_ids.len(), 1);
}
