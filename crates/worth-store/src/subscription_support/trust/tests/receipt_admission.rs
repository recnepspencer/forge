use super::super::{
    admit_support_trust_request, translate_support_trust_inputs, RawSupportTrustRequest,
    SupportBasisReceipt, SupportCompatibilityReceipt, SupportCursorCheckpointReceipt,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt,
    SupportResumeClassificationReceipt, SupportRetentionReceipt, SupportTrustBatchCardinality,
    SupportTrustComplexityContract, SupportTrustComplexityStatus, SupportTrustEvidenceBudget,
    SupportTrustFailureKind, SupportTrustProvenance, SupportTrustReceiptBundle,
    SupportTrustReceiptStatus, SupportTrustRecoveryPosture, SupportTrustRequestedUse,
    SupportTrustStrength,
};
use super::operational_basis::{basis, epochs, phase2_performance_plan, raw_phase2_request};
use super::receipt_evidence::{family_role_receipt, phase2_receipts};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn phase2_rebuild_requires_maintenance_receipt_before_translation() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::RebuildOnly,
            SupportTrustProvenance::Rebuilt,
        ),
        SupportTrustReceiptBundle::new(
            SupportResumeClassificationReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                SubscriptionResumeClassification::RebuildRequired,
                "resume:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            SupportOperationalVerdictReceipt::new(
                basis(),
                SubscriptionSupportOperationalVerdict::RebuildRequired,
                "operational:proof",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
            family_role_receipt(),
            SupportBasisReceipt::new(
                SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
                "basis:trust",
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
        ),
    )
    .unwrap();

    let error = translate_support_trust_inputs(admitted)
        .expect_err("rebuild-derived support trust requires maintenance proof");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase2_imported_support_requires_target_admission_receipt() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::RebuildOnly,
            SupportTrustProvenance::Imported,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::RebuildRequired,
            SubscriptionSupportOperationalVerdict::RebuildRequired,
        ),
    )
    .unwrap();

    let error = translate_support_trust_inputs(admitted)
        .expect_err("imported support requires target admission proof");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase2_receipt_bundle_must_fit_evidence_budget() {
    let request = RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SupportTrustRequestedUse::StoreLocalResume,
        SupportTrustBatchCardinality::SingleSupportArtifact,
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(4096, 7, 1).unwrap(),
    );

    let error = admit_support_trust_request(
        request,
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .expect_err("exact support proof bundle has eight receipts");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
    );
}

#[test]
fn phase2_receipt_bundle_byte_budget_rejects_oversized_proofs() {
    let request = RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SupportTrustRequestedUse::StoreLocalResume,
        SupportTrustBatchCardinality::SingleSupportArtifact,
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(16, 8, 1).unwrap(),
    );

    let error = admit_support_trust_request(
        request,
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .expect_err("proof digest bytes must be accounted before translation");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
    );
}

#[test]
fn phase2_contextual_receipts_must_match_requested_artifact() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        SupportTrustReceiptBundle::new(
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
                "basis:trust",
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
                SubscriptionSupportArtifactId("artifact:trust:other".into()),
                "retention:trust",
                SupportTrustReceiptStatus::Proven,
            )
            .unwrap(),
        ),
    )
    .unwrap();

    let error = translate_support_trust_inputs(admitted)
        .expect_err("foreign-artifact retention proof must not satisfy exact support trust");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustBasisMismatch
    );
}

#[test]
fn phase2_certified_platform_claim_waits_for_coverage_phase() {
    let request = RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SupportTrustRequestedUse::CertifiedPlatformClaim,
        SupportTrustBatchCardinality::SingleSupportArtifact,
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(4096, 8, 1).unwrap(),
    );

    let error = admit_support_trust_request(
        request,
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .expect_err("operational trust admission cannot certify platform claims");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
    assert_eq!(
        error.recovery_posture(),
        SupportTrustRecoveryPosture::RerunCertification
    );
}

#[test]
fn phase2_batch_cardinality_must_match_declared_density() {
    let request = RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SupportTrustRequestedUse::StoreLocalResume,
        SupportTrustBatchCardinality::FamilyRoleBatch { artifact_count: 2 },
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(4096, 8, 2).unwrap(),
    );

    let error = admit_support_trust_request(
        request,
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .expect_err("family-role batches cannot pretend to be scalar density");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );
}

#[test]
fn complexity_contracts_name_bounds_and_status() {
    let contract = SupportTrustComplexityContract::verified(
        "support_trust_classification",
        "O(index_probes + receipts + drift_checks + equivalence_checks)",
        1,
        4,
        0,
    )
    .unwrap();

    assert_eq!(contract.status(), SupportTrustComplexityStatus::Verified);
    assert_eq!(contract.max_global_scans(), 0);
}
