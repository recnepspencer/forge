use super::{
    unique_test_sqlite_path, SubscriptionResumeClassification, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest, WORTHStoreBuilder,
};

use super::raw_exact;

#[test]
fn sqlite_subscription_support_reopen_preserves_identity_and_digest() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-sqlite");
    let (artifact_id, artifact_digest) = {
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
        let published = store.publish_subscription_support(publishable).unwrap();
        (
            published.artifact_id().clone(),
            published.artifact_digest().to_string(),
        )
    };

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();

    assert_eq!(fetched.record_set().artifact_digest(), artifact_digest);
}

#[test]
fn sqlite_subscription_support_reopen_classifies_exact_resume_from_fetched_evidence() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-sqlite-classify-exact");
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

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true).unwrap();
    let report = reopened
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(report.primary_cause(), None);
    assert_eq!(report.cost_surface().decoded_payload_bytes(), 128);
    assert_eq!(report.cost_surface().scanned_support_rows(), 1);
    assert_eq!(
        reopened
            .subscription_support_counters()
            .exact_classifications(),
        1
    );
}

#[test]
fn sqlite_subscription_support_restart_reconstruction_is_shard_bounded() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-restart-shard");
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

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                ),
                8,
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(report.support_rows_read(), 1);
    assert_eq!(report.restart_shards_touched(), 1);
    assert_eq!(report.global_scan_count(), 0);
    assert_eq!(report.reports().len(), 1);
    assert_eq!(report.reports()[0].artifact_id(), &artifact_id);
    assert_eq!(
        report.reports()[0].classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(
        report.reports()[0].cost_surface().restart_shards_touched(),
        1
    );
    assert_eq!(
        report.reports()[0].cost_surface().decoded_payload_bytes(),
        0
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .restart_reconstruction_count(),
        1
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .restart_global_scan_count(),
        0
    );
}
