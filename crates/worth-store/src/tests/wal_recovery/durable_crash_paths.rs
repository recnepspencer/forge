use super::*;

#[test]
fn crash_before_ack_discards_unpublished_intent() {
    let path = unique_test_store_path("worth-store-m3-before-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterIntentRecorded,
        )
        .expect("crash simulation should record durable intent");
    drop(durable);

    let reopened_runtime = runtime_with_demo_schema();
    let recovery_handle = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(reopened_runtime)
        .build_pending()
        .expect("recovery handle should build");
    let plan = recovery_handle.plan();
    assert_eq!(plan.pending_durable_mutation_ids, vec![durable_mutation_id]);
    let recovered = recovery_handle.recover().expect("recovery should complete");

    assert!(recovered
        .store()
        .export_authoritative_records()
        .commit_envelopes
        .is_empty());
    assert_eq!(recovered.last_recovery().decisions.len(), 1);
    assert_eq!(
        recovered.last_recovery().decisions[0].durable_mutation_id,
        durable_mutation_id
    );
    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::DiscardUnpublished
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 0);
    assert_eq!(recovered.last_recovery().source_reports.len(), 1);

    let counters = recovered.store().counters();
    assert_eq!(counters.durable_commit_unacknowledged_discard_count, 1);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 1);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
}

#[test]
fn crash_after_authoritative_publication_retains_truth_and_resolves_retry() {
    let path = unique_test_store_path("worth-store-m3-after-publish");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterAuthoritativeAppendPublished,
        )
        .expect("crash simulation should publish authoritative truth before crash");
    drop(durable);

    let reopened_runtime = runtime_with_demo_schema();
    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(reopened_runtime)
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should complete");

    let export = recovered.store().export_authoritative_records();
    assert_eq!(export.commit_envelopes.len(), 1);
    assert_eq!(recovered.last_recovery().decisions.len(), 1);
    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::RetainPublishedTruth
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 1);
    assert!(matches!(
        recovered.resolve_retry(durable_mutation_id),
        Ok(crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit { .. })
    ));
    let counters = recovered.store().counters();
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
    let degraded_report = recovered.last_recovery().degraded_state_report();
    assert_eq!(degraded_report.retained_without_acknowledgment().len(), 1);
    assert!(degraded_report.quarantines().is_empty());
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::RetainedWithoutAcknowledgment
    );
    assert_eq!(status.source_summary().published_authoritative_truth(), 1);
    assert_eq!(status.source_summary().requires_quarantine(), 0);
    assert_eq!(status.recommended_actions().len(), 1);
    assert_eq!(
        status.recommended_actions()[0].kind(),
        RecoveryOperatorActionKind::InspectRetainedWithoutAcknowledgment
    );
}

#[test]
fn crash_after_acknowledgment_retains_truth_exactly_once() {
    let path = unique_test_store_path("worth-store-m3-after-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge before crash");
    drop(durable);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should complete");

    let export = recovered.store().export_authoritative_records();
    assert_eq!(export.commit_envelopes.len(), 1);
    assert!(recovered.last_recovery().decisions.iter().any(|decision| {
        decision.durable_mutation_id == acknowledged.durable_mutation_id()
            && decision.decision == RecoveryDecisionClass::SuppressDuplicateReplay
    }));
    assert!(recovered.last_recovery().degraded.is_empty());

    let counters = recovered.store().counters();
    assert_eq!(counters.durable_commit_duplicate_suppression_count, 1);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.durable_commit_unacknowledged_discard_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
}
