use crate::{
    DurableMutationRequest, ForgeStoreBuilder, MaintenanceArtifactFamily,
    MaintenanceRecoveryDisposition, ObservedPublicationFailure, ObservedRecoveryFailure356,
    PublicationClassification, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
    SnapshotCaptureRequest, StoreError,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{
            evaluate_completeness, ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST,
            DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST,
        },
    },
    corruption::local_file::force_branch_head_gap,
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
    scenarios::{
        publication::{create_alpha_commit, durable_publication_reports},
        recovery::{create_beta_commit, recovery_and_rebuild_equivalence},
    },
};

fn milestone_3_5_failures() -> Vec<ObservedPublicationFailure> {
    let record =
        crate::wal::WalRecord::durable_mutation_intent(1, crate::DurableMutationId(77), "rt", "x")
            .unwrap();
    let classified = record
        .classify_media_barrier(crate::DurabilityBarrierClass::FileContentDurable)
        .unwrap();
    let framed = classified.record().framed_record().as_bytes().to_vec();
    let truncated_error =
        crate::wal::WalRecord::decode_from_media_bytes(framed[..framed.len() - 5].to_vec())
            .unwrap_err();

    let mut torn_bytes = framed.clone();
    let payload_index = torn_bytes.iter().position(|byte| *byte == b'r').unwrap();
    torn_bytes[payload_index] = b'X';
    let torn_error = crate::wal::WalRecord::decode_from_media_bytes(torn_bytes).unwrap_err();
    let source_error = StoreError::external_runtime_artifact_rejection(
        "integrity-valid family failed local source admission",
    );

    vec![
        ObservedPublicationFailure::from_error(&truncated_error),
        ObservedPublicationFailure::from_error(&torn_error),
        ObservedPublicationFailure::from_error(&source_error),
    ]
}

fn quarantined_recovery_handle() -> crate::DurableStoreHandle {
    let path = unique_test_store_path("forge-store-m36-quarantine-certification");
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
    drop(durable);
    force_branch_head_gap(&path);
    ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap()
}

fn retained_without_ack_recovery_handle() -> crate::DurableStoreHandle {
    let path = unique_test_store_path("forge-store-m36-retained-without-ack");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            crate::modes::SimulatedCrashPoint::AfterAuthoritativeAppendPublished,
        )
        .unwrap();
    drop(durable);
    ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap()
}

