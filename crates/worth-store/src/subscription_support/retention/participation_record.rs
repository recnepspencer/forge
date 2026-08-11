use super::super::{
    classification_error, CompletedSupportProgramAction, SubscriptionSupportActionOrigin,
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole, SupportActionId,
};
use super::affected_set::SupportAffectedSetDigest;
use super::decision::SubscriptionSupportRetentionDecisionKind;
use super::materialization::SubscriptionSupportRetentionMaterialization;
use super::survival_witness::SupportRetentionSurvivalWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRetentionParticipationRecord {
    action_id: SupportActionId,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    affected_count: u64,
    decision_kind: SubscriptionSupportRetentionDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
}

impl SupportRetentionParticipationRecord {
    pub(crate) fn new(
        completed_action: &CompletedSupportProgramAction,
        survival_witness: &SupportRetentionSurvivalWitness,
        materialization: &SubscriptionSupportRetentionMaterialization,
        decision_kind: SubscriptionSupportRetentionDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != SubscriptionSupportActionOrigin::Retention
        {
            return Err(classification_error(
                "subscription-support retention participation records require retention-origin envelopes",
            ));
        }
        if materialization.affected_set().affected_count() != survival_witness.affected_count() {
            return Err(classification_error(
                "subscription-support retention participation record breadth drift",
            ));
        }
        if materialization.affected_set().affected_set_digest()
            != survival_witness.affected_set_digest()
        {
            return Err(classification_error(
                "subscription-support retention participation record affected-set digest drift",
            ));
        }
        if decision_kind != materialization.materialization_kind() {
            return Err(classification_error(
                "subscription-support retention participation record decision kind drift",
            ));
        }
        let affected_set = materialization.affected_set();
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            affected_artifact_ids: affected_set.affected_artifact_ids(),
            affected_count: affected_set.affected_count(),
            decision_kind,
            verdict: survival_witness.verdict(),
            action_origin: completed_action.envelope().action_origin(),
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_count
    }

    pub fn decision_kind(&self) -> SubscriptionSupportRetentionDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }
}
