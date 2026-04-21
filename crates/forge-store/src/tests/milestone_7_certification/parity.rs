use super::*;

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
