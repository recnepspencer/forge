use super::super::classification_error;
use super::super::{
    CompletedSupportProgramAction, SubscriptionSupportActionOrigin, SupportActionId,
    SupportAffectedSetDigest,
};
use super::affected_set::SupportCompatibilityAffectedSet;
use super::decision::SubscriptionSupportCompatibilityDecisionKind;
use super::decoded_row_access::SupportDecodedRowSemanticAccess;
use super::manifest_admission::SupportManifestAdmissionWitness;
use crate::failure::StoreError;
use crate::CompatibilityRejectionKind;
use crate::CompatibilityRelation;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityParticipationRecord {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: Option<CompatibilityRelation>,
    milestone12_rejection_kind: Option<CompatibilityRejectionKind>,
    decision_kind: SubscriptionSupportCompatibilityDecisionKind,
    semantic_digest: String,
}

impl SupportCompatibilityParticipationRecord {
    pub(super) fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportCompatibilityAffectedSet,
        manifest_admission: &SupportManifestAdmissionWitness,
        semantic_access: &SupportDecodedRowSemanticAccess,
        decision_kind: SubscriptionSupportCompatibilityDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin()
            != SubscriptionSupportActionOrigin::Compatibility
        {
            return Err(classification_error(
                "subscription-support compatibility participation records require compatibility-origin actions",
            ));
        }
        if manifest_admission != semantic_access.admission_witness() {
            return Err(classification_error(
                "subscription-support compatibility records require admitted semantic access",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            manifest_digest: manifest_admission.manifest_digest().to_string(),
            compatibility_digest: manifest_admission.compatibility_digest().to_string(),
            milestone12_receipt_digest: manifest_admission
                .compatibility_receipt()
                .receipt_digest()
                .to_string(),
            milestone12_relation: manifest_admission.compatibility_receipt().relation(),
            milestone12_rejection_kind: manifest_admission.compatibility_receipt().rejection_kind(),
            decision_kind,
            semantic_digest: semantic_access.semantic_digest().to_string(),
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        self.decision_kind
    }

    pub fn milestone12_receipt_digest(&self) -> &str {
        &self.milestone12_receipt_digest
    }

    pub fn milestone12_relation(&self) -> Option<CompatibilityRelation> {
        self.milestone12_relation
    }

    pub fn milestone12_rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.milestone12_rejection_kind
    }
}
