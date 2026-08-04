use super::super::{
    ExactSupportTrustWitness, OperationalSupportTrustReport, SupportRoleTrustPosture,
    SupportTrustAccessIndexKind, SupportTrustAccessPath, SupportTrustAccessStructurePlan,
    SupportTrustAllocationScope, SupportTrustClass, SupportTrustClassificationPlan,
    SupportTrustClassificationReport, SupportTrustCloneBoundary, SupportTrustDensityClass,
    SupportTrustEvidenceBudget, SupportTrustFailureKind, SupportTrustFreshnessWitness,
    SupportTrustPathClass, SupportTrustPerformancePlan, SupportTrustProvenance,
    SupportTrustStrength, SupportTrustTranslationPlan, SupportTrustUseBoundary,
    UncertifiedSupportTrustPosture,
};
use super::operational_basis::{basis, epochs};
use super::operational_classification::exact_translation;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn uncertified_posture_preserves_operational_boundary() {
    let witness = ExactSupportTrustWitness::from_exact_translation(
        exact_translation(),
        SupportTrustProvenance::NativePublished,
        SupportTrustFreshnessWitness::new(epochs()),
    )
    .unwrap();
    let report = OperationalSupportTrustReport::from_exact_witness(witness);
    let posture = UncertifiedSupportTrustPosture::new(report);

    assert_eq!(
        posture.report().use_boundary(),
        SupportTrustUseBoundary::StoreLocalOperational
    );
}

#[test]
fn mismatched_translation_lowers_to_rejected_for_drift_audit() {
    let plan = SupportTrustTranslationPlan::from_resume_and_operational(
        basis(),
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
    )
    .expect("translation should preserve mismatched receipts for drift localization");

    assert!(matches!(plan, SupportTrustTranslationPlan::Rejected { .. }));
}

#[test]
fn performance_plan_rejects_store_global_and_foreground_certification() {
    let error = SupportTrustPerformancePlan::new(
        SupportTrustPathClass::BatchCertificationPath,
        SupportTrustDensityClass::StoreGlobalRejected,
        SupportTrustAccessPath::Rejected,
        SupportTrustAllocationScope::BatchCertification,
        1,
        1,
        0,
        0,
        SupportTrustCloneBoundary::NoClone,
    )
    .expect_err("store-global trust work is rejected in Phase 1");
    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );

    let error = SupportTrustPerformancePlan::new(
        SupportTrustPathClass::ForegroundResumeTrustPath,
        SupportTrustDensityClass::SingleSupportArtifact,
        SupportTrustAccessPath::PointLookup,
        SupportTrustAllocationScope::BatchCertification,
        1,
        1,
        0,
        0,
        SupportTrustCloneBoundary::NoClone,
    )
    .expect_err("foreground resume trust cannot allocate in batch certification scope");
    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
    );
}

#[test]
fn performance_plan_requires_clone_boundary_when_clone_count_is_nonzero() {
    let error = SupportTrustPerformancePlan::new(
        SupportTrustPathClass::BatchCertificationPath,
        SupportTrustDensityClass::CertificationScopeLocal,
        SupportTrustAccessPath::BatchLookup,
        SupportTrustAllocationScope::BatchCertification,
        3,
        8,
        2,
        1,
        SupportTrustCloneBoundary::NoClone,
    )
    .expect_err("clone count must name a semantic boundary");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded
    );
}

#[test]
fn classification_plan_carries_role_epoch_and_performance_contract() {
    let posture = SupportRoleTrustPosture::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
    );
    let performance_plan = SupportTrustPerformancePlan::new(
        SupportTrustPathClass::ForegroundResumeTrustPath,
        SupportTrustDensityClass::SingleSupportArtifact,
        SupportTrustAccessPath::PointLookup,
        SupportTrustAllocationScope::ForegroundScratch,
        1,
        1,
        0,
        0,
        SupportTrustCloneBoundary::NoClone,
    )
    .unwrap();
    let plan = SupportTrustClassificationPlan::new(posture.clone(), epochs(), performance_plan);
    let report = SupportTrustClassificationReport::from_plan(
        plan,
        SupportTrustClass::ExactSupportTrusted,
        None,
    );

    assert_eq!(report.posture(), &posture);
    assert_eq!(report.epoch(), epochs());
    assert_eq!(report.trust_class(), SupportTrustClass::ExactSupportTrusted);
}

#[test]
fn access_structure_and_evidence_budgets_reject_unbounded_shapes() {
    let access_error = SupportTrustAccessStructurePlan::new(
        SupportTrustAccessIndexKind::CertificationRow,
        SupportTrustAccessPath::Rejected,
        "certification-index-rebuild",
        "certification-epoch",
        "support_trust_certification_row_reads",
    )
    .expect_err("required trust indexes cannot lower to rejected access");

    assert_eq!(
        access_error.kind(),
        SupportTrustFailureKind::SupportTrustAccessStructureDebt
    );

    let budget = SupportTrustEvidenceBudget::new(1024, 4, 2).unwrap();
    assert!(budget.admits(1024, 4, 2));
    assert!(!budget.admits(1025, 4, 2));
}
