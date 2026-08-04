use super::super::{
    SupportCertificationBatchScope, SupportCertificationBatchScopeKind,
    SupportCertificationCounterSnapshot, SupportCertificationCoverageMatrix,
    SupportCertificationEvidenceBundle, SupportTrustAllocationScope, SupportTrustDensityClass,
    SupportTrustFailureKind, SupportTrustPathClass, SupportTrustProvenance, SupportTrustStrength,
};
use super::certification_bundle::{first_ship_batch_scope, first_ship_counter_snapshot};
use super::certification_coverage::{
    exact_certification_plan, exact_certification_row, first_ship_certification_matrix,
    first_ship_certification_matrix_for_basis_artifact_and_materialized_family,
};
use super::operational_classification::classify_phase2;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalVerdict,
};

#[test]
fn phase5_certification_bundle_emits_first_ship_machine_checkable_outputs() {
    let matrix = first_ship_certification_matrix();
    let summary_digest = matrix.summary().certification_summary_digest().to_string();
    let bundle = SupportCertificationEvidenceBundle::new(
        "run:13.3:first-ship",
        matrix,
        first_ship_batch_scope(),
        first_ship_counter_snapshot(),
    )
    .unwrap();

    assert_eq!(bundle.certification_summary_digest(), summary_digest);
    assert!(!bundle.evidence_bundle_digest().is_empty());
    assert_eq!(bundle.counter_snapshot().coverage_row_count(), 4);
    assert_eq!(bundle.counter_snapshot().first_ship_family_count(), 4);
    assert_eq!(bundle.counter_snapshot().receipt_reuse_count(), 3);
}

#[test]
fn phase5_certification_bundle_rejects_missing_first_ship_family() {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    let plan = exact_certification_plan("row:exact-control");
    let matrix = SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![exact_certification_row("row:exact-control", &classified)],
    )
    .unwrap();
    let error = SupportCertificationEvidenceBundle::new(
        "run:13.3:incomplete",
        matrix,
        SupportCertificationBatchScope::new(
            SupportCertificationBatchScopeKind::CertificationScopeLocal,
            SupportTrustDensityClass::CertificationScopeLocal,
            SupportTrustPathClass::BatchCertificationPath,
            SupportTrustAllocationScope::BatchCertification,
            1,
            1,
            0,
            1,
        )
        .unwrap(),
        SupportCertificationCounterSnapshot::new(1, 1, 0, 1, 1, 0, 0),
    )
    .expect_err("first-ship bundle must cover all required support families");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_bundle_rejects_impostor_first_ship_family_id() {
    let matrix = first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
        "artifact:trust:phase-1",
        "materialized-narrowing-support-impostor",
    );
    let error = SupportCertificationEvidenceBundle::new(
        "run:13.3:impostor-family",
        matrix,
        first_ship_batch_scope(),
        first_ship_counter_snapshot(),
    )
    .expect_err("first-ship coverage must name canonical family ids, not only family kinds");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_bundle_rejects_receipt_reuse_counter_mismatch() {
    let matrix = first_ship_certification_matrix();
    let error = SupportCertificationEvidenceBundle::new(
        "run:13.3:bad-counters",
        matrix,
        first_ship_batch_scope(),
        SupportCertificationCounterSnapshot::new(4, 4, 2, 4, 1, 0, 0),
    )
    .expect_err("counter snapshot must prove declared receipt reuse");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase5_certification_batch_scope_rejects_foreground_or_mismatched_density() {
    let error = SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::CertificationScopeLocal,
        SupportTrustDensityClass::FamilyLocal,
        SupportTrustPathClass::BatchCertificationPath,
        SupportTrustAllocationScope::BatchCertification,
        4,
        4,
        3,
        1,
    )
    .expect_err("certification scope batches must declare certification-scope density");
    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );

    let error = SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::CertificationScopeLocal,
        SupportTrustDensityClass::CertificationScopeLocal,
        SupportTrustPathClass::ForegroundResumeTrustPath,
        SupportTrustAllocationScope::ForegroundScratch,
        4,
        4,
        3,
        1,
    )
    .expect_err("foreground resume paths cannot build certification bundles");
    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );
}
