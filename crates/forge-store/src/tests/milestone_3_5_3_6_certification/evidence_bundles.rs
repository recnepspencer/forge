use super::*;

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
