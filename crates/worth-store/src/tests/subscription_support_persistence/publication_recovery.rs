use super::{
    unique_test_sqlite_path, unique_test_store_path, RawSupportProgramAction,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRetentionDecision,
    SupportActionBreadthBudget, SupportActionId, SupportActionRecoveryDisposition,
    SupportAllocationScope, SupportPathClass, SupportProgramDensityClass, WORTHStoreBuilder,
};

use super::{raw_exact, retention_basis};

#[test]
fn publish_subscription_support_persists_complete_record_family() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();

    assert_eq!(
        fetched.record_set().key().artifact_id(),
        published.artifact_id().as_str()
    );
    assert_eq!(
        fetched.record_set().artifact_digest(),
        published.artifact_digest()
    );
    assert_eq!(fetched.record_set().basis_digest(), "basis:1");
    assert_eq!(fetched.record_set().cursor_digest(), "cursor:1");
    assert_eq!(fetched.record_set().checkpoint_digest(), "checkpoint:1");
    assert_eq!(fetched.record_set().schema_digest(), "schema:1");
    assert_eq!(
        fetched.record_set().compatibility_digest(),
        "compatibility:1"
    );
}

#[test]
fn subscription_support_action_publication_recovery_marks_pending_action_interrupted_after_reopen()
{
    let path = unique_test_store_path("worth-store-subscription-support-action-pending-recovery");
    let action_id = SupportActionId::new("support-retention:pending-publication-recovery").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("pending-recovery"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let mut reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let report = reopened
        .recover_subscription_support_action_publication(action_id.clone())
        .unwrap();
    assert_eq!(
        report.recovery_disposition(),
        SupportActionRecoveryDisposition::InterruptedBeforePublication
    );
    assert!(report.completed_action().is_none());
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        1
    );

    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get("subscription_support_action_records")
        .and_then(serde_json::Value::as_object)
        .expect("support action records should persist");
    let record = records
        .get(action_id.as_str())
        .expect("pending action record should persist");
    assert_eq!(
        record
            .get("publication_state")
            .and_then(serde_json::Value::as_str),
        Some("InterruptedBeforePublication")
    );

    let mut reopened_again = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let second = reopened_again
        .recover_subscription_support_action_publication(action_id)
        .unwrap();
    assert_eq!(
        second.recovery_disposition(),
        SupportActionRecoveryDisposition::InterruptedBeforePublication
    );
    assert_eq!(
        reopened_again
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        1
    );
}

#[test]
fn subscription_support_action_publication_recovery_reopens_published_consequence_without_duplication(
) {
    let path =
        unique_test_sqlite_path("worth-store-subscription-support-action-published-recovery");
    let action_id =
        SupportActionId::new("support-retention:published-publication-recovery").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                action_id.clone(),
                vec![retention_basis("published-recovery")],
                SubscriptionSupportRetentionDecision::retain_exact(),
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::FamilyLocalBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
    }

    let mut reopened = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let report = reopened
        .recover_subscription_support_action_publication(action_id)
        .unwrap();
    assert_eq!(
        report.recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    let completed = report
        .completed_action()
        .expect("published action recovery should expose completed action");
    assert_eq!(
        completed.envelope().recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        0
    );

    let mut reopened_again = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let second = reopened_again
        .recover_subscription_support_action_publication(
            SupportActionId::new("support-retention:published-publication-recovery").unwrap(),
        )
        .unwrap();
    assert_eq!(
        second.recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    assert_eq!(
        reopened_again
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        0
    );
}
