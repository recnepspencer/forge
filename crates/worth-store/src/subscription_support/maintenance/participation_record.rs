use super::super::{
    classification_error, CompletedSupportProgramAction, SubscriptionSupportActionOrigin,
    SubscriptionSupportOperationalVerdict, SupportActionId, SupportAffectedSetDigest,
};
use super::affected_set::SupportMaintenanceAffectedSet;
use super::decision::SubscriptionSupportMaintenanceDecisionKind;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceParticipationRecord {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    decision_kind: SubscriptionSupportMaintenanceDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl SupportMaintenanceParticipationRecord {
    pub(super) fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportMaintenanceAffectedSet,
        decision_kind: SubscriptionSupportMaintenanceDecisionKind,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin()
            != SubscriptionSupportActionOrigin::Maintenance
        {
            return Err(classification_error(
                "subscription-support maintenance participation record action origin drift",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            decision_kind,
            verdict: completed_action.envelope().verdict(),
            descriptor_count,
            coalesced_duplicate_count,
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportMaintenanceDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn descriptor_count(&self) -> u64 {
        self.descriptor_count
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }
}
