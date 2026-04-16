use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    DurableCursorAcknowledgeRequest, ForgeStore, ForgeStoreBuilder, RecoveryOperatorDisposition,
    SupportArtifactRecoveryReport,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, SCHEMA_LINEAGE_CURSOR_DURABILITY_TEST},
    },
    corruption::{
        local_file::{
            force_commit_support_summary_key_mismatch, force_cursor_checkpoint_gap,
            force_cursor_identity_key_mismatch, force_embedded_checkpoint_key_mismatch,
            force_first_lineage_support_gap, force_lineage_support_key_mismatch,
            force_schema_support_key_mismatch, force_subscriber_checkpoint_key_mismatch,
        },
        sqlite::delete_first_sqlite_lineage_support_record,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

fn canonical_support_truth(bundle: &crate::Milestone7CertificationBundle) -> serde_json::Value {
    serde_json::json!({
        "backend_family": format!("{:?}", bundle.backend_family),
        "history_digest": bundle.history_digest,
        "artifact_digest": bundle.artifact_digest,
        "replay_digest": bundle.replay_digest,
        "support_truth_digest": bundle.support_truth_digest,
        "support_artifact_recovery_report": bundle.support_artifact_recovery_report,
        "certification_summary": bundle.certification_summary,
        "access_structure_contract": bundle.access_structure_contract,
        "access_structure_verification": bundle.access_structure_verification,
        "complexity_status": bundle.complexity_status,
    })
}

fn support_gap_surface(
    report: &SupportArtifactRecoveryReport,
    disposition: RecoveryOperatorDisposition,
    gap_count: u64,
) -> serde_json::Value {
    serde_json::json!({
        "support_artifact_recovery_report": report,
        "operator_disposition": format!("{disposition:?}"),
        "support_artifact_recovery_gap_count": gap_count,
    })
}

fn assert_complexity_debt(
    path: &crate::Milestone7ComplexityPathStatus,
    verification: &crate::Milestone7AccessStructureVerificationPath,
) {
    assert!(!verification.verified_at_open);
    assert!(
        verification
            .verification_gap
            .as_deref()
            .unwrap_or_default()
            .contains("stored key")
            || verification
                .verification_gap
                .as_deref()
                .unwrap_or_default()
                .contains("map key")
    );
    assert_eq!(path.status, crate::ComplexityStatus::Debt);
    assert!(path.proof_basis.is_none());
    assert!(
        path.debt_reason
            .as_deref()
            .unwrap_or_default()
            .contains("stored key")
            || path
                .debt_reason
                .as_deref()
                .unwrap_or_default()
                .contains("map key")
    );
}

fn milestone_7_suite() -> CertificationSuite<String, String> {
    let clean_bundle = {
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

        let control = ForgeStore::restore_from_authoritative_export(
            store.export_authoritative_records().admit_restore(),
        )
        .unwrap();

        (
            serde_json::to_string(&canonical_support_truth(
                &store.milestone_7_certification_bundle(&control.export_authoritative_records()),
            ))
            .unwrap(),
            serde_json::to_string(&canonical_support_truth(
                &control.milestone_7_certification_bundle(&control.export_authoritative_records()),
            ))
            .unwrap(),
        )
    };

    let support_gap_local = {
        let path = unique_test_store_path("forge-store-m7-support-gap-local");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let envelope = latest_envelope(&runtime);
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
        drop(store);
        force_first_lineage_support_gap(&path);
        let recovered = ForgeStoreBuilder::new()
            .local_file(path)
            .durable_mode(runtime_with_demo_schema())
            .build_pending()
            .unwrap()
            .recover()
            .unwrap();
        (
            serde_json::to_string(&support_gap_surface(
                recovered.last_support_artifact_recovery(),
                recovered
                    .recovery_status_report()
                    .unwrap()
                    .operator_disposition(),
                recovered
                    .store()
                    .counters()
                    .support_artifact_recovery_gap_count,
            ))
            .unwrap(),
            recovered
                .recovery_status_report()
                .unwrap()
                .operator_disposition(),
            recovered
                .store()
                .counters()
                .support_artifact_recovery_gap_count,
        )
    };

    let support_gap_sqlite = {
        let path = unique_test_sqlite_path("forge-store-m7-support-gap-sqlite");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let envelope = latest_envelope(&runtime);
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
        drop(store);
        delete_first_sqlite_lineage_support_record(&path);
        let recovered = ForgeStoreBuilder::new()
            .sqlite_file(path)
            .durable_mode(runtime_with_demo_schema())
            .build_pending()
            .unwrap()
            .recover()
            .unwrap();
        (
            serde_json::to_string(&support_gap_surface(
                recovered.last_support_artifact_recovery(),
                recovered
                    .recovery_status_report()
                    .unwrap()
                    .operator_disposition(),
                recovered
                    .store()
                    .counters()
                    .support_artifact_recovery_gap_count,
            ))
            .unwrap(),
            recovered
                .recovery_status_report()
                .unwrap()
                .operator_disposition(),
            recovered
                .store()
                .counters()
                .support_artifact_recovery_gap_count,
        )
    };

    let typed_cursor_gap = {
        let path = unique_test_store_path("forge-store-m7-typed-cursor-gap");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let envelope = latest_envelope(&runtime);
        let mut store = ForgeStoreBuilder::new()
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
        force_cursor_checkpoint_gap(&path, "cursor-main", 1);
        let recovered = ForgeStoreBuilder::new()
            .local_file(path)
            .durable_mode(runtime_with_demo_schema())
            .build_pending()
            .unwrap()
            .recover()
            .unwrap();
        format!(
            "{:?}:{:?}:{:?}:{:?}:{}",
            recovered.last_support_artifact_recovery().entries()[0].family(),
            recovered.last_support_artifact_recovery().entries()[0].kind(),
            recovered.last_support_artifact_recovery().entries()[0].disposition(),
            recovered
                .recovery_status_report()
                .unwrap()
                .operator_disposition(),
            recovered
                .store()
                .counters()
                .support_artifact_recovery_gap_count
        )
    };

    CertificationSuite::new(SCHEMA_LINEAGE_CURSOR_DURABILITY_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "support_artifact_restart_parity",
            vec![
                LaneResult::new("primary", clean_bundle.0),
                LaneResult::new("control", clean_bundle.1),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "support_gap_backend_parity",
            vec![
                LaneResult::new("local_file", support_gap_local.0),
                LaneResult::new("sqlite", support_gap_sqlite.0),
            ],
            &[AssertionClass::Equality],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_support_gap_classification",
            vec![LaneResult::new("cursor_checkpoint_gap", typed_cursor_gap)],
            &[AssertionClass::TypedFailure, AssertionClass::ExactCounter],
        ))
}

#[test]
fn milestone_7_certification_harness_scaffolds_support_durability_suite() {
    let suite = milestone_7_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness = evaluate_completeness(&suite, &SCHEMA_LINEAGE_CURSOR_DURABILITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_7_bundle_proves_clean_support_artifact_restart_parity() {
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

    let control = ForgeStore::restore_from_authoritative_export(
        store.export_authoritative_records().admit_restore(),
    )
    .unwrap();
    let bundle = store.milestone_7_certification_bundle(&control.export_authoritative_records());
    let control_bundle =
        control.milestone_7_certification_bundle(&control.export_authoritative_records());

    assert_eq!(bundle.backend_family, crate::DurableBackendFamily::InMemory);
    assert!(!bundle.history_digest.is_empty());
    assert!(!bundle.artifact_digest.is_empty());
    assert!(!bundle.replay_digest.is_empty());
    assert!(!bundle.support_truth_digest.is_empty());
    assert!(!bundle.diagnostics_digest.is_empty());
    assert!(bundle.support_artifact_recovery_report.is_clean());
    assert!(bundle.certification_summary.clean_restart_support);
    assert!(
        bundle
            .certification_summary
            .exactly_once_support_publication
    );
    assert_eq!(
        bundle.certification_summary.support_rebuild_required_count,
        0
    );
    assert_eq!(
        bundle
            .certification_summary
            .support_quarantine_required_count,
        0
    );
    assert_eq!(bundle.certification_summary.cursor_support_entry_count, 0);
    assert_eq!(bundle.counter_contract.commit_support_publication_count, 1);
    assert_eq!(
        bundle.counter_contract.commit_support_summary_build_count,
        1
    );
    assert_eq!(bundle.counter_contract.cursor_ack_count, 1);
    assert_eq!(bundle.counter_contract.subscriber_checkpoint_write_count, 1);
    assert_eq!(
        bundle.counter_contract.schema_boundary_index_lookup_count,
        0
    );
    assert_eq!(bundle.counter_contract.lineage_identity_lookup_count, 0);
    assert_eq!(
        bundle.counter_contract.support_artifact_recovery_gap_count,
        0
    );
    assert_eq!(
        bundle.access_structure_contract.backend_family,
        crate::DurableBackendFamily::InMemory
    );
    assert!(
        bundle
            .access_structure_verification
            .schema_boundary_fetch
            .verified_at_open
    );
    assert!(bundle
        .access_structure_contract
        .schema_boundary_fetch
        .access_structure
        .contains("in-memory BTreeMap indexes"));
    assert_eq!(
        bundle.complexity_status.schema_boundary_fetch.status,
        crate::ComplexityStatus::Verified
    );
    assert!(bundle
        .complexity_status
        .schema_boundary_fetch
        .proof_basis
        .as_deref()
        .unwrap_or_default()
        .contains("loaded schema support map preserves exact artifact-id addressing"));
    assert!(bundle
        .complexity_status
        .schema_boundary_fetch
        .debt_reason
        .is_none());
    assert_eq!(
        bundle.complexity_status.lineage_lookup.status,
        crate::ComplexityStatus::Verified
    );
    assert!(bundle
        .complexity_status
        .lineage_lookup
        .proof_basis
        .as_deref()
        .unwrap_or_default()
        .contains("loaded lineage support map preserves exact artifact-id addressing"));
    assert!(bundle
        .complexity_status
        .lineage_lookup
        .debt_reason
        .is_none());
    assert_eq!(
        bundle.complexity_status.cursor_resume.status,
        crate::ComplexityStatus::Verified
    );
    assert!(bundle
        .complexity_status
        .cursor_resume
        .proof_basis
        .as_deref()
        .unwrap_or_default()
        .contains("loaded durable cursor identity map preserves exact cursor-id addressing"));
    assert!(bundle.complexity_status.cursor_resume.debt_reason.is_none());
    assert_eq!(
        bundle.support_truth_digest,
        control_bundle.support_truth_digest
    );
    assert_ne!(bundle.diagnostics_digest, control_bundle.diagnostics_digest);
}

#[test]
fn milestone_7_backend_contracts_are_family_specific() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let local_path = unique_test_store_path("forge-store-m7-local-contract");
    let sqlite_path = unique_test_sqlite_path("forge-store-m7-sqlite-contract");

    let local_bundle = {
        let mut store = ForgeStoreBuilder::new()
            .local_file(local_path)
            .build()
            .unwrap();
        store.append_canonical_commit(envelope.clone()).unwrap();
        store.milestone_7_certification_bundle(&store.export_authoritative_records())
    };

    let sqlite_bundle = {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(sqlite_path)
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
        store.milestone_7_certification_bundle(&store.export_authoritative_records())
    };

    assert_eq!(
        local_bundle.backend_family,
        crate::DurableBackendFamily::LocalFileAtomicRewrite
    );
    assert_eq!(
        sqlite_bundle.backend_family,
        crate::DurableBackendFamily::SqliteTransactional
    );
    assert!(
        local_bundle
            .access_structure_verification
            .schema_boundary_fetch
            .verified_at_open
    );
    assert!(
        sqlite_bundle
            .access_structure_verification
            .schema_boundary_fetch
            .verified_at_open
    );
    assert!(local_bundle
        .access_structure_contract
        .schema_boundary_fetch
        .access_structure
        .contains("local-file authoritative image maps rebuilt atomically per write"));
    assert!(sqlite_bundle
        .access_structure_contract
        .schema_boundary_fetch
        .access_structure
        .contains("sqlite primary-key and transactional row indexes"));
    assert_ne!(
        local_bundle.access_structure_contract,
        sqlite_bundle.access_structure_contract
    );
    assert_ne!(
        local_bundle.access_structure_verification.backend_family,
        sqlite_bundle.access_structure_verification.backend_family
    );
}

#[test]
fn milestone_7_support_gap_bundle_captures_typed_rebuild_classification() {
    let path = unique_test_store_path("forge-store-m7-gap-bundle");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);
    force_first_lineage_support_gap(&path);

    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let bundle = recovered
        .store()
        .milestone_7_certification_bundle(&recovered.store().export_authoritative_records());

    assert_eq!(bundle.support_artifact_recovery_report.entries().len(), 1);
    assert_eq!(
        bundle.support_artifact_recovery_report.entries()[0].family(),
        crate::SupportArtifactFamily::LineageSupport
    );
    assert_eq!(
        bundle.support_artifact_recovery_report.entries()[0].disposition(),
        crate::SupportArtifactRecoveryDisposition::RequireRebuild
    );
    assert!(bundle.support_artifact_recovery_report.entries()[0]
        .scope_identity()
        .contains("commit-support-summary:lineage:"));
    assert!(!bundle.certification_summary.clean_restart_support);
    assert!(
        bundle
            .certification_summary
            .exactly_once_support_publication
    );
    assert_eq!(
        bundle.certification_summary.support_rebuild_required_count,
        1
    );
    assert_eq!(
        bundle
            .certification_summary
            .support_quarantine_required_count,
        0
    );
    assert_eq!(bundle.certification_summary.schema_support_entry_count, 0);
    assert_eq!(bundle.certification_summary.lineage_support_entry_count, 1);
    assert_eq!(bundle.certification_summary.related_commit_entry_count, 1);
    assert_eq!(
        recovered
            .recovery_status_report()
            .unwrap()
            .operator_disposition(),
        RecoveryOperatorDisposition::RebuildRequired
    );
    assert_eq!(bundle.counter_contract.commit_support_publication_count, 0);
    assert_eq!(
        bundle.counter_contract.commit_support_summary_build_count,
        0
    );
    assert_eq!(
        bundle.counter_contract.commit_support_publication_gap_count,
        0
    );
    assert_eq!(
        bundle.counter_contract.support_artifact_recovery_gap_count,
        1
    );
    assert_eq!(
        bundle.counter_snapshot.support_artifact_recovery_gap_count,
        1
    );
}

#[test]
fn milestone_7_bundle_proves_exactly_once_support_publication_under_duplicate_append() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store.append_canonical_commit(envelope).unwrap();

    let control = ForgeStore::restore_from_authoritative_export(
        store.export_authoritative_records().admit_restore(),
    )
    .unwrap();
    let bundle = store.milestone_7_certification_bundle(&control.export_authoritative_records());
    let control_bundle =
        control.milestone_7_certification_bundle(&control.export_authoritative_records());

    assert!(
        bundle
            .certification_summary
            .exactly_once_support_publication
    );
    assert_eq!(bundle.counter_contract.commit_support_publication_count, 1);
    assert_eq!(
        bundle.counter_contract.commit_support_summary_build_count,
        1
    );
    assert_eq!(
        bundle.support_truth_digest,
        control_bundle.support_truth_digest
    );
}

#[test]
fn milestone_7_access_structure_verification_degrades_to_debt_when_cursor_index_is_corrupted() {
    let path = unique_test_store_path("forge-store-m7-cursor-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m7-schema-index-debt");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(first).unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_schema_support_key_mismatch(&path);

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m7-lineage-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_lineage_support_key_mismatch(&path);

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m7-summary-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);

    force_commit_support_summary_key_mismatch(&path);

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m7-checkpoint-index-debt");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m7-embedded-index-debt");
    let mut store = ForgeStoreBuilder::new()
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

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let bundle =
        reopened.milestone_7_certification_bundle(&reopened.export_authoritative_records());

    assert_complexity_debt(
        &bundle.complexity_status.embedded_checkpoint_fetch,
        &bundle
            .access_structure_verification
            .embedded_checkpoint_fetch,
    );
}
