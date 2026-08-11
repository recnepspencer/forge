use super::super::{
    classification_error, SubscriptionSupportOperationalVerdict, SupportActionId,
    SupportAffectedSetDigest,
};
use super::affected_set::SupportMaintenanceAffectedSet;
use super::decision::{SubscriptionSupportMaintenanceDecision, SupportMaintenanceWorkKind};
use super::evidence_validation::require_non_empty as require_maintenance_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceDebtSummary {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    work_kind: SupportMaintenanceWorkKind,
    verdict: SubscriptionSupportOperationalVerdict,
    delay_reason: String,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl SupportMaintenanceDebtSummary {
    pub(super) fn new(
        action_id: &SupportActionId,
        affected_set: &SupportMaintenanceAffectedSet,
        decision: &SubscriptionSupportMaintenanceDecision,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
        delay_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if descriptor_count == 0 {
            return Err(classification_error(
                "subscription-support maintenance debt summaries require admitted descriptors",
            ));
        }
        Ok(Self {
            action_id: action_id.clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            work_kind: decision.work_kind(),
            verdict: decision.verdict(),
            delay_reason: require_maintenance_non_empty("delay reason", delay_reason)?,
            descriptor_count,
            coalesced_duplicate_count,
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn work_kind(&self) -> SupportMaintenanceWorkKind {
        self.work_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn delay_reason(&self) -> &str {
        &self.delay_reason
    }

    pub fn descriptor_count(&self) -> u64 {
        self.descriptor_count
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }
}
