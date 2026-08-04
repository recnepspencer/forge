use super::{
    unique_test_sqlite_path, SubscriptionResumeClassification, SubscriptionSupportDriftCause,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest, WORTHStoreBuilder,
};

use super::raw_exact;

#[test]
fn subscription_support_resume_rejects_cross_family_evidence() {
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
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_expected_family_kind(SubscriptionSupportFamilyKind::DegradedContinuationSupport);

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportFamilyMismatch)
    );
}

#[test]
fn sqlite_subscription_support_legacy_rows_backfill_index_projections() {
    let legacy_path = unique_test_sqlite_path("worth-store-subscription-support-legacy-indexes");
    let record_set = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = source
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
        let published = source.publish_subscription_support(publishable).unwrap();
        source
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                published.artifact_id().clone(),
            ))
            .unwrap()
            .record_set()
            .clone()
    };

    let connection = rusqlite::Connection::open(&legacy_path).unwrap();
    connection
        .execute_batch(
            "
            CREATE TABLE subscription_support_record_sets (
                storage_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            ",
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO subscription_support_record_sets \
             (storage_key, family_id, artifact_id, payload_json) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                record_set.key().storage_key(),
                record_set.key().family_id(),
                record_set.key().artifact_id(),
                serde_json::to_string(&record_set).unwrap(),
            ],
        )
        .unwrap();
    drop(connection);

    let reopened = WORTHStoreBuilder::new()
        .sqlite_file(legacy_path.clone())
        .build()
        .unwrap();
    assert!(reopened
        .subscription_support_access_structure_report()
        .has_debt());

    let connection = rusqlite::Connection::open(&legacy_path).unwrap();
    let basis_digest: String = connection
        .query_row(
            "SELECT basis_digest FROM subscription_support_record_sets",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(basis_digest, "basis:1");
}
