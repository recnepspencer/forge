use super::super::{
    SubscriptionSupportAccuracyCertificationRow, SubscriptionSupportAccuracyCertificationRowKind,
    SubscriptionSupportAccuracyCertificationSuite, SubscriptionSupportAccuracyLaneEvidence,
    SubscriptionSupportAccuracyLaneEvidenceSet, SupportCatalogEpoch, SupportOperationalLedgerEpoch,
    SupportTrustFailureKind, SupportTrustProvenance,
};
use super::accuracy_failure_evidence::phase7_expected_lane_failure;
use super::accuracy_lane_evidence::phase7_required_suite_rows;
use super::accuracy_lane_evidence::{
    phase7_certified_transformed_exact_report, phase7_lane_evidence,
};
use super::domain_handoff::phase7_suite_artifacts;

#[test]
fn phase7_named_suite_rejects_missing_required_row() {
    let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
    let lane_evidence = phase7_lane_evidence();
    let mut rows = phase7_required_suite_rows(
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    );
    rows.retain(|row| {
        row.row_kind()
            != SubscriptionSupportAccuracyCertificationRowKind::DomainAiDegradedSupportTrust
    });

    let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
        rows,
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    )
    .expect_err("missing named Phase 7 row must reject suite completion");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_named_suite_rejects_duplicate_required_row() {
    let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
    let lane_evidence = phase7_lane_evidence();
    let mut rows = phase7_required_suite_rows(
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    );
    rows.push(
        SubscriptionSupportAccuracyCertificationRow::new(
            SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
            rows.iter()
                .find(|row| {
                    row.row_kind()
                        == SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl
                })
                .unwrap()
                .row_digest(),
            0,
            0,
        )
        .unwrap(),
    );

    let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
        rows,
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    )
    .expect_err("duplicate named Phase 7 row must reject suite completion");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_named_suite_rejects_tampered_artifact_row_evidence() {
    let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
    let lane_evidence = phase7_lane_evidence();
    let mut rows = phase7_required_suite_rows(
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    );
    let exact = rows
        .iter_mut()
        .find(|row| {
            row.row_kind()
                == SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl
        })
        .expect("exact suite row should be present");
    *exact = SubscriptionSupportAccuracyCertificationRow::new(
        SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
        "tampered:exact-control:not-from-phase-artifacts",
        0,
        0,
    )
    .unwrap();

    let error = SubscriptionSupportAccuracyCertificationSuite::from_rows_and_phase_artifacts(
        rows,
        &evidence_bundle,
        &generic,
        &domain,
        &handoff,
        &lane_evidence,
    )
    .expect_err("suite row evidence must be recomputed from supplied artifacts");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_named_suite_rejects_overclaim_or_global_scan_debt_rows() {
    let exact_overclaim = SubscriptionSupportAccuracyCertificationRow::new(
        SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
        "phase7:evidence:overclaim",
        1,
        0,
    )
    .expect_err("exact overclaim counter must reject named suite row");
    let global_scan = SubscriptionSupportAccuracyCertificationRow::new(
        SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
        "phase7:evidence:global-scan",
        0,
        1,
    )
    .expect_err("global scan debt counter must reject named suite row");

    assert_eq!(
        exact_overclaim.kind(),
        SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
    );
    assert_eq!(
        global_scan.kind(),
        SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
    );
}

#[test]
fn phase7_named_suite_rejects_missing_hostile_lane_evidence() {
    let mut lanes = phase7_lane_evidence().lanes().to_vec();
    lanes.retain(|lane| {
        lane.row_kind()
            != SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust
    });
    let error = SubscriptionSupportAccuracyLaneEvidenceSet::new(lanes)
        .expect_err("every hostile named suite row requires explicit lane evidence");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_named_suite_rejects_misclassified_hostile_lane_outcome() {
    let wrong_failure = phase7_expected_lane_failure(
        SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable,
    );
    let error = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
        SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust,
        &wrong_failure,
    )
    .expect_err("compatibility drift lane must carry compatibility mismatch evidence");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_certified_pass_lane_derives_from_matching_certified_report() {
    let replicated = phase7_certified_transformed_exact_report(
        SupportTrustProvenance::Replicated,
        "row:phase7:replicated-exact",
    );
    let lane = SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
        SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
        &replicated,
        "phase7:diagnostics:replicated",
        "phase7:counter:replicated",
    )
    .expect("replicated exact suite lane must derive from certified replicated exact report");

    assert_eq!(
        lane.row_kind(),
        SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence
    );
    assert!(!lane.evidence_digest().is_empty());
}

#[test]
fn phase7_certified_pass_lane_rejects_report_posture_drift() {
    let replicated = phase7_certified_transformed_exact_report(
        SupportTrustProvenance::Replicated,
        "row:phase7:replicated-exact",
    );
    let error = SubscriptionSupportAccuracyLaneEvidence::certified_pass_from_report(
        SubscriptionSupportAccuracyCertificationRowKind::MigratedSupportExactEquivalence,
        &replicated,
        "phase7:diagnostics:wrong-posture",
        "phase7:counter:wrong-posture",
    )
    .expect_err("migrated exact suite lane cannot consume replicated exact certification");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_counter_pass_lanes_derive_from_zero_debt_evidence_bundle() {
    let (evidence_bundle, _, _, _) = phase7_suite_artifacts();
    let overclaim =
        SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
            SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero,
            &evidence_bundle,
        )
        .expect("zero exact-overclaim counter lane must derive from evidence bundle counters");
    let global_scan =
        SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
            SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden,
            &evidence_bundle,
        )
        .expect("zero global-scan counter lane must derive from evidence bundle counters");
    let wrong_row =
        SubscriptionSupportAccuracyLaneEvidence::certified_counter_pass_from_evidence_bundle(
            SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
            &evidence_bundle,
        )
        .expect_err("non-counter suite rows cannot be certified from bundle counters");

    assert_eq!(
        overclaim.row_kind(),
        SubscriptionSupportAccuracyCertificationRowKind::ForbiddenExactOverclaimZero
    );
    assert_eq!(
        global_scan.row_kind(),
        SubscriptionSupportAccuracyCertificationRowKind::GlobalScanDebtForbidden
    );
    assert_eq!(
        wrong_row.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_rejection_lane_digest_is_bound_to_failure_evidence() {
    let catalog_failure = SupportCatalogEpoch::new(0).expect_err("zero catalog epoch must reject");
    let ledger_failure = SupportOperationalLedgerEpoch::new(0)
        .expect_err("zero operational ledger epoch must reject");

    let catalog_lane = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
        SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
        &catalog_failure,
    )
    .unwrap();
    let ledger_lane = SubscriptionSupportAccuracyLaneEvidence::typed_rejection_from_failure(
        SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
        &ledger_failure,
    )
    .unwrap();

    assert_ne!(
        catalog_lane.evidence_digest(),
        ledger_lane.evidence_digest()
    );
}
