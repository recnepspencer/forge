use super::super::SubscriptionSupportOperationalVerdict;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRetentionDecision {
    evidence: SubscriptionSupportRetentionDecisionEvidence,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) enum SubscriptionSupportRetentionDecisionEvidence {
    RetainExact,
    RetainDegraded {
        weakened_condition: String,
    },
    CompactExact {
        compacted_basis_digest: String,
    },
    ReclaimWithRebuild {
        retained_rebuild_basis_digest: String,
        maintenance_admission_key: String,
    },
    ReclaimWithoutRebuild {
        missing_rebuild_basis_reason: String,
    },
    ExpireByPolicy {
        policy_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportRetentionDecision {
    pub(crate) fn retain_exact() -> Self {
        Self {
            evidence: SubscriptionSupportRetentionDecisionEvidence::RetainExact,
        }
    }

    pub(crate) fn retain_degraded(
        weakened_condition: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded {
                weakened_condition: require_non_empty(
                    "weakened support condition",
                    weakened_condition,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn compact_exact(
        compacted_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(SubscriptionSupportRetentionDecisionEvidence::CompactExact {
            compacted_basis_digest: require_non_empty(
                "compacted support basis",
                compacted_basis_digest,
            )?,
        }
        .into())
    }

    pub(crate) fn reclaim_with_rebuild(
        retained_rebuild_basis_digest: impl Into<String>,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest: require_non_empty(
                    "retained rebuild basis",
                    retained_rebuild_basis_digest,
                )?,
                maintenance_admission_key: require_non_empty(
                    "maintenance admission",
                    maintenance_admission_key,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn reclaim_without_rebuild(
        missing_rebuild_basis_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason: require_non_empty(
                    "missing rebuild basis reason",
                    missing_rebuild_basis_reason,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn expire_by_policy(policy_reason: impl Into<String>) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy {
                policy_reason: require_non_empty("retention policy reason", policy_reason)?,
            }
            .into(),
        )
    }

    pub(super) fn evidence(&self) -> &SubscriptionSupportRetentionDecisionEvidence {
        &self.evidence
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact
            | SubscriptionSupportRetentionDecisionEvidence::CompactExact { .. } => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. } => {
                SubscriptionSupportOperationalVerdict::RebuildRequired
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. } => {
                SubscriptionSupportOperationalVerdict::NotResumable
            }
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { .. } => {
                SubscriptionSupportOperationalVerdict::RejectedByPolicy
            }
        }
    }

    pub fn is_reclaim(&self) -> bool {
        matches!(
            self.evidence,
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. }
                | SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. }
        )
    }

    pub fn kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainExact => {
                SubscriptionSupportRetentionDecisionKind::RetainExact
            }
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { .. } => {
                SubscriptionSupportRetentionDecisionKind::RetainDegraded
            }
            SubscriptionSupportRetentionDecisionEvidence::CompactExact { .. } => {
                SubscriptionSupportRetentionDecisionKind::CompactExact
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild { .. } => {
                SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            }
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild { .. } => {
                SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild
            }
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { .. } => {
                SubscriptionSupportRetentionDecisionKind::ExpireByPolicy
            }
        }
    }

    pub fn weakened_condition(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::RetainDegraded { weakened_condition } => {
                Some(weakened_condition)
            }
            _ => None,
        }
    }

    pub fn compacted_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::CompactExact {
                compacted_basis_digest,
            } => Some(compacted_basis_digest),
            _ => None,
        }
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                retained_rebuild_basis_digest,
                ..
            } => Some(retained_rebuild_basis_digest),
            _ => None,
        }
    }

    pub fn maintenance_admission_key(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithRebuild {
                maintenance_admission_key,
                ..
            } => Some(maintenance_admission_key),
            _ => None,
        }
    }

    pub fn missing_rebuild_basis_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ReclaimWithoutRebuild {
                missing_rebuild_basis_reason,
            } => Some(missing_rebuild_basis_reason),
            _ => None,
        }
    }

    pub fn policy_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportRetentionDecisionEvidence::ExpireByPolicy { policy_reason } => {
                Some(policy_reason)
            }
            _ => None,
        }
    }
}

impl From<SubscriptionSupportRetentionDecisionEvidence> for SubscriptionSupportRetentionDecision {
    fn from(evidence: SubscriptionSupportRetentionDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SubscriptionSupportRetentionDecisionKind {
    RetainExact,
    RetainDegraded,
    CompactExact,
    ReclaimWithRebuild,
    ReclaimWithoutRebuild,
    ExpireByPolicy,
}
