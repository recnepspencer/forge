use super::*;

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
