use super::*;

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

