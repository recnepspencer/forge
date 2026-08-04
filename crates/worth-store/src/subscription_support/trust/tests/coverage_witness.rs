use super::super::{
    check_support_trust_coverage, classify_certified_support_trust,
    SubscriptionSupportCertificationCoveragePlan, SupportCertificationCorpusVersion,
    SupportCertificationCoverageMatrix, SupportCertificationEpoch,
    SupportCertificationEvidenceBundle, SupportCertificationGapReport,
    SupportCertificationLaneDigestSet, SupportCertificationRow, SupportCertificationRowEvidence,
    SupportCertificationRowRequirement, SupportOperationalLedgerEpoch,
    SupportTrustCertificationStamp, SupportTrustClass, SupportTrustFailureKind,
    SupportTrustProvenance, SupportTrustStrength,
};
use super::certification_bundle::{
    first_ship_batch_scope, first_ship_certification_bundle, first_ship_counter_snapshot,
};
use super::certification_coverage::{
    certification_lanes, exact_certification_plan, exact_certification_requirement,
    exact_certification_row, first_ship_certification_matrix_for_basis_artifact,
};
use super::operational_classification::classify_phase2;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn phase5_certification_coverage_witness_enables_certified_exact_trust() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let bundle = first_ship_certification_bundle();
    let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-5",
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        "row:basis-bound-exact",
        evidence_bundle_digest,
    )
    .unwrap();

    let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
    let certified = classify_certified_support_trust(coverage_checked, stamp).unwrap();

    assert_eq!(
        certified.report().trust_class(),
        SupportTrustClass::ExactSupportTrusted
    );
    assert_eq!(
        certified.report().certification_stamp().row_id(),
        "row:basis-bound-exact"
    );
    assert_eq!(certified.coverage_witness().summary().row_count(), 4);
}

#[test]
fn phase5_certification_rejects_stamp_not_bound_to_checked_bundle() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let coverage_checked =
        check_support_trust_coverage(classified, first_ship_certification_bundle()).unwrap();
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-5",
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        "row:basis-bound-exact",
        "bundle:digest:WORTHd",
    )
    .unwrap();

    let error = classify_certified_support_trust(coverage_checked, stamp)
        .expect_err("certification stamp must name the checked evidence bundle");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_rejects_stamp_for_different_covered_row() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let bundle = first_ship_certification_bundle();
    let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
    let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-5",
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        "row:materialized-narrowing-exact",
        evidence_bundle_digest,
    )
    .unwrap();

    let error = classify_certified_support_trust(coverage_checked, stamp)
        .expect_err("certification stamp row id must name the row that covered the report");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_coverage_is_artifact_and_basis_bound() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let bundle = SupportCertificationEvidenceBundle::new(
        "run:13.3:wrong-artifact",
        first_ship_certification_matrix_for_basis_artifact("artifact:first-ship:other"),
        first_ship_batch_scope(),
        first_ship_counter_snapshot(),
    )
    .unwrap();

    let error = check_support_trust_coverage(classified, bundle)
        .expect_err("same family and posture cannot cover a different support artifact");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_matrix_rejects_duplicate_rows() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let plan = exact_certification_plan("row:exact-control");

    let error = SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![
            exact_certification_row("row:exact-control", &classified),
            exact_certification_row("row:exact-control", &classified),
        ],
    )
    .expect_err("duplicate certification rows cannot complete coverage");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_gap_report_names_missing_required_rows() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let plan = SubscriptionSupportCertificationCoveragePlan::new(
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        vec![
            exact_certification_requirement("row:exact-control"),
            exact_certification_requirement("row:hostile-stale"),
        ],
    )
    .unwrap();
    let rows = vec![exact_certification_row("row:exact-control", &classified)];
    let gap = SupportCertificationGapReport::from_plan_and_rows(&plan, &rows);

    assert_eq!(gap.missing_row_ids(), &["row:hostile-stale".to_string()]);
    let error = SupportCertificationCoverageMatrix::from_rows(&plan, rows)
        .expect_err("missing required rows cannot complete coverage");
    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_rows_reject_self_comparison() {
    let error = SupportCertificationLaneDigestSet::new("lane:same", "lane:same", "lane:replay")
        .expect_err("control and hostile lanes cannot be the same run");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_matrix_rejects_mislabeled_trust_posture() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let plan = SubscriptionSupportCertificationCoveragePlan::new(
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        vec![SupportCertificationRowRequirement::new(
            "row:exact-control",
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustClass::DegradedSupportTrusted,
            SupportTrustStrength::Degraded,
            SupportTrustProvenance::NativePublished,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionResumeClassification::Exact,
            None,
        )
        .unwrap()],
    )
    .unwrap();

    let error = SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![exact_certification_row("row:exact-control", &classified)],
    )
    .expect_err("row labels cannot certify a different trust posture");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_row_rejects_digest_mismatch() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let evidence = SupportCertificationRowEvidence::from_operational_report(
        "row:exact-control",
        classified.report(),
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        certification_lanes(),
        "artifact:digest:exact",
        "subscription-support:digest:exact",
        "diagnostics:digest:exact",
        None,
        Vec::new(),
    )
    .unwrap()
    .with_declared_row_digest("digest:WORTHd")
    .unwrap();

    let error = SupportCertificationRow::new(evidence)
        .expect_err("declared row digest must recompute from structured evidence");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}
