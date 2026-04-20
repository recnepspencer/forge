use super::*;
use crate::tests::milestone_6_certification::suite_helpers::*;

pub(super) fn dedup_control_overlap_branch_parity() -> Vec<LaneResult<String>> {
    [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite]
        .into_iter()
        .map(|lane| {
            let mut runtime = runtime_with_demo_schema();
            let entity_id = create_entity(&mut runtime, "alpha");
            let root = latest_envelope(&runtime);
            update_entity_on_branch(&mut runtime, entity_id, "beta", None);
            let main_head = latest_envelope(&runtime);
            let main_branch = main_head.branch_context.clone();
            let feature_branch = BranchId("m6-suite-dedup-feature".to_string());

            let mut store = match lane {
                StoreLane::InMemory => ForgeStoreBuilder::new().in_memory().build().unwrap(),
                _ => build_store_for_lane(
                    lane,
                    &format!("milestone-6-dedup-overlap-{}", lane.label()),
                ),
            };
            store.append_canonical_commit(root).unwrap();
            store.append_canonical_commit(main_head.clone()).unwrap();
            store
                .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
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
                "feature-only",
                Some(feature_branch.clone()),
            );
            let feature_head = latest_envelope(&runtime);
            let request = request_for_scope(
                &feature_head,
                AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                    "entity-a".to_string(),
                    "entity-b".to_string(),
                ])),
                &["profile", "status"],
            );
            store.append_canonical_commit(feature_head).unwrap();
            store
                .materialize_milestone_6_layout_support(request.clone())
                .unwrap();

            let aspect_read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
                crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
                other => panic!("expected admitted overlap execution result, got {other:?}"),
            };
            let dedup_read = store.execute_dedup_backed_read(request.clone()).unwrap();
            let control = store.read_aspect_layout_control_truth(request).unwrap();
            assert_eq!(
                aspect_read.semantic_truth_digest(),
                control.authoritative_truth_digest()
            );
            assert_eq!(
                aspect_read.authoritative_commit_count(),
                control.authoritative_commit_count()
            );
            assert_eq!(
                dedup_read.read().semantic_truth_digest(),
                control.authoritative_truth_digest()
            );
            LaneResult::new(
                lane.label(),
                serde_json::to_string(&overlap_branch_parity_surface(
                    &aspect_read,
                    &dedup_read,
                    &control,
                ))
                .unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub(super) fn commit_coupled_seed_corruption() -> Vec<LaneResult<String>> {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("forge-store-m6-suite-seed-corruption");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_support_summary_seed_gap(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    vec![LaneResult::new(
        "commit_coupled_seed_gap",
        serde_json::to_string(&serde_json::json!({
            "error_kind": format!("{:?}", error.kind()),
            "message": error.message(),
        }))
        .unwrap(),
    )]
}

pub(super) fn chunk_export_corruption() -> Vec<LaneResult<String>> {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("forge-store-m6-suite-chunk-corruption");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_layout_materialization_chunk_member_count_drift(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    vec![LaneResult::new(
        "chunk_member_drift",
        serde_json::to_string(&serde_json::json!({
            "error_kind": format!("{:?}", error.kind()),
            "message": error.message(),
        }))
        .unwrap(),
    )]
}

pub(super) fn chunk_export_boundary_mismatch() -> Vec<LaneResult<String>> {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("forge-store-m6-suite-chunk-boundary");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_chunk_membership_boundary_drift(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    vec![LaneResult::new(
        "chunk_boundary_drift",
        serde_json::to_string(&serde_json::json!({
            "error_kind": format!("{:?}", error.kind()),
            "message": error.message(),
        }))
        .unwrap(),
    )]
}
