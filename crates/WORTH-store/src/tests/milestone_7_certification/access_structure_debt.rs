use super::*;

#[test]
fn milestone_7_access_structure_verification_degrades_to_debt_when_cursor_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-cursor-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();
    drop(store);

    force_cursor_identity_key_mismatch(&path, "cursor-main");

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_eq!(
        bundle.backend_family,
        crate::DurableBackendFamily::LocalFileAtomicRewrite
    );
    assert_complexity_debt(
        &bundle.complexity_status.cursor_resume,
        &bundle.access_structure_verification.cursor_resume,
    );
}

#[test]
fn milestone_7_schema_access_structure_degrades_to_debt_when_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-schema-index-debt");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(first).unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_schema_support_key_mismatch(&path);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.schema_boundary_fetch,
        &bundle.access_structure_verification.schema_boundary_fetch,
    );
}

#[test]
fn milestone_7_lineage_access_structure_degrades_to_debt_when_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-lineage-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_lineage_support_key_mismatch(&path);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.lineage_lookup,
        &bundle.access_structure_verification.lineage_lookup,
    );
}

#[test]
fn milestone_7_support_publication_degrades_to_debt_when_summary_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-summary-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_commit_support_summary_key_mismatch(&path);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.commit_coupled_support_publication,
        &bundle
            .access_structure_verification
            .commit_coupled_support_publication,
    );
}

#[test]
fn milestone_7_cursor_identity_admission_degrades_to_debt_when_checkpoint_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-checkpoint-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();
    drop(store);

    force_subscriber_checkpoint_key_mismatch(&path, "cursor-main", 1);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.cursor_identity_admission,
        &bundle
            .access_structure_verification
            .cursor_identity_admission,
    );
}

#[test]
fn milestone_7_embedded_checkpoint_access_structure_degrades_to_debt_when_index_is_corrupted() {
    let path = unique_test_store_path("worth-store-m7-embedded-index-debt");
    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store
        .persist_embedded_checkpoint_record(EmbeddedCheckpointRecord {
            checkpoint_id: "checkpoint-main".to_string(),
            source_runtime_id: "runtime-main".to_string(),
            basis_branch_id: None,
            basis_commit_id: None,
            classification: StoredCheckpointClassification::DerivedDurable,
            contained_commit_ids: Vec::new(),
            metadata: serde_json::json!({"kind":"adversarial"}),
        })
        .unwrap();
    drop(store);

    force_embedded_checkpoint_key_mismatch(&path, "checkpoint-main");

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let bundle =
        reopened.milestone_7_certification_bundle(&reopened.export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.embedded_checkpoint_fetch,
        &bundle
            .access_structure_verification
            .embedded_checkpoint_fetch,
    );
}
