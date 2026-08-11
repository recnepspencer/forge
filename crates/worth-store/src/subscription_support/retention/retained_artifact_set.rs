use super::affected_set::SupportAffectedSet;
use super::decision::SubscriptionSupportRetentionDecisionKind;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedSupportArtifactSet {
    affected_set: SupportAffectedSet,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    weakened_condition: Option<String>,
}

impl RetainedSupportArtifactSet {
    pub(crate) fn exact(affected_set: SupportAffectedSet) -> Self {
        Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::RetainExact,
            weakened_condition: None,
        }
    }

    pub(crate) fn degraded(
        affected_set: SupportAffectedSet,
        weakened_condition: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            decision_kind: SubscriptionSupportRetentionDecisionKind::RetainDegraded,
            weakened_condition: Some(require_non_empty(
                "weakened support condition",
                weakened_condition,
            )?),
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn weakened_condition(&self) -> Option<&str> {
        self.weakened_condition.as_deref()
    }
}
