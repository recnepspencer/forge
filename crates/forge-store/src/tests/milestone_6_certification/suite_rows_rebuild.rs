use super::*;
use crate::tests::milestone_6_certification::suite_helpers::*;

pub(super) fn authority_rebuild_parity() -> Vec<LaneResult<String>> {
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
    let path = unique_test_store_path("forge-store-m6-suite-authority-rebuild");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let before = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    let after = reopened.milestone_6_certification_bundle(request).unwrap();
    vec![
        LaneResult::new(
            "before_rebuild",
            serde_json::to_string(&rebuild_identity_surface(&before)).unwrap(),
        ),
        LaneResult::new(
            "after_rebuild",
            serde_json::to_string(&rebuild_identity_surface(&after)).unwrap(),
        ),
    ]
}

pub(super) fn chunk_export_rebuild_parity() -> Vec<LaneResult<String>> {
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
    let path = unique_test_store_path("forge-store-m6-suite-chunk-export");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let before = store
        .export_milestone_6_chunk_model(request.clone())
        .unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    let after = reopened.export_milestone_6_chunk_model(request).unwrap();
    vec![
        LaneResult::new(
            "before_rebuild",
            serde_json::to_string(&chunk_export_surface(&before)).unwrap(),
        ),
        LaneResult::new(
            "after_rebuild",
            serde_json::to_string(&chunk_export_surface(&after)).unwrap(),
        ),
    ]
}

pub(super) fn authority_rebuild_execution_parity() -> Vec<LaneResult<String>> {
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
    let path = unique_test_store_path("forge-store-m6-suite-rebuild-execution");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let before_read = match store.execute_aspect_layout_read(request.clone()).unwrap() {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result before rebuild, got {other:?}"),
    };
    let before_dedup = store.execute_dedup_backed_read(request.clone()).unwrap();
    drop(store);

    force_clear_milestone_6_materializations_and_derived_access_structures(&path);

    let mut reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    reopened
        .rebuild_milestone_6_derived_artifacts_from_authority()
        .unwrap();
    let after_read = match reopened
        .execute_aspect_layout_read(request.clone())
        .unwrap()
    {
        crate::AspectLayoutReadExecutionDecision::Admitted(read) => read,
        other => panic!("expected admitted execution result after rebuild, got {other:?}"),
    };
    let after_dedup = reopened.execute_dedup_backed_read(request).unwrap();
    vec![
        LaneResult::new(
            "before_rebuild",
            serde_json::to_string(&execution_surface(&before_read, &before_dedup)).unwrap(),
        ),
        LaneResult::new(
            "after_rebuild",
            serde_json::to_string(&execution_surface(&after_read, &after_dedup)).unwrap(),
        ),
    ]
}

pub(super) fn sqlite_legacy_seed_migration_parity() -> Vec<LaneResult<String>> {
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
    let path = unique_test_sqlite_path("forge-store-m6-suite-legacy-seed");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let before = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    drop(store);

    simulate_legacy_milestone_6_commit_coupled_layout_seed_storage(&path);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let after = reopened.milestone_6_certification_bundle(request).unwrap();
    vec![
        LaneResult::new(
            "before_migration",
            serde_json::to_string(&rebuild_identity_surface(&before)).unwrap(),
        ),
        LaneResult::new(
            "after_migration",
            serde_json::to_string(&rebuild_identity_surface(&after)).unwrap(),
        ),
    ]
}
