use super::*;

#[test]
fn sqlite_direct_layer_read_matches_authoritative_snapshot_after_reopen() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_sqlite_path("worth-store-delta-parity");

    let feature_target_commit_id = {
        let mut store = WORTHStoreBuilder::new()
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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "feature-one",
            Some(feature_branch.clone()),
        );
        let feature_one = latest_envelope(&runtime);
        store.append_canonical_commit(feature_one).unwrap();

        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "feature-two",
            Some(feature_branch.clone()),
        );
        let feature_two = latest_envelope(&runtime);
        let feature_target_commit_id = feature_two.commit.commit_id;
        store.append_canonical_commit(feature_two).unwrap();
        feature_target_commit_id
    };

    let mut store = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let direct =
        admitted_branch_delta_read(&store, feature_branch.clone(), feature_target_commit_id);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            feature_branch.clone(),
            feature_target_commit_id,
        ))
        .unwrap();
    let authoritative = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            feature_target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        direct.authoritative_export().canonical_json(),
        authoritative.image.authoritative_export().canonical_json()
    );
}
