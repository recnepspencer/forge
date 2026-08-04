use super::{
    unique_test_sqlite_path, StoreErrorKind, SubscriptionSupportAccessStructure,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    WORTHStoreBuilder,
};

use super::raw_exact;

#[test]
fn sqlite_subscription_support_access_structure_debt_is_typed() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-access-debt");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
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
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_access_structure_state SET verified = 0 WHERE state_id = 'first_ship'",
            [],
        )
        .unwrap();

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened.subscription_support_access_structure_report();
    assert!(report.has_debt());
    assert_eq!(report.debted(), report.required());

    let error = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .expect_err("access-structure debt must not fall back to a global scan");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .access_structure_debts(),
        1
    );
}

#[test]
fn sqlite_subscription_support_missing_lookup_index_marks_access_debt() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-missing-index");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
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
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("DROP INDEX idx_subscription_support_family_artifact", [])
        .unwrap();
    drop(connection);

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened.subscription_support_access_structure_report();
    assert!(report.has_debt());
    assert_eq!(
        report.debted(),
        &[SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact]
    );

    let error = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .expect_err("missing lookup index must be remembered as access debt");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .access_structure_debts(),
        1
    );
}
