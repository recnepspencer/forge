use crate::{
    modes::SimulatedCrashPoint, DurableMutationRequest, ForgeStore, ForgeStoreBuilder,
    RecoveryDecisionClass, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
};

use super::harness::{
    corruption::local_file::{force_branch_head_gap, force_publication_commit_id_conflict},
    fixtures::{
        runtime::{create_entity_commit, runtime_with_demo_schema},
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

fn create_beta_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "beta"))
}

#[test]
fn crash_before_ack_discards_unpublished_intent() {
    let path = unique_test_store_path("forge-store-m3-before-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
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
    let recovery_handle = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m3-after-publish");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
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
    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-m3-after-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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

#[test]
fn repeated_crash_restart_loops_converge_to_same_truth_as_rebuild() {
    let path = unique_test_sqlite_path("forge-store-m3-restart-loop");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
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

    let recovered_once = ForgeStoreBuilder::new()
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

    let recovered_twice = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());

    let recovered_export = recovered_twice.store().export_authoritative_records();
    let rebuilt =
        ForgeStore::restore_from_authoritative_export(recovered_export.clone().admit_restore())
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

#[test]
fn conflicting_publication_commit_ids_are_rejected_during_recovery() {
    let path = unique_test_store_path("forge-store-m3-recovery-source-conflict");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    drop(durable);

    force_publication_commit_id_conflict(
        &path,
        forge_relational::facade::history::CommitId(
            acknowledged.persisted().envelope().commit.commit_id.0 + 999,
        ),
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .unwrap_err();

    assert_eq!(error.kind(), &crate::StoreErrorKind::RecoverySourceConflict);
}

#[test]
fn publication_gap_is_classified_as_quarantine_before_recovery_bluffs_truth() {
    let path = unique_test_store_path("forge-store-m3-publication-gap");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");
    let _acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    drop(durable);

    force_branch_head_gap(&path);

    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should emit typed degraded state instead of generic error");

    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::RequiresQuarantine
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 1);
    assert_eq!(
        recovered.last_recovery().degraded[0].kind,
        crate::DurableRecoveryDegradedKind::QuarantineRequired
    );
    let degraded_report = recovered.last_recovery().degraded_state_report();
    assert_eq!(degraded_report.quarantines().len(), 1);
    assert!(degraded_report.rebuilds().is_empty());
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::QuarantineRequired
    );
    assert_eq!(status.source_summary().requires_quarantine(), 1);
    assert_eq!(status.maintenance().entries().len(), 3);
    assert_eq!(status.recommended_actions().len(), 1);
    assert_eq!(
        status.recommended_actions()[0].kind(),
        RecoveryOperatorActionKind::QuarantineScope
    );
}
