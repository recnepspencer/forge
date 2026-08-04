use super::super::{
    admit_support_trust_request, check_support_trust_drift, check_support_trust_equivalence,
    translate_support_trust_inputs, RawSupportTrustRequest,
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportCertificationCoveragePlan,
    SupportBasisReceipt, SupportCatalogEpoch, SupportCertificationCoverageMatrix,
    SupportCertificationEpoch, SupportCertificationLaneDigestSet,
    SupportCertificationRowRequirement, SupportCompatibilityReceipt,
    SupportCursorCheckpointReceipt, SupportImportAdmissionReceipt, SupportOperationalLedgerEpoch,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportRetentionReceipt, SupportTrustBatchCardinality,
    SupportTrustClass, SupportTrustDriftScanPlan, SupportTrustEquivalenceEvidence,
    SupportTrustEvidenceBudget, SupportTrustFailure, SupportTrustPathClass, SupportTrustProvenance,
    SupportTrustReceiptBundle, SupportTrustReceiptStatus, SupportTrustRequestedUse,
    SupportTrustStrength,
};
use super::certification_coverage::{
    exact_certification_plan, exact_certification_requirement, exact_certification_row,
};
use super::operational_basis::{
    basis, basis_for, epochs, phase2_performance_plan, raw_phase2_request,
};
use super::operational_classification::classify_phase2;
use super::receipt_evidence::{family_role_receipt, phase2_receipts, phase2_receipts_for_basis};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

