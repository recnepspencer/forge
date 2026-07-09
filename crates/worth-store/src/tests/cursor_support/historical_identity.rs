use super::*;

#[test]
fn historical_identity_resolution_returns_commit_scoped_lineage_neighborhood() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let lineage_id = envelope.lineage_events()[0].targets()[0];

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let resolution = store
        .fetch_lineage_history(HistoricalIdentityRequest::new(
            envelope.commit.commit_id,
            envelope.branch_context.clone(),
            lineage_id,
        ))
        .unwrap();

    assert_eq!(resolution.commit_id(), envelope.commit.commit_id);
    assert_eq!(resolution.branch_id(), &envelope.branch_context);
    assert_eq!(resolution.lineage_id(), lineage_id);
    assert_eq!(resolution.matching_events().len(), 1);
    assert_eq!(
        resolution.matching_event_ids(),
        vec![envelope.lineage_events()[0].event_id()]
    );
    assert!(resolution.resolved_lineage_ids().contains(&lineage_id));

    let counters = store.counters();
    assert_eq!(counters.lineage_lookup_count, 1);
    assert_eq!(counters.lineage_identity_lookup_count, 1);
    assert_eq!(
        counters.lineage_event_rows_read,
        envelope.lineage_events().len() as u64
    );
}

#[test]
fn historical_identity_resolution_gap_is_typed() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let error = store
        .fetch_lineage_history(HistoricalIdentityRequest::new(
            envelope.commit.commit_id,
            envelope.branch_context.clone(),
            LineageId(u64::MAX),
        ))
        .expect_err("unknown lineage id must surface as a typed historical resolution gap");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::HistoricalIdentityResolutionGap
    );
    let counters = store.counters();
    assert_eq!(counters.lineage_lookup_count, 1);
    assert_eq!(counters.lineage_identity_lookup_count, 1);
}

#[test]
fn durable_recovery_reports_checkpoint_shape_violation_as_support_quarantine() {
    let path = unique_test_store_path("worth-store-support-gap-checkpoint");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut embedded = WORTHStoreBuilder::new()
        .local_file(path.clone())
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

    force_embedded_checkpoint_shape_violation(&path, "checkpoint-1");

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
        crate::SupportArtifactRecoveryDisposition::RequireQuarantine
    );
    assert_eq!(
        support.entries()[0].kind(),
        &StoreErrorKind::CheckpointShapeViolation
    );
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        crate::RecoveryOperatorDisposition::QuarantineRequired
    );
}
