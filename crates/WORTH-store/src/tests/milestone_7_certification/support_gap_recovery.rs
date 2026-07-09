use super::*;

#[test]
fn milestone_7_support_gap_bundle_captures_typed_rebuild_classification() {
    let path = unique_test_store_path("worth-store-m7-gap-bundle");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    drop(store);
    force_first_lineage_support_gap(&path);

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
