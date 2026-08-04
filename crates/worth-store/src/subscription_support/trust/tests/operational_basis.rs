use super::super::{
    RawSupportTrustRequest, SupportCatalogEpoch, SupportCertificationEpoch,
    SupportCompatibilityEpoch, SupportOperationalLedgerEpoch, SupportTrustAccessPath,
    SupportTrustAllocationScope, SupportTrustBatchCardinality, SupportTrustCloneBoundary,
    SupportTrustDensityClass, SupportTrustEpoch, SupportTrustEvidenceBudget, SupportTrustPathClass,
    SupportTrustPerformancePlan, SupportTrustProvenance, SupportTrustRequestedUse,
    SupportTrustStrength,
};
use crate::subscription_support::{
    SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalBasis, SubscriptionSupportRole,
};

pub(super) fn basis() -> SubscriptionSupportOperationalBasis {
    basis_for(
        "basis-bound-continuation-support",
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        "artifact:trust:phase-1",
    )
}

pub(super) fn basis_for(
    family_id: &str,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: &str,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        family_kind,
        support_role,
        SubscriptionSupportArtifactId(artifact_id.into()),
        "basis:trust",
        "cursor:trust",
        "checkpoint:trust",
        "compatibility:trust",
        "portability:trust",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

pub(super) fn epochs() -> SupportTrustEpoch {
    SupportTrustEpoch::new(
        SupportCatalogEpoch::new(1).unwrap(),
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCompatibilityEpoch::new(3).unwrap(),
        Some(SupportCertificationEpoch::new(11).unwrap()),
    )
}

pub(super) fn phase2_performance_plan() -> SupportTrustPerformancePlan {
    SupportTrustPerformancePlan::new(
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
    .unwrap()
}

pub(super) fn raw_phase2_request(
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
) -> RawSupportTrustRequest {
    raw_phase2_request_for(
        "basis-bound-continuation-support",
        SubscriptionSupportRole::ExactContinuation,
        "artifact:trust:phase-1",
        requested_strength,
        provenance,
    )
}

pub(super) fn raw_phase2_request_for(
    family_id: &str,
    support_role: SubscriptionSupportRole,
    artifact_id: &str,
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
) -> RawSupportTrustRequest {
    RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        support_role,
        SubscriptionSupportArtifactId(artifact_id.into()),
        requested_strength,
        provenance,
        SupportTrustRequestedUse::StoreLocalResume,
        SupportTrustBatchCardinality::SingleSupportArtifact,
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(4096, 8, 1).unwrap(),
    )
}
