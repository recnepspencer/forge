use super::*;

fn milestone_3_6_suite() -> CertificationSuite<String, String> {
    let scenario = recovery_and_rebuild_equivalence();
    let recovered_export = scenario.recovered.store().export_authoritative_records();
    let quarantined = quarantined_recovery_handle();
    let retained_without_ack = retained_without_ack_recovery_handle();

    let mut snapshot_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    snapshot_store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    snapshot_store.append_canonical_commit(second).unwrap();
    let snapshot = snapshot_store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    snapshot_store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();
    let snapshot_report = snapshot_store
        .snapshot_maintenance_recovery_report(snapshot.snapshot_id)
        .unwrap();
    let maintenance_report = snapshot_store.maintenance_recovery_report().unwrap();

    let path = unique_test_store_path("forge-store-m36-quiescent-certification");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-beta", create_beta_commit),
            crate::modes::SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .unwrap();
    drop(durable);
    let _first = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();
    let second = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();
    let second_status = second.recovery_status_report().unwrap();

    let support_gap_local = {
        let path = unique_test_store_path("forge-store-m36-support-gap-local");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let envelope = latest_envelope(&runtime);
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
        drop(store);
        crate::tests::harness::corruption::local_file::force_first_lineage_support_gap(&path);
        let recovered = ForgeStoreBuilder::new()
            .local_file(path)
            .durable_mode(runtime_with_demo_schema())
            .build_pending()
            .unwrap()
            .recover()
            .unwrap();
        serde_json::to_string(recovered.last_support_artifact_recovery()).unwrap()
    };

    let support_gap_sqlite = {
        let path = unique_test_sqlite_path("forge-store-m36-support-gap-sqlite");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let envelope = latest_envelope(&runtime);
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
        drop(store);
        crate::tests::harness::corruption::sqlite::delete_first_sqlite_lineage_support_record(&path);
        let recovered = ForgeStoreBuilder::new()
            .sqlite_file(path)
            .durable_mode(runtime_with_demo_schema())
            .build_pending()
            .unwrap()
            .recover()
            .unwrap();
        serde_json::to_string(recovered.last_support_artifact_recovery()).unwrap()
    };

    CertificationSuite::new(ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "authoritative_truth_outranks_residue",
            vec![
                LaneResult::new("recovered", recovered_export.canonical_json()),
                LaneResult::new("rebuilt", scenario.rebuilt_export_json),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "interrupted_snapshot_publication",
            vec![LaneResult::new(
                "basis_only",
                format!(
                    "{:?}:{:?}:{:?}",
                    snapshot_report.publication_classification(),
                    snapshot_report.action(),
                    maintenance_report
                        .entries()
                        .iter()
                        .find(|entry| entry.family() == MaintenanceArtifactFamily::Snapshot)
                        .map(|entry| entry.disposition())
                        .unwrap()
                ),
            )],
            &[AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "retained_without_ack_lane",
            vec![LaneResult::new(
                "post_publish_crash",
                format!(
                    "{:?}:{}:{}",
                    retained_without_ack
                        .recovery_status_report()
                        .unwrap()
                        .operator_disposition(),
                    retained_without_ack
                        .last_recovery()
                        .degraded_state_report()
                        .retained_without_acknowledgment()
                        .len(),
                    retained_without_ack
                        .recovery_status_report()
                        .unwrap()
                        .recommended_actions()
                        .len()
                ),
            )],
            &[AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "quiescent_second_restart",
            vec![LaneResult::new(
                "second_restart",
                format!(
                    "{}:{}:{}",
                    second_status.recovered_decision_count(),
                    second_status.quiescent_restart(),
                    second.store().counters().recovery_quiescent_restart_count
                ),
            )],
            &[AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "support_gap_backend_parity",
            vec![
                LaneResult::new("local_file", support_gap_local),
                LaneResult::new("sqlite", support_gap_sqlite),
            ],
            &[AssertionClass::Equality],
        ))
        .with_rejection_row(RejectionRow::new(
            "quarantine_required_lane",
            vec![LaneResult::new(
                "branch_head_gap",
                format!(
                    "{:?}:{:?}:{:?}",
                    quarantined.last_recovery().decisions[0].decision,
                    quarantined.last_recovery().degraded[0].kind,
                    quarantined
                        .recovery_status_report()
                        .unwrap()
                        .operator_disposition()
                ),
            )],
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_3_6_certification_harness_scaffolds_recovery_suite() {
    let suite = milestone_3_6_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_eq!(
        suite.canonical_rows()[1].name(),
        "interrupted_snapshot_publication"
    );
    assert_eq!(
        suite.canonical_rows()[2].name(),
        "retained_without_ack_lane"
    );
    assert_eq!(suite.canonical_rows()[3].name(), "quiescent_second_restart");
    assert_eq!(
        suite.canonical_rows()[4].name(),
        "support_gap_backend_parity"
    );
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness =
        evaluate_completeness(&suite, &ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}
