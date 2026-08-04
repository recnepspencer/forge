use super::super::{
    check_support_trust_coverage, classify_certified_support_trust,
    CertifiedSupportTrustClassified, SupportCertificationBatchScope,
    SupportCertificationBatchScopeKind, SupportCertificationCorpusVersion,
    SupportCertificationCounterSnapshot, SupportCertificationEpoch,
    SupportCertificationEvidenceBundle, SupportGenericCertificationCounterSnapshot,
    SupportGenericCertificationReport, SupportTrustAllocationScope, SupportTrustCertificationStamp,
    SupportTrustDensityClass, SupportTrustPathClass, SupportTrustProvenance, SupportTrustStrength,
};
use super::certification_coverage::first_ship_certification_matrix;
use super::operational_basis::basis;
use super::operational_classification::classify_phase2_for_basis;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};

pub(super) fn first_ship_batch_scope() -> SupportCertificationBatchScope {
    SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::CertificationScopeLocal,
        SupportTrustDensityClass::CertificationScopeLocal,
        SupportTrustPathClass::BatchCertificationPath,
        SupportTrustAllocationScope::BatchCertification,
        4,
        4,
        3,
        1,
    )
    .unwrap()
}

pub(super) fn first_ship_counter_snapshot() -> SupportCertificationCounterSnapshot {
    SupportCertificationCounterSnapshot::new(4, 4, 3, 4, 1, 0, 0)
}

pub(super) fn first_ship_certification_bundle() -> SupportCertificationEvidenceBundle {
    SupportCertificationEvidenceBundle::new(
        "run:13.3:first-ship",
        first_ship_certification_matrix(),
        first_ship_batch_scope(),
        first_ship_counter_snapshot(),
    )
    .unwrap()
}

pub(super) fn certified_first_ship_support_trust() -> CertifiedSupportTrustClassified {
    certified_first_ship_support_trust_for(
        basis(),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        "row:basis-bound-exact",
    )
}

pub(super) fn certified_first_ship_support_trust_for(
    basis: SubscriptionSupportOperationalBasis,
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
    row_id: &str,
) -> CertifiedSupportTrustClassified {
    let family_id = basis.family_id().clone();
    let support_role = basis.support_role();
    let classified = classify_phase2_for_basis(
        basis,
        requested_strength,
        provenance,
        classification,
        verdict,
    )
    .unwrap();
    let bundle = first_ship_certification_bundle();
    let evidence_bundle_digest = bundle.evidence_bundle_digest().to_string();
    let coverage_checked = check_support_trust_coverage(classified, bundle).unwrap();
    let stamp = SupportTrustCertificationStamp::new(
        SupportCertificationCorpusVersion::new("corpus:13.3").unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        "suite:13.3",
        family_id,
        support_role,
        requested_strength,
        provenance,
        row_id,
        evidence_bundle_digest,
    )
    .unwrap();
    classify_certified_support_trust(coverage_checked, stamp).unwrap()
}

pub(super) fn generic_support_certification_report() -> SupportGenericCertificationReport {
    let certified = certified_first_ship_support_trust();
    generic_support_certification_report_for(
        "generic:subscription-support-trust:first-ship",
        certified,
    )
}

pub(super) fn generic_support_certification_report_for(
    generic_row_id: &str,
    certified: CertifiedSupportTrustClassified,
) -> SupportGenericCertificationReport {
    SupportGenericCertificationReport::from_certified_support_trust(
        generic_row_id,
        certified.report().clone(),
        certified.coverage_witness(),
        SupportGenericCertificationCounterSnapshot::new(1, 1, 1, 1, 1, 1).unwrap(),
    )
    .unwrap()
}
