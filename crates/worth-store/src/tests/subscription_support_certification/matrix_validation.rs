use super::{
    fetched_exact_report, publish_exact, retention_basis, retention_batch_request, StoreErrorKind,
    SubscriptionSupportCatalog, SubscriptionSupportCertificationBundle,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SubscriptionSupportClassificationPlan, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionDecision, SupportActionId, WORTHStoreBuilder,
};

#[test]
fn durable_subscription_support_certification_matrix_rejects_missing_phase_5b_floor() {
    let report = fetched_exact_report(&mut WORTHStoreBuilder::new().in_memory().build().unwrap());
    let error = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        std::slice::from_ref(&report),
        vec![
            SubscriptionSupportCertificationLaneOutcome::from_classification_report(
                SubscriptionSupportCertificationLaneKind::ExactResumeControl,
                &report,
            )
            .unwrap(),
        ],
    )
    .expect_err("certification must reject matrices that miss the Phase 5B floor");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn durable_subscription_support_certification_matrix_rejects_mislabeled_lane_evidence() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id = publish_exact(&mut store, "basis:mislabeled", "cursor:1", "checkpoint:1");
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_support_artifact_digest("artifact:drift")
        .unwrap();
    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    let error = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        std::slice::from_ref(&report),
        vec![
            SubscriptionSupportCertificationLaneOutcome::from_classification_report(
                SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
                &report,
            )
            .unwrap(),
        ],
    )
    .expect_err("certification must reject a support-digest report mislabeled as cursor drift");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn durable_subscription_support_certification_matrix_rejects_mislabeled_retention_lane() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(retention_batch_request(
            SupportActionId::new("support-retention:mislabeled").unwrap(),
            vec![retention_basis("mislabeled")],
            SubscriptionSupportRetentionDecision::retain_exact(),
        ))
        .unwrap();
    let report = store
        .publish_subscription_support_retention_consequence(plan)
        .unwrap();

    let error = SubscriptionSupportCertificationLaneOutcome::from_retention_report(
        SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy,
        &report,
        store.subscription_support_counters(),
    )
    .and_then(|lane| {
        SubscriptionSupportCertificationBundle::from_lane_outcomes(
            &SubscriptionSupportCatalog::first_ship(),
            SubscriptionSupportCounterSnapshot::default(),
            &[],
            vec![lane],
        )
    })
    .expect_err("retained support cannot masquerade as expired policy");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
