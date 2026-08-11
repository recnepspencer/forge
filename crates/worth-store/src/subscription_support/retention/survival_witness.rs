use super::super::{
    classification_error, CompletedSupportProgramAction, SubscriptionSupportActionOrigin,
    SubscriptionSupportOperationalVerdict,
};
use super::affected_set::{SupportAffectedSet, SupportAffectedSetDigest};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionSurvivalWitness {
    verdict: SubscriptionSupportOperationalVerdict,
    affected_count: u64,
    affected_set_digest: SupportAffectedSetDigest,
}

impl SupportRetentionSurvivalWitness {
    pub(crate) fn new(
        completed_action: &CompletedSupportProgramAction,
        expected_verdict: SubscriptionSupportOperationalVerdict,
        affected_set: &SupportAffectedSet,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != SubscriptionSupportActionOrigin::Retention
        {
            return Err(classification_error(
                "subscription-support retention survival witnesses require retention-origin envelopes",
            ));
        }
        if completed_action.envelope().verdict() != expected_verdict {
            return Err(classification_error(
                "subscription-support retention survival witness verdict drift",
            ));
        }
        Ok(Self {
            verdict: expected_verdict,
            affected_count: affected_set.affected_count(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
        })
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_count
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }
}
