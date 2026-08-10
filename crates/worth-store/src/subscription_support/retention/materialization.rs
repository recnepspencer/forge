use super::affected_set::SupportAffectedSet;
use super::compacted_basis::CompactedSupportBasis;
use super::decision::{
    SubscriptionSupportRetentionDecision, SubscriptionSupportRetentionDecisionEvidence,
    SubscriptionSupportRetentionDecisionKind,
};
use super::expired_artifact_set::ExpiredSupportArtifactSet;
use super::reclaimed_artifact_set::ReclaimedSupportArtifactSet;
use super::retained_artifact_set::RetainedSupportArtifactSet;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportRetentionMaterialization {
    Retained(RetainedSupportArtifactSet),
    Compacted(CompactedSupportBasis),
    Reclaimed(ReclaimedSupportArtifactSet),
    Expired(ExpiredSupportArtifactSet),
}

impl SubscriptionSupportRetentionMaterialization {
    pub(crate) fn from_decision(
        affected_set: SupportAffectedSet,
        decision: &SubscriptionSupportRetentionDecision,
    ) -> Result<Self, StoreError> {
        match decision.evidence() {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact => Ok(Self::Retained(
                RetainedSupportArtifactSet::exact(affected_set),
            )),
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { weakened_condition } => {
                Ok(Self::Retained(RetainedSupportArtifactSet::degraded(
                    affected_set,
                    weakened_condition.clone(),
                )?))
            }
            SubscriptionSupportRetentionDecisionEvidence::CompactExact {
                compacted_basis_digest,
            } => Ok(Self::Compacted(CompactedSupportBasis::new(
                affected_set,
                compacted_basis_digest.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest,
                maintenance_admission_key,
            } => Ok(Self::Reclaimed(ReclaimedSupportArtifactSet::rebuildable(
                affected_set,
                retained_rebuild_basis_digest.clone(),
                maintenance_admission_key.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason,
            } => Ok(Self::Reclaimed(ReclaimedSupportArtifactSet::non_resumable(
                affected_set,
                missing_rebuild_basis_reason.clone(),
            )?)),
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { policy_reason } => {
                Ok(Self::Expired(ExpiredSupportArtifactSet::new(
                    affected_set,
                    policy_reason.clone(),
                )?))
            }
        }
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        match self {
            Self::Retained(set) => set.affected_set(),
            Self::Compacted(basis) => basis.affected_set(),
            Self::Reclaimed(set) => set.affected_set(),
            Self::Expired(set) => set.affected_set(),
        }
    }

    pub fn materialization_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        match self {
            Self::Retained(set) => set.decision_kind(),
            Self::Compacted(_) => SubscriptionSupportRetentionDecisionKind::CompactExact,
            Self::Reclaimed(set) => set.decision_kind(),
            Self::Expired(_) => SubscriptionSupportRetentionDecisionKind::ExpireByPolicy,
        }
    }

    pub fn maintenance_admission_key(&self) -> Option<&str> {
        match self {
            Self::Reclaimed(set) => set.maintenance_admission_key(),
            _ => None,
        }
    }
}
