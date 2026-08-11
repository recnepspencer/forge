use super::super::epochs::SupportTrustEpoch;
use super::super::performance::{SupportTrustEvidenceBudget, SupportTrustPerformancePlan};
use super::super::taxonomy::{SupportTrustProvenance, SupportTrustStrength};
use crate::subscription_support::{
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustRequestedUse {
    StoreLocalResume,
    CertifiedPlatformClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustBatchCardinality {
    SingleSupportArtifact,
    FamilyRoleBatch { artifact_count: u64 },
}

impl SupportTrustBatchCardinality {
    pub fn artifact_count(self) -> u64 {
        match self {
            Self::SingleSupportArtifact => 1,
            Self::FamilyRoleBatch { artifact_count } => artifact_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawSupportTrustRequest {
    family_id: SubscriptionSupportFamilyId,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    requested_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    requested_use: SupportTrustRequestedUse,
    batch_cardinality: SupportTrustBatchCardinality,
    epoch: SupportTrustEpoch,
    performance_plan: SupportTrustPerformancePlan,
    evidence_budget: SupportTrustEvidenceBudget,
}

impl RawSupportTrustRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        requested_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        requested_use: SupportTrustRequestedUse,
        batch_cardinality: SupportTrustBatchCardinality,
        epoch: SupportTrustEpoch,
        performance_plan: SupportTrustPerformancePlan,
        evidence_budget: SupportTrustEvidenceBudget,
    ) -> Self {
        Self {
            family_id,
            support_role,
            artifact_id,
            requested_strength,
            provenance,
            requested_use,
            batch_cardinality,
            epoch,
            performance_plan,
            evidence_budget,
        }
    }

    pub(super) fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub(super) fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub(super) fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub(super) fn requested_strength(&self) -> SupportTrustStrength {
        self.requested_strength
    }

    pub(super) fn provenance(&self) -> SupportTrustProvenance {
        self.provenance
    }

    pub(super) fn requested_use(&self) -> SupportTrustRequestedUse {
        self.requested_use
    }

    pub(super) fn batch_cardinality(&self) -> SupportTrustBatchCardinality {
        self.batch_cardinality
    }

    pub(super) fn epoch(&self) -> SupportTrustEpoch {
        self.epoch
    }

    pub(super) fn performance_plan(&self) -> &SupportTrustPerformancePlan {
        &self.performance_plan
    }

    pub(super) fn evidence_budget(&self) -> &SupportTrustEvidenceBudget {
        &self.evidence_budget
    }
}
