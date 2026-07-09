use super::*;

#[test]
fn repeated_crash_restart_loops_converge_to_same_truth_as_rebuild() {
    let path = unique_test_sqlite_path("worth-store-m3-restart-loop");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("first durable commit should acknowledge");
    let crashed_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha-again", create_beta_commit),
            SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .expect("second durable mutation should crash after recording canonical result");
    drop(durable);

    let recovered_once = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert_eq!(
        recovered_once
            .store()
            .export_authoritative_records()
            .commit_envelopes
            .len(),
        2
    );
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(
            |decision| decision.durable_mutation_id == crashed_mutation_id
                && decision.decision == RecoveryDecisionClass::FinishPublicationFromCanonicalResult
        ));
    assert!(recovered_once.last_recovery().degraded.is_empty());
    drop(recovered_once);

    let recovered_twice = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());

    let recovered_export = recovered_twice.store().export_authoritative_records();
    let rebuilt =
        WORTHStore::restore_from_authoritative_export(recovered_export.clone().admit_restore())
            .expect("rebuild from authoritative export should succeed");
    let rebuilt_export = rebuilt.export_authoritative_records();

    assert_eq!(
        recovered_export.canonical_json(),
        rebuilt_export.canonical_json()
    );
    let counters = recovered_twice.store().counters();
    assert!(counters.wal_record_scan_count >= 1);
    assert_eq!(counters.durable_commit_duplicate_suppression_count, 0);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 0);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 0);
    assert_eq!(counters.recovery_quiescent_restart_count, 1);
    assert_eq!(
        recovered_twice
            .resolve_retry(acknowledged.durable_mutation_id())
            .expect("retry resolution should stay stable"),
        crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: acknowledged.persisted().envelope().commit.commit_id,
        }
    );
    let status = recovered_twice.recovery_status_report().unwrap();
    assert!(status.quiescent_restart());
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::Clean
    );
    assert_eq!(status.planned_mutation_count(), 0);
}
