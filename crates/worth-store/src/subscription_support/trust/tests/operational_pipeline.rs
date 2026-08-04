use super::super::{
    admit_support_trust_request, check_support_trust_drift, translate_support_trust_inputs,
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportRetentionReceipt, SupportStalenessVerdict,
    SupportTrustDriftCause, SupportTrustDriftScanPlan, SupportTrustFailureKind,
    SupportTrustProvenance, SupportTrustReceiptBundle, SupportTrustReceiptStatus,
    SupportTrustStrength,
};
use super::operational_basis::{basis, raw_phase2_request};
use super::operational_classification::classify_phase2;
use super::receipt_evidence::family_role_receipt;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId,
    SubscriptionSupportOperationalVerdict,
};

#[test]
fn phase2_exact_pipeline_classifies_only_after_receipts_and_checks() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::Exact
    );
    assert_eq!(classified.counter_snapshot().exact_trust_count(), 1);
    assert_eq!(classified.cost_surface().receipts_consumed(), 8);
    assert_eq!(classified.cost_surface().drift_checks_performed(), 8);
    assert_eq!(classified.cost_surface().index_probes(), 2);
}

#[test]
fn phase2_degraded_pipeline_cannot_satisfy_exact_request() {
    let error = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Degraded,
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
    )
    .expect_err("degraded receipts cannot satisfy exact trust requests");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
    );
}

#[test]
fn phase2_rebuild_pipeline_reports_rebuild_only_for_rebuild_request() {
    let classified = classify_phase2(
        SupportTrustStrength::RebuildOnly,
        SupportTrustProvenance::Rebuilt,
        SubscriptionResumeClassification::RebuildRequired,
        SubscriptionSupportOperationalVerdict::RebuildRequired,
    )
    .unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::RebuildOnly
    );
    assert_eq!(
        classified.counter_snapshot().rebuild_derived_trust_count(),
        1
    );
}

#[test]
fn phase2_drift_check_rejects_digest_mismatch_before_classification() {
    let bundle = SupportTrustReceiptBundle::new(
        SupportResumeClassificationReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SubscriptionResumeClassification::Exact,
            "resume:proof",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportOperationalVerdictReceipt::new(
            basis(),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "operational:proof",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        family_role_receipt(),
        SupportBasisReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "basis:wrong",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCursorCheckpointReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "cursor:trust:checkpoint:trust",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCompatibilityReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "compatibility:trust",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportPortabilityReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "portability:trust",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    )
    .with_retention(
        SupportRetentionReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            "retention:trust",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    );
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        bundle,
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();

    let error = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .expect_err("drift check must reject stale basis receipt");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustBasisMismatch
    );
    let drift_report = error
        .drift_report()
        .expect("basis drift failures must retain the deterministic drift report");
    assert_eq!(
        drift_report.primary_cause(),
        Some(SupportTrustDriftCause::Basis)
    );
    assert_eq!(
        drift_report.staleness_verdict(),
        SupportStalenessVerdict::StaleRejected
    );
}
