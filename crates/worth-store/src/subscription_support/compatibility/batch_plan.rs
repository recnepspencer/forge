use super::super::{SupportActionId, SupportProgramPathPlan};
use super::affected_set::SupportCompatibilityAffectedSet;
use super::decision::SubscriptionSupportCompatibilityDecision;
use super::decoded_row_access::SupportDecodedRowSemanticAccess;
use super::evidence_validation::validate_decision_against_receipt;
use super::manifest_admission::SupportManifestAdmissionWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCompatibilityBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportCompatibilityAffectedSet,
    path_plan: SupportProgramPathPlan,
    manifest_admission: SupportManifestAdmissionWitness,
    semantic_access: SupportDecodedRowSemanticAccess,
    decision: SubscriptionSupportCompatibilityDecision,
}

impl SupportCompatibilityBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportCompatibilityAffectedSet,
        path_plan: SupportProgramPathPlan,
        manifest_admission: SupportManifestAdmissionWitness,
        semantic_access: SupportDecodedRowSemanticAccess,
        decision: SubscriptionSupportCompatibilityDecision,
    ) -> Result<Self, StoreError> {
        if affected_set.family_id() != manifest_admission.version_window().family_id()
            || affected_set.family_kind() != manifest_admission.version_window().family_kind()
        {
            return Err(super::super::classification_error(
                "subscription-support compatibility batch manifest admission must match affected family",
            ));
        }
        if manifest_admission != *semantic_access.admission_witness() {
            return Err(super::super::classification_error(
                "decoded subscription-support semantic access requires the same manifest admission witness",
            ));
        }
        validate_decision_against_receipt(&decision, manifest_admission.compatibility_receipt())?;
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            manifest_admission,
            semantic_access,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportCompatibilityAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn manifest_admission(&self) -> &SupportManifestAdmissionWitness {
        &self.manifest_admission
    }

    pub fn semantic_access(&self) -> &SupportDecodedRowSemanticAccess {
        &self.semantic_access
    }

    pub fn decision(&self) -> &SubscriptionSupportCompatibilityDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportCompatibilityAffectedSet,
        SupportProgramPathPlan,
        SupportManifestAdmissionWitness,
        SupportDecodedRowSemanticAccess,
        SubscriptionSupportCompatibilityDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.manifest_admission,
            self.semantic_access,
            self.decision,
        )
    }
}