pub(super) fn phase7_expected_lane_failure(
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
) -> SupportTrustFailure {
    match row_kind {
        SubscriptionSupportAccuracyCertificationRowKind::RebuildDerivedSupportDowngraded => {
            phase7_missing_equivalence_failure(SupportTrustProvenance::Rebuilt)
        }
        SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportIdentityNotEnough => {
            phase7_missing_equivalence_failure(SupportTrustProvenance::Replicated)
        }
        SubscriptionSupportAccuracyCertificationRowKind::ImportedSupportMissingBasisNotResumable => {
            phase7_import_basis_mismatch_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected => {
            SupportCatalogEpoch::new(0).expect_err("zero catalog epoch must reject as stale")
        }
        SubscriptionSupportAccuracyCertificationRowKind::PolicyRejectedSupport => {
            phase7_operational_drift_failure(
                SubscriptionSupportOperationalVerdict::RejectedByPolicy,
            )
        }
        SubscriptionSupportAccuracyCertificationRowKind::FamilyRoleMismatchRejected => {
            phase7_family_role_mismatch_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust => {
            phase7_receipt_drift_failure(
                "basis:trust",
                "compatibility:wrong",
                "portability:trust",
            )
        }
        SubscriptionSupportAccuracyCertificationRowKind::OperationalVerdictDriftRejectsExactTrust => {
            phase7_operational_drift_failure(
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            )
        }
        SubscriptionSupportAccuracyCertificationRowKind::PortabilityDriftRejectsExactTrust => {
            phase7_receipt_drift_failure(
                "basis:trust",
                "compatibility:trust",
                "portability:wrong",
            )
        }
        SubscriptionSupportAccuracyCertificationRowKind::CoverageDriftRejectsPlatformTrust => {
            phase7_coverage_drift_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::CertificationMissingRowRejected => {
            phase7_certification_missing_row_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::CertificationDuplicateRowRejected => {
            phase7_certification_duplicate_row_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::CertificationMislabeledRowRejected => {
            phase7_certification_mislabeled_row_failure()
        }
        SubscriptionSupportAccuracyCertificationRowKind::CertificationSelfComparisonRejected => {
            SupportCertificationLaneDigestSet::new(
                "phase7:lane:same",
                "phase7:lane:same",
                "phase7:lane:replay",
            )
            .expect_err("self-comparison lane digests must reject")
        }
        SubscriptionSupportAccuracyCertificationRowKind::MultiDriftPrecedenceDeterministic => {
            phase7_receipt_drift_failure(
                "basis:wrong",
                "compatibility:wrong",
                "portability:wrong",
            )
        }
        _ => unreachable!("phase7 pass rows do not create rejection failures"),
    }
}

pub(super) fn phase7_missing_equivalence_failure(
    provenance: SupportTrustProvenance,
) -> SupportTrustFailure {
    let admitted = admit_support_trust_request(
        raw_phase2_request(SupportTrustStrength::Exact, provenance),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
        .expect_err("missing transformed equivalence must reject exact support trust")
}

pub(super) fn phase7_import_basis_mismatch_failure() -> SupportTrustFailure {
    let bundle = phase2_receipts(
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .with_import_admission(
        SupportImportAdmissionReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:wrong-import".into()),
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            "import:admission",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    );
    let admitted = admit_support_trust_request(
        RawSupportTrustRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Imported,
            SupportTrustRequestedUse::StoreLocalResume,
            SupportTrustBatchCardinality::SingleSupportArtifact,
            epochs(),
            phase2_performance_plan(),
            SupportTrustEvidenceBudget::new(4096, 9, 1).unwrap(),
        ),
        bundle,
    )
    .unwrap();
    translate_support_trust_inputs(admitted)
        .expect_err("import admission bound to a different artifact must reject")
}

pub(super) fn phase7_family_role_mismatch_failure() -> SupportTrustFailure {
    let role_mismatched_basis = basis_for(
        "basis-bound-continuation-support",
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        "artifact:trust:phase-1",
    );
    admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts_for_basis(
            role_mismatched_basis,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .expect_err("family-role receipt role drift must reject admission")
}

pub(super) fn phase7_operational_drift_failure(
    verdict: SubscriptionSupportOperationalVerdict,
) -> SupportTrustFailure {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts(SubscriptionResumeClassification::Exact, verdict),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .expect_err("operational verdict drift must reject exact trust")
}

pub(super) fn phase7_receipt_drift_failure(
    basis_digest: &str,
    compatibility_digest: &str,
    portability_digest: &str,
) -> SupportTrustFailure {
    let artifact_id = SubscriptionSupportArtifactId("artifact:trust:phase-1".into());
    let bundle = SupportTrustReceiptBundle::new(
        SupportResumeClassificationReceipt::new(
            artifact_id.clone(),
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
            artifact_id.clone(),
            basis_digest,
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCursorCheckpointReceipt::new(
            artifact_id.clone(),
            "cursor:trust:checkpoint:trust",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportCompatibilityReceipt::new(
            artifact_id.clone(),
            compatibility_digest,
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
        SupportPortabilityReceipt::new(
            artifact_id.clone(),
            portability_digest,
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    )
    .with_retention(
        SupportRetentionReceipt::new(
            artifact_id,
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
    check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .expect_err("receipt drift must reject exact trust")
}

pub(super) fn phase7_coverage_drift_failure() -> SupportTrustFailure {
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
    check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::certification_scope(
            SupportTrustPathClass::BatchCertificationPath,
            9,
            2,
            false,
        )
        .unwrap(),
    )
    .expect_err("missing coverage must reject platform trust")
}

pub(super) fn phase7_certification_missing_row_failure() -> SupportTrustFailure {
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
    SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![exact_certification_row("row:exact-control", &classified)],
    )
    .expect_err("missing required certification row must reject coverage")
}

pub(super) fn phase7_certification_duplicate_row_failure() -> SupportTrustFailure {
    let classified = classify_phase2(
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .unwrap();
    SupportCertificationCoverageMatrix::from_rows(
        &exact_certification_plan("row:exact-control"),
        vec![
            exact_certification_row("row:exact-control", &classified),
            exact_certification_row("row:exact-control", &classified),
        ],
    )
    .expect_err("duplicate certification rows must reject coverage")
}

pub(super) fn phase7_certification_mislabeled_row_failure() -> SupportTrustFailure {
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
    SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![exact_certification_row("row:exact-control", &classified)],
    )
    .expect_err("mislabeled certification row must reject coverage")
}
