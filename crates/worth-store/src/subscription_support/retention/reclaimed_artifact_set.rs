use super::affected_set::SupportAffectedSet;
use super::decision::SubscriptionSupportRetentionDecisionKind;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReclaimedSupportArtifactSet {
    affected_set: SupportAffectedSet,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    retained_rebuild_basis_digest: Option<String>,
    maintenance_admission_key: Option<String>,
    missing_rebuild_basis_reason: Option<String>,
}

impl ReclaimedSupportArtifactSet {
    pub(crate) fn rebuildable(
        affected_set: SupportAffectedSet,
        retained_rebuild_basis_digest: impl Into<String>,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild,
            retained_rebuild_basis_digest: Some(require_non_empty(
                "retained rebuild basis",
                retained_rebuild_basis_digest,
            )?),
            maintenance_admission_key: Some(require_non_empty(
                "maintenance admission",
                maintenance_admission_key,
            )?),
            missing_rebuild_basis_reason: None,
        })
    }

    pub(crate) fn non_resumable(
        affected_set: SupportAffectedSet,
        missing_rebuild_basis_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild,
            retained_rebuild_basis_digest: None,
            maintenance_admission_key: None,
            missing_rebuild_basis_reason: Some(require_non_empty(
                "missing rebuild basis reason",
                missing_rebuild_basis_reason,
            )?),
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub fn maintenance_admission_key(&self) -> Option<&str> {
        self.maintenance_admission_key.as_deref()
    }

    pub fn missing_rebuild_basis_reason(&self) -> Option<&str> {
        self.missing_rebuild_basis_reason.as_deref()
    }
}
