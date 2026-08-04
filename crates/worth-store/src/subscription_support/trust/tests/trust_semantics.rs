use super::super::{
    CertifiedSupportTrustReport, ExactSupportTrustWitness, OperationalSupportTrustReport,
    SupportCertificationCorpusVersion, SupportCertificationEpoch, SupportTrustCertificationStamp,
    SupportTrustClass, SupportTrustEquivalenceWitness, SupportTrustFailureKind,
    SupportTrustFreshnessWitness, SupportTrustProvenance, SupportTrustRecoveryPosture,
    SupportTrustStrength, SupportTrustUseBoundary,
};
use super::operational_basis::epochs;
use super::operational_classification::exact_translation;
use crate::subscription_support::{
    SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn exact_trust_is_strength_and_provenance_not_one_overloaded_enum() {
    let witness = ExactSupportTrustWitness::from_exact_translation(
        exact_translation(),
        SupportTrustProvenance::NativePublished,
        SupportTrustFreshnessWitness::new(epochs()),
    )
    .unwrap();
    let report = OperationalSupportTrustReport::from_exact_witness(witness);

    assert_eq!(report.trust_strength(), SupportTrustStrength::Exact);
    assert_eq!(report.provenance(), SupportTrustProvenance::NativePublished);
    assert_eq!(report.trust_class(), SupportTrustClass::ExactSupportTrusted);
    assert_eq!(
        report.use_boundary(),
        SupportTrustUseBoundary::StoreLocalOperational
    );
}

#[test]
fn replicated_exact_requires_equivalence_witness() {
    let error = ExactSupportTrustWitness::from_exact_translation(
        exact_translation(),
        SupportTrustProvenance::Replicated,
        SupportTrustFreshnessWitness::new(epochs()),
    )
    .expect_err("replicated exact trust requires an equivalence witness");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustEquivalenceMissing
    );
    assert_eq!(
        error.recovery_posture(),
        SupportTrustRecoveryPosture::RetryWithFresherReceipts
    );
}

#[test]
fn transformed_exact_trust_requires_family_bound_equivalence() {
    let translation = exact_translation();
    let equivalence = SupportTrustEquivalenceWitness::new(
        translation.basis().clone(),
        SubscriptionSupportFamilyId::new("replicated-continuation-support").unwrap(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        "equivalence:digest",
    )
    .unwrap();

    let witness = ExactSupportTrustWitness::from_equivalent_operational_basis(
        translation,
        SupportTrustProvenance::Replicated,
        SupportTrustFreshnessWitness::new(epochs()),
        equivalence,
    )
    .unwrap();

    assert_eq!(witness.trust().strength(), SupportTrustStrength::Exact);
    assert_eq!(
        witness.trust().provenance(),
        SupportTrustProvenance::Replicated
    );
}

#[test]
fn transformed_exact_trust_rejects_unbound_equivalence() {
    let error = ExactSupportTrustWitness::from_equivalent_operational_basis(
        exact_translation(),
        SupportTrustProvenance::Replicated,
        SupportTrustFreshnessWitness::new(epochs()),
        SupportTrustEquivalenceWitness::new(
            SubscriptionSupportOperationalBasis::new(
                SubscriptionSupportFamilyId::new("other-support-family").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                SubscriptionSupportRole::ExactContinuation,
                SubscriptionSupportArtifactId("artifact:trust:other".into()),
                "basis:other",
                "cursor:trust",
                "checkpoint:trust",
                "compatibility:trust",
                "portability:trust",
                SubscriptionSupportActionOrigin::Retention,
            )
            .unwrap(),
            SubscriptionSupportFamilyId::new("replicated-continuation-support").unwrap(),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "equivalence:digest",
        )
        .unwrap(),
    )
    .expect_err("equivalence proof must be family-bound");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustBasisMismatch
    );
}

#[test]
fn certified_report_requires_certification_epoch_and_stamp() {
    let translation = exact_translation();
    let family_id = translation.basis().family_id().clone();
    let support_role = translation.basis().support_role();
    let witness = ExactSupportTrustWitness::from_exact_translation(
        translation,
        SupportTrustProvenance::NativePublished,
        SupportTrustFreshnessWitness::new(epochs()),
    )
    .unwrap();
    let operational = OperationalSupportTrustReport::from_exact_witness(witness);
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-1",
        family_id,
        support_role,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        "row:exact-control",
        "bundle:digest",
    )
    .unwrap();

    let certified = CertifiedSupportTrustReport::from_operational_report(operational, stamp)
        .expect("matching certification epoch should certify operational trust");

    assert_eq!(
        certified.use_boundary(),
        SupportTrustUseBoundary::CertifiedPlatform
    );
    assert_eq!(
        certified.certification_stamp().row_id(),
        "row:exact-control"
    );
}

#[test]
fn certification_stamp_must_match_operational_report_scope() {
    let witness = ExactSupportTrustWitness::from_exact_translation(
        exact_translation(),
        SupportTrustProvenance::NativePublished,
        SupportTrustFreshnessWitness::new(epochs()),
    )
    .unwrap();
    let operational = OperationalSupportTrustReport::from_exact_witness(witness);
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3-phase-1",
        SubscriptionSupportFamilyId::new("other-support-family").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        "row:exact-control",
        "bundle:digest",
    )
    .unwrap();

    let error = CertifiedSupportTrustReport::from_operational_report(operational, stamp)
        .expect_err("certification coverage must be family-scoped");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustFamilyMismatch
    );
}
