use crate::{ForgeStore, ForgeStoreBuilder};

use super::support::{
    create_entity, latest_envelope, runtime_with_demo_schema, unique_test_store_path,
    update_entity_on_branch,
};

#[test]
fn durable_artifact_authority_equivalence_bundle_matches_across_backend_and_rebuild_lanes() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut in_memory_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    in_memory_store
        .append_canonical_commit(first.clone())
        .unwrap();
    in_memory_store
        .append_canonical_commit(second.clone())
        .unwrap();

    let file_path = unique_test_store_path("forge-store-certification");
    let file_bundle = {
        let mut file_store = ForgeStoreBuilder::new()
            .local_file(file_path.clone())
            .build()
            .unwrap();
        file_store.append_canonical_commit(first.clone()).unwrap();
        file_store.append_canonical_commit(second.clone()).unwrap();
        file_store.milestone_1_certification_bundle()
    };

    let sqlite_path = unique_test_store_path("forge-store-certification-sqlite");
    let sqlite_bundle = {
        let mut sqlite_store = ForgeStoreBuilder::new()
            .sqlite_file(sqlite_path)
            .build()
            .unwrap();
        sqlite_store.append_canonical_commit(first.clone()).unwrap();
        sqlite_store
            .append_canonical_commit(second.clone())
            .unwrap();
        sqlite_store.milestone_1_certification_bundle()
    };

    let rebuild_bundle = {
        let rebuilt = ForgeStore::rebuild_from_authoritative_export(
            in_memory_store.export_authoritative_records(),
        )
        .unwrap();
        rebuilt.milestone_1_certification_bundle()
    };

    let in_memory_bundle = in_memory_store.milestone_1_certification_bundle();

    assert_eq!(
        in_memory_bundle.semantic.truth_digest,
        file_bundle.semantic.truth_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.history_digest,
        file_bundle.semantic.history_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.branch_heads_digest,
        file_bundle.semantic.branch_heads_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.artifact_digest,
        file_bundle.semantic.artifact_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.replay_digest,
        file_bundle.semantic.replay_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.truth_digest,
        sqlite_bundle.semantic.truth_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.history_digest,
        sqlite_bundle.semantic.history_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.branch_heads_digest,
        sqlite_bundle.semantic.branch_heads_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.artifact_digest,
        sqlite_bundle.semantic.artifact_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.replay_digest,
        sqlite_bundle.semantic.replay_digest
    );

    assert_eq!(
        in_memory_bundle.semantic.truth_digest,
        rebuild_bundle.semantic.truth_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.history_digest,
        rebuild_bundle.semantic.history_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.branch_heads_digest,
        rebuild_bundle.semantic.branch_heads_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.artifact_digest,
        rebuild_bundle.semantic.artifact_digest
    );
    assert_eq!(
        in_memory_bundle.semantic.replay_digest,
        rebuild_bundle.semantic.replay_digest
    );
}

#[test]
fn authoritative_export_and_certification_json_are_identical_across_equivalent_lanes() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut in_memory_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    in_memory_store
        .append_canonical_commit(first.clone())
        .unwrap();
    in_memory_store
        .append_canonical_commit(second.clone())
        .unwrap();

    let file_path = unique_test_store_path("forge-store-certification-json");
    let (file_export_json, file_bundle_json) = {
        let mut file_store = ForgeStoreBuilder::new()
            .local_file(file_path)
            .build()
            .unwrap();
        file_store.append_canonical_commit(first.clone()).unwrap();
        file_store.append_canonical_commit(second.clone()).unwrap();
        (
            file_store.export_authoritative_records().canonical_json(),
            file_store
                .milestone_1_certification_bundle()
                .semantic_json(),
        )
    };

    let sqlite_path = unique_test_store_path("forge-store-certification-json-sqlite");
    let (sqlite_export_json, sqlite_bundle_json) = {
        let mut sqlite_store = ForgeStoreBuilder::new()
            .sqlite_file(sqlite_path)
            .build()
            .unwrap();
        sqlite_store.append_canonical_commit(first.clone()).unwrap();
        sqlite_store
            .append_canonical_commit(second.clone())
            .unwrap();
        (
            sqlite_store.export_authoritative_records().canonical_json(),
            sqlite_store
                .milestone_1_certification_bundle()
                .semantic_json(),
        )
    };

    let rebuilt = ForgeStore::rebuild_from_authoritative_export(
        in_memory_store.export_authoritative_records(),
    )
    .unwrap();

    assert_eq!(
        in_memory_store
            .export_authoritative_records()
            .canonical_json(),
        file_export_json
    );
    assert_eq!(
        in_memory_store
            .export_authoritative_records()
            .canonical_json(),
        rebuilt.export_authoritative_records().canonical_json()
    );
    assert_eq!(
        in_memory_store
            .export_authoritative_records()
            .canonical_json(),
        sqlite_export_json
    );
    assert_eq!(
        in_memory_store
            .milestone_1_certification_bundle()
            .semantic_json(),
        file_bundle_json
    );
    assert_eq!(
        in_memory_store
            .milestone_1_certification_bundle()
            .semantic_json(),
        rebuilt.milestone_1_certification_bundle().semantic_json()
    );
    assert_eq!(
        in_memory_store
            .milestone_1_certification_bundle()
            .semantic_json(),
        sqlite_bundle_json
    );
}

#[test]
fn certification_counters_remain_lane_local_while_semantic_evidence_stays_equal() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first).unwrap();
    store.append_canonical_commit(second).unwrap();
    let operational_bundle = store.milestone_1_certification_bundle();

    let rebuilt =
        ForgeStore::rebuild_from_authoritative_export(store.export_authoritative_records())
            .unwrap();
    let rebuilt_bundle = rebuilt.milestone_1_certification_bundle();

    assert_eq!(operational_bundle.semantic, rebuilt_bundle.semantic);
    assert_ne!(
        operational_bundle.counter_snapshot,
        rebuilt_bundle.counter_snapshot
    );
    assert_eq!(
        rebuilt_bundle
            .counter_snapshot
            .authoritative_commit_append_count,
        0
    );
    assert_eq!(
        rebuilt_bundle
            .counter_snapshot
            .authoritative_commit_fetch_count,
        0
    );
}
