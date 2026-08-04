use super::super::{
    publish_exact, StoreErrorKind, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportClassificationPlan,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportDriftCause, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_drift(evidence: &mut CertificationMatrixEvidence) {
    let basis_drift_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:control", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_basis_digest("basis:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        basis_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::NotResumableBasisDrift,
            &basis_drift_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(basis_drift_report);

    let cursor_drift_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:cursor", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_cursor_digest("cursor:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        cursor_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
            &cursor_drift_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(cursor_drift_report);

    let support_digest_drift_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:support", "cursor:1", "checkpoint:1");
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
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        support_digest_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::SupportDigestDrift,
            &support_digest_drift_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(support_digest_drift_report);

    let compatibility_drift_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id =
            publish_exact(&mut store, "basis:compat-only", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_compatibility_digest("compatibility:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        compatibility_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::CompatibilityDrift,
            &compatibility_drift_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(compatibility_drift_report);

    let cross_family_reuse_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id =
            publish_exact(&mut store, "basis:cross-family", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_expected_family_kind(SubscriptionSupportFamilyKind::MaterializedNarrowingSupport);
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        cross_family_reuse_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportFamilyMismatch)
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::CrossFamilyReuseRejected,
            &cross_family_reuse_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(cross_family_reuse_report);

    let basis_precedence_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:precedence", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_basis_digest("basis:drift")
            .unwrap()
            .with_cursor_digest("cursor:drift")
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        basis_precedence_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift)
    );
    assert_eq!(
        basis_precedence_report.suppressed_causes(),
        &[
            SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift,
            SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch
        ]
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::MultiDriftBasisPrecedence,
            &basis_precedence_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(basis_precedence_report);

    let cursor_only_error = StoreErrorKind::SubscriptionSupportClassificationViolation;
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::CursorOnlyExactResumeRejected,
            cursor_only_error,
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );
}
