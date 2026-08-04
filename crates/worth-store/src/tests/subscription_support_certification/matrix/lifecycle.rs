use super::super::{
    publish_exact, SubscriptionResumeClassification, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportClassificationPlan,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportFetchRequest, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRuntimeHandoffRequest, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_lifecycle(evidence: &mut CertificationMatrixEvidence) {
    let session_loss_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:session", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, false).unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        session_loss_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::SessionMemoryLossNonAuthoritative,
            &session_loss_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(session_loss_report);

    let tier_recall_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:tier", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_placement_unavailable();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        tier_recall_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable)
    );
    assert_eq!(
        tier_recall_report.classification(),
        SubscriptionResumeClassification::Exact
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::TierRecallCostOnly,
            &tier_recall_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(tier_recall_report);

    let runtime_handoff_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:handoff", "cursor:1", "checkpoint:1");
        let report = store
            .handoff_subscription_support_runtime(
                SubscriptionSupportRuntimeHandoffRequest::new(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                    artifact_id,
                    "runtime:source",
                    "runtime:target",
                )
                .unwrap(),
            )
            .unwrap();
        assert!(!report.delivery_session_memory_persisted());
        assert_eq!(
            store
                .subscription_support_counters()
                .runtime_handoff_count(),
            1
        );
        report
    };
    assert_eq!(
        runtime_handoff_report.durable_report().classification(),
        SubscriptionResumeClassification::Exact
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RuntimeHandoffEquivalence,
            runtime_handoff_report.durable_report(),
        )
        .unwrap(),
    );
    evidence.record_classification_report(runtime_handoff_report.durable_report().clone());
}
