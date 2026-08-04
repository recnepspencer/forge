use super::super::{
    admit_support_trust_request, check_support_trust_drift, check_support_trust_equivalence,
    classify_operational_support_trust, translate_support_trust_inputs, SupportStalenessVerdict,
    SupportTrustDriftCause, SupportTrustDriftLocality, SupportTrustDriftReport,
    SupportTrustDriftScanPlan, SupportTrustEquivalenceEvidence, SupportTrustFailureKind,
    SupportTrustPathClass, SupportTrustProvenance, SupportTrustStrength,
    SupportTrustSuppressedCause,
};
use super::operational_basis::raw_phase2_request;
use super::receipt_evidence::phase2_receipts;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalVerdict,
};

#[test]
fn phase4_multi_drift_report_orders_primary_and_suppressed_causes() {
    let plan = SupportTrustDriftScanPlan::foreground_support_identity();
    let report = SupportTrustDriftReport::from_observed_causes(
        &plan,
        [
            (
                SupportTrustDriftCause::Portability,
                SupportTrustDriftLocality::SupportIdentity,
            ),
            (
                SupportTrustDriftCause::Basis,
                SupportTrustDriftLocality::BasisLocal,
            ),
            (
                SupportTrustDriftCause::Compatibility,
                SupportTrustDriftLocality::CompatibilityEpoch,
            ),
        ],
    );

    assert_eq!(report.primary_cause(), Some(SupportTrustDriftCause::Basis));
    assert_eq!(
        report
            .suppressed_causes()
            .iter()
            .map(SupportTrustSuppressedCause::cause)
            .collect::<Vec<_>>(),
        vec![
            SupportTrustDriftCause::Compatibility,
            SupportTrustDriftCause::Portability
        ]
    );
    assert_eq!(
        report.staleness_verdict(),
        SupportStalenessVerdict::StaleRejected
    );
}

#[test]
fn phase4_certification_coverage_drift_rejects_platform_scope() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let plan = SupportTrustDriftScanPlan::certification_scope(
        SupportTrustPathClass::BatchCertificationPath,
        9,
        2,
        false,
    )
    .unwrap();

    let error = check_support_trust_drift(translated, plan)
        .expect_err("missing certification coverage rejects platform trust");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
    let drift_report = error
        .drift_report()
        .expect("coverage drift failures must retain the deterministic drift report");
    assert_eq!(
        drift_report.primary_cause(),
        Some(SupportTrustDriftCause::CertificationCoverage)
    );
    assert_eq!(drift_report.coverage_drift_count(), 1);
}

#[test]
fn phase4_operational_verdict_drift_is_reachable_and_audited() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();

    let error = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .expect_err("resume/operational disagreement must localize as operational drift");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch
    );
    let drift_report = error
        .drift_report()
        .expect("operational drift failures must retain the deterministic drift report");
    assert_eq!(
        drift_report.primary_cause(),
        Some(SupportTrustDriftCause::OperationalVerdict)
    );
    assert_eq!(drift_report.stale_rejection_count(), 1);
}

#[test]
fn phase4_placement_cost_drift_is_advisory_only() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let plan = SupportTrustDriftScanPlan::new(
        SupportTrustDriftLocality::PlacementCostAdvisory,
        SupportTrustPathClass::ForegroundResumeTrustPath,
        8,
        1,
    )
    .unwrap();
    let drift_checked = check_support_trust_drift(translated, plan).unwrap();
    assert_eq!(
        drift_checked.drift_report().staleness_verdict(),
        SupportStalenessVerdict::PlacementCostAdvisory
    );
    let equivalence_checked =
        check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
            .unwrap();
    let classified = classify_operational_support_trust(equivalence_checked).unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::Exact
    );
    assert_eq!(classified.counter_snapshot().placement_advisory_count(), 1);
}

#[test]
fn phase4_store_global_drift_scan_plan_rejects_before_execution() {
    let error = SupportTrustDriftScanPlan::new(
        SupportTrustDriftLocality::SupportIdentity,
        SupportTrustPathClass::RoadmapHandoffPath,
        8,
        1,
    )
    .expect_err("drift checks cannot hide global roadmap handoff scans");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );
}