fn milestone_3_6_failures() -> Vec<ObservedRecoveryFailure356> {
    let path = unique_test_store_path("forge-store-m36-source-conflict");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    drop(durable);

    super::harness::corruption::local_file::force_publication_commit_id_conflict(
        &path,
        forge_relational::facade::history::CommitId(
            acknowledged.persisted().envelope().commit.commit_id.0 + 999,
        ),
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap_err();

    vec![ObservedRecoveryFailure356::from_error(&error)]
}

fn milestone_3_5_suite() -> CertificationSuite<String, String> {
    let reports = durable_publication_reports();
    let failures = milestone_3_5_failures();
    let normalize = |report: &crate::PublicationWriteOutcome| {
        serde_json::to_string(
            &report
                .family_states()
                .iter()
                .map(|state| (state.family(), state.state(), state.source_admitted()))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };

    let path = unique_test_store_path("forge-store-m35-gap-certification");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    drop(durable);
    force_branch_head_gap(&path);
    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let gap_report = reopened
        .durable_publication_report(
            acknowledged.durable_mutation_id(),
            Some(acknowledged.persisted().envelope().commit.commit_id),
        )
        .unwrap();

    CertificationSuite::new(DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "publication_family_equivalence",
            vec![
                LaneResult::new("local_file", normalize(&reports.local_report)),
                LaneResult::new("sqlite", normalize(&reports.sqlite_report)),
            ],
            &[AssertionClass::Equality],
        ))
        .with_rejection_row(RejectionRow::new(
            "publication_gap_classification",
            vec![LaneResult::new(
                "branch_head_gap",
                format!("{:?}", gap_report.classification()),
            )],
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_media_failures",
            vec![LaneResult::new(
                "failure_kinds",
                serde_json::to_string(
                    &failures
                        .iter()
                        .map(|failure| failure.kind.clone())
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            )],
            &[AssertionClass::TypedFailure],
        ))
}

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
        super::harness::corruption::local_file::force_first_lineage_support_gap(&path);
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
        super::harness::corruption::sqlite::delete_first_sqlite_lineage_support_record(&path);
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
fn milestone_3_5_certification_harness_scaffolds_publication_suite() {
    let suite = milestone_3_5_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness = evaluate_completeness(&suite, &DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
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

#[test]
fn milestone_3_5_evidence_bundle_captures_write_path_proof_surface() {
    let path = unique_test_store_path("forge-store-m35-bundle");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    let report = durable
        .store()
        .durable_publication_report(
            acknowledged.durable_mutation_id(),
            Some(acknowledged.persisted().envelope().commit.commit_id),
        )
        .unwrap();
    let bundle = durable
        .store()
        .milestone_3_5_certification_bundle(report, &milestone_3_5_failures());

    assert_eq!(
        bundle.ack_boundary_report.classification(),
        PublicationClassification::RetainTrusted
    );
    assert_eq!(bundle.certification_summary.family_count, 6);
    assert_eq!(bundle.certification_summary.published_family_count, 6);
    assert_eq!(bundle.certification_summary.publication_gap_family_count, 0);
    assert_eq!(
        bundle
            .certification_summary
            .non_source_admitted_family_count,
        0
    );
    assert!(bundle.certification_summary.sufficient_for_published_truth);
    assert!(bundle.certification_summary.acknowledgment_eligible);
    assert!(!bundle.artifact_digest.is_empty());
    assert!(!bundle.write_path_digest.is_empty());
    assert_eq!(
        bundle.media_barrier_matrix.backend_family,
        crate::DurableBackendFamily::LocalFileAtomicRewrite
    );
    assert_eq!(
        bundle.tail_validation_report.durable_frame_scan_count,
        bundle.counter_snapshot.durable_frame_scan_count
    );
    assert_eq!(bundle.counter_snapshot.state_clone_fallback_count, 0);
    assert!(bundle.counter_snapshot.state_delta_apply_count >= 6);
    assert_eq!(bundle.observed_failures.len(), 3);
    assert!(bundle
        .observed_failures
        .iter()
        .any(|failure| { failure.kind == crate::StoreErrorKind::DurableTailTruncated }));
    assert!(bundle
        .observed_failures
        .iter()
        .any(|failure| { failure.kind == crate::StoreErrorKind::DurableTornWriteDetected }));
    assert!(bundle.observed_failures.iter().any(|failure| {
        failure.kind == crate::StoreErrorKind::ExternalRuntimeArtifactRejection
    }));
    assert!(!bundle.failure_digest.is_empty());
}

#[test]
fn milestone_3_6_evidence_bundle_captures_recovery_reports() {
    let scenario = recovery_and_rebuild_equivalence();
    let maintenance_count = scenario
        .recovered
        .store()
        .maintenance_recovery_report()
        .unwrap()
        .entries()
        .len();
    let bundle = scenario
        .recovered
        .milestone_3_6_certification_bundle(&milestone_3_6_failures())
        .unwrap();

    assert!(!bundle.truth_digest.is_empty());
    assert!(!bundle.artifact_digest.is_empty());
    assert!(!bundle.compatibility_digest.is_empty());
    assert!(bundle
        .backup_restore_compatibility_report
        .local_restart_only());
    assert!(bundle
        .backup_restore_compatibility_report
        .external_restore_requires_explicit_mode());
    assert!(bundle
        .backup_restore_compatibility_report
        .incompatibilities()
        .is_empty());
    assert_eq!(
        bundle.recovery_status_report.operator_disposition(),
        RecoveryOperatorDisposition::Clean
    );
    assert_eq!(
        bundle.certification_summary.source_report_count,
        bundle.recovery_source_report.len()
    );
    assert_eq!(
        bundle.certification_summary.fallback_source_count,
        bundle
            .recovery_source_report
            .iter()
            .filter(|report| {
                !matches!(
                    report.source_kind(),
                    crate::RecoverySourceKind::PublishedAuthoritativeTruth
                )
            })
            .count()
    );
    assert_eq!(
        bundle.certification_summary.quarantine_source_count,
        bundle
            .recovery_source_report
            .iter()
            .filter(|report| {
                matches!(
                    report.source_kind(),
                    crate::RecoverySourceKind::RequiresQuarantine
                )
            })
            .count()
    );
    assert_eq!(bundle.certification_summary.degraded_quarantine_count, 0);
    assert_eq!(
        bundle
            .certification_summary
            .degraded_retained_without_ack_count,
        0
    );
    assert_eq!(
        bundle
            .certification_summary
            .support_artifact_rebuild_required_count,
        0
    );
    assert_eq!(
        bundle
            .certification_summary
            .support_artifact_quarantine_required_count,
        0
    );
    assert_eq!(
        bundle.maintenance_recovery_report.entries().len(),
        maintenance_count
    );
    assert!(bundle.quiescence_report.recovered_decision_count >= 1);
    assert!(!bundle.quiescence_report.quiescent_restart);
    assert_eq!(
        bundle
            .quiescence_report
            .recovery_non_quiescent_restart_count,
        bundle.counter_snapshot.recovery_non_quiescent_restart_count
    );
    assert_eq!(
        bundle.quiescence_report.recovery_quiescent_restart_count,
        bundle.counter_snapshot.recovery_quiescent_restart_count
    );
    assert!(bundle.recovery_source_report.iter().all(|report| {
        !matches!(
            report.source_kind(),
            crate::RecoverySourceKind::RequiresQuarantine
        )
    }));
    assert_eq!(bundle.observed_failures.len(), 1);
    assert_eq!(
        bundle.observed_failures[0].kind,
        crate::StoreErrorKind::RecoverySourceConflict
    );
}

#[test]
fn milestone_3_6_bundle_captures_retained_without_ack_lane() {
    let recovered = retained_without_ack_recovery_handle();
    let bundle = recovered.milestone_3_6_certification_bundle(&[]).unwrap();

    assert_eq!(
        bundle.recovery_status_report.operator_disposition(),
        RecoveryOperatorDisposition::RetainedWithoutAcknowledgment
    );
    assert_eq!(bundle.certification_summary.source_report_count, 1);
    assert_eq!(bundle.certification_summary.fallback_source_count, 0);
    assert_eq!(
        bundle
            .certification_summary
            .degraded_retained_without_ack_count,
        1
    );
    assert_eq!(bundle.certification_summary.recommended_action_count, 1);
}

#[test]
fn milestone_3_6_bundle_captures_quarantine_lane() {
    let recovered = quarantined_recovery_handle();
    let bundle = recovered.milestone_3_6_certification_bundle(&[]).unwrap();

    assert_eq!(
        bundle.recovery_status_report.operator_disposition(),
        RecoveryOperatorDisposition::QuarantineRequired
    );
    assert_eq!(bundle.certification_summary.source_report_count, 1);
    assert_eq!(bundle.certification_summary.fallback_source_count, 1);
    assert_eq!(bundle.certification_summary.quarantine_source_count, 1);
    assert_eq!(bundle.certification_summary.degraded_quarantine_count, 1);
    assert_eq!(bundle.certification_summary.recommended_action_count, 1);
}

#[test]
fn interrupted_snapshot_publication_requires_rebuild_not_trusted_truth() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let report = store
        .snapshot_maintenance_recovery_report(snapshot.snapshot_id)
        .unwrap();
    assert_eq!(
        report.publication_classification(),
        PublicationClassification::RequireRebuild
    );
    assert_eq!(
        report.action(),
        crate::SnapshotMaintenanceRecoveryAction::RequireRebuild
    );
    let maintenance_report = store.maintenance_recovery_report().unwrap();
    let snapshot_entry = maintenance_report
        .entries()
        .iter()
        .find(|entry| entry.family() == MaintenanceArtifactFamily::Snapshot)
        .unwrap();
    assert_eq!(
        snapshot_entry.disposition(),
        MaintenanceRecoveryDisposition::RequireRebuild
    );
}

#[test]
fn recovery_status_report_elevates_snapshot_rebuild_requirement_to_operator_surface() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let outcome = crate::DurableRecoveryOutcome {
        decisions: Vec::new(),
        degraded: Vec::new(),
        source_reports: Vec::new(),
    };
    let plan = crate::recovery::DurableRecoveryPlan {
        pending_durable_mutation_ids: Vec::new(),
    };
    let report = crate::RecoveryStatusReport::new(
        &plan,
        &outcome,
        store.maintenance_recovery_report().unwrap(),
        store.support_artifact_recovery_report(),
    );

    assert_eq!(
        report.operator_disposition(),
        RecoveryOperatorDisposition::RebuildRequired
    );
    assert_eq!(report.maintenance().entries().len(), 4);
    assert!(report.recommended_actions().iter().any(|action| {
        action.kind() == RecoveryOperatorActionKind::RebuildMaintenanceArtifact
            && action.scope_identity().contains("snapshot:")
    }));
}
