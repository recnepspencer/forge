use super::*;

#[test]
fn embedded_checkpoint_shape_requires_complete_basis() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();

    let error = store
        .persist_embedded_checkpoint_record(EmbeddedCheckpointRecord {
            checkpoint_id: "checkpoint-bad-shape".to_string(),
            source_runtime_id: "runtime-a".to_string(),
            basis_branch_id: Some(worth_relational::facade::history::BranchId(
                "main".to_string(),
            )),
            basis_commit_id: None,
            classification: StoredCheckpointClassification::DerivedDurable,
            contained_commit_ids: Vec::new(),
            metadata: Value::Null,
        })
        .expect_err("basis branch without basis commit must be rejected");

    assert_eq!(error.kind(), &StoreErrorKind::CheckpointShapeViolation);
}

#[test]
fn embedded_checkpoint_fetch_records_basis_reads() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut embedded = WORTHStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .unwrap();
    embedded
        .persist_external_commit(crate::ExternalRuntimeCommitEnvelope::new(
            "runtime-a",
            envelope.clone(),
        ))
        .unwrap();
    embedded
        .persist_external_checkpoint(
            embedded
                .admit_external_checkpoint(BasisBoundCheckpoint::<
                    DerivedDurableCheckpointKind,
                    NoContainedCommits,
                >::new(
                    "checkpoint-1",
                    "runtime-a",
                    envelope.branch_context.clone(),
                    envelope.commit.commit_id,
                ))
                .unwrap(),
        )
        .unwrap();

    embedded.fetch_persisted_checkpoint("checkpoint-1").unwrap();

    let counters = embedded.store().counters();
    assert_eq!(counters.embedded_checkpoint_fetch_count, 1);
    assert_eq!(counters.embedded_checkpoint_index_lookup_count, 1);
    assert_eq!(counters.embedded_checkpoint_basis_read_count, 1);
}

#[test]
fn durable_recovery_reports_cursor_checkpoint_gap_as_support_rebuild() {
    let path = unique_test_store_path("worth-store-support-gap-cursor");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    durable.append_canonical_commit(envelope.clone()).unwrap();
    durable
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

    force_cursor_checkpoint_gap(&path, "cursor-main", 1);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let support = recovered.last_support_artifact_recovery();
    assert_eq!(support.entries().len(), 1);
    assert_eq!(
        support.entries()[0].disposition(),
        crate::SupportArtifactRecoveryDisposition::RequireRebuild
    );
    assert_eq!(
        support.entries()[0].kind(),
        &StoreErrorKind::CursorCheckpointMissing
    );
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        crate::RecoveryOperatorDisposition::RebuildRequired
    );
    assert_eq!(status.support_artifacts().entries().len(), 1);
    assert_eq!(
        recovered
            .store()
            .counters()
            .support_artifact_recovery_gap_count,
        1
    );
}
