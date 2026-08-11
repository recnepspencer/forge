use super::super::{
    classification_error, CompletedSupportProgramAction, SubscriptionSupportActionOrigin,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole, SupportActionId,
    SupportAffectedSetDigest,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::capsule_manifest::CapsuleSupportManifest;
use super::decision::SubscriptionSupportPortabilityDecisionKind;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityParticipationRecord {
    action_id: SupportActionId,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    decision_kind: SubscriptionSupportPortabilityDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
    manifest_digest: String,
    manifest_entry_count: u64,
    omitted_support_count: u64,
    required_basis_count: u64,
}

impl SupportPortabilityParticipationRecord {
    pub(super) fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportPortabilityAffectedSet,
        manifest: &CapsuleSupportManifest,
        decision_kind: SubscriptionSupportPortabilityDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != affected_set.action_origin() {
            return Err(classification_error(
                "subscription-support portability participation record action origin drift",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            decision_kind,
            verdict: completed_action.envelope().verdict(),
            action_origin: completed_action.envelope().action_origin(),
            manifest_digest: manifest.manifest_digest().to_string(),
            manifest_entry_count: manifest.manifest_entry_count(),
            omitted_support_count: manifest.omitted_support_count(),
            required_basis_count: manifest.required_basis_count(),
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn manifest_entry_count(&self) -> u64 {
        self.manifest_entry_count
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.omitted_support_count
    }

    pub fn required_basis_count(&self) -> u64 {
        self.required_basis_count
    }
}
