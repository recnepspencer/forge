use super::{
    unique_test_store_path, StoreErrorKind, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest, WORTHStoreBuilder,
};

use super::raw_exact;

#[test]
fn fetch_subscription_support_requires_family_and_artifact_identity() {
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

    let error = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("degraded-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            published.artifact_id().clone(),
        ))
        .expect_err("artifact ids are not universal subscription-support fetch keys");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn duplicate_subscription_support_publication_is_idempotent_when_equivalent() {
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

    let first = store
        .publish_subscription_support(publishable.clone())
        .unwrap();
    let second = store.publish_subscription_support(publishable).unwrap();

    assert_eq!(first.artifact_id(), second.artifact_id());
    assert_eq!(store.subscription_support_counters().duplicate_retries(), 1);
}

#[test]
fn subscription_support_fetch_reports_direct_lookup_cost() {
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

    assert_eq!(fetched.fetch_report().lookup_key_count(), 1);
    assert_eq!(fetched.fetch_report().rows_read(), 1);
    assert_eq!(fetched.fetch_report().global_scan_count(), 0);
    assert!(!fetched.fetch_report().access_structure_debt());
    assert_eq!(store.subscription_support_counters().lookup_keys_used(), 1);
    assert_eq!(store.subscription_support_counters().rows_read(), 1);
}

#[test]
fn local_file_subscription_support_reopen_preserves_identity_and_digest() {
    let path = unique_test_store_path("worth-store-subscription-support-local");
    let (artifact_id, artifact_digest) = {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
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
        let published = store.publish_subscription_support(publishable).unwrap();
        (
            published.artifact_id().clone(),
            published.artifact_digest().to_string(),
        )
    };

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();

    assert_eq!(fetched.record_set().artifact_digest(), artifact_digest);
}
