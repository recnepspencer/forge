use serde_json::Value;

use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    BasisBoundCheckpoint, DerivedDurableCheckpointKind, DurableCursorAcknowledgeRequest,
    DurableCursorResumeRequest, ForgeStoreBuilder, HistoricalIdentityRequest, NoContainedCommits,
    StoreErrorKind,
};
use forge_relational::facade::identity::LineageId;

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::unique_test_sqlite_path,
};
use super::harness::{
    corruption::local_file::{
        force_cursor_checkpoint_gap, force_embedded_checkpoint_shape_violation,
    },
    fixtures::stores::unique_test_store_path,
};

#[test]
fn durable_cursor_resume_survives_sqlite_reopen() {
    let path = unique_test_sqlite_path("forge-store-cursor-resume");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let persisted = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();
    assert_eq!(persisted.record().checkpoint_sequence, 1);
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let plan = reopened
        .plan_cursor_resume(DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();

    assert_eq!(plan.identity().cursor_id, "cursor-main");
    assert_eq!(plan.latest_checkpoint().basis_commit_id, commit_id);
    assert_eq!(plan.latest_checkpoint().checkpoint_sequence, 1);

    let counters = reopened.counters();
    assert_eq!(counters.cursor_resume_count, 1);
    assert_eq!(counters.cursor_identity_lookup_count, 1);
    assert_eq!(counters.cursor_resume_support_rows_read, 2);
}

#[test]
fn cursor_resume_and_acknowledgment_support_explicit_witness_vocabulary() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();

    let admitted = store
        .admit_cursor_resume(DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();
    assert_eq!(admitted.identity().cursor_id, "cursor-main");

    let witness = store
        .admit_resumed_cursor_advance(
            &admitted,
            DurableCursorAcknowledgeRequest::new(
                "cursor-main",
                "subscriber-a",
                envelope.branch_context.clone(),
                "demo-feed",
                "schema:v1",
                1,
                envelope.commit.commit_id,
            ),
        )
        .unwrap();
    let persisted = store
        .acknowledge_resumed_cursor_progress(&admitted, witness)
        .unwrap();
    assert_eq!(persisted.record().checkpoint_sequence, 2);
}

#[test]
fn durable_cursor_regression_is_rejected() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    create_entity(&mut runtime, "beta");
    let second = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first.clone()).unwrap();
    store.append_canonical_commit(second.clone()).unwrap();

    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            second.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            second.commit.commit_id,
        ))
        .unwrap();

    let error = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            first.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            first.commit.commit_id,
        ))
        .expect_err("earlier frontier must be rejected as cursor regression");

    assert_eq!(error.kind(), &StoreErrorKind::CursorRegression);
    assert_eq!(store.counters().cursor_regression_reject_count, 1);
}

#[test]
fn durable_cursor_equivalence_basis_is_not_mutable() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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

    let error = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "different-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .expect_err("changing feed shape must mint a new cursor identity");

    assert_eq!(error.kind(), &StoreErrorKind::CursorEquivalenceViolation);
    assert_eq!(store.counters().cursor_equivalence_reject_count, 1);
}

#[test]
fn embedded_checkpoint_shape_requires_complete_basis() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();

    let error = store
        .persist_embedded_checkpoint_record(EmbeddedCheckpointRecord {
            checkpoint_id: "checkpoint-bad-shape".to_string(),
            source_runtime_id: "runtime-a".to_string(),
            basis_branch_id: Some(forge_relational::facade::history::BranchId(
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

    let mut embedded = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-support-gap-cursor");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut durable = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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

#[test]
fn historical_identity_resolution_returns_commit_scoped_lineage_neighborhood() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let lineage_id = envelope.lineage_events()[0].targets()[0];

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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
    let path = unique_test_store_path("forge-store-support-gap-checkpoint");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut embedded = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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
