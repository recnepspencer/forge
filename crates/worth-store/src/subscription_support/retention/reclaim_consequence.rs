use super::super::{classification_error, CompletedSupportProgramAction};
use super::materialization::SubscriptionSupportRetentionMaterialization;
use super::participation_record::SupportRetentionParticipationRecord;
use super::reclaimed_artifact_set::ReclaimedSupportArtifactSet;
use super::survival_witness::SupportRetentionSurvivalWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportReclaimConsequence {
    completed_action: CompletedSupportProgramAction,
    survival_witness: SupportRetentionSurvivalWitness,
    retention_record: SupportRetentionParticipationRecord,
    reclaimed_artifacts: ReclaimedSupportArtifactSet,
}

impl SupportReclaimConsequence {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        survival_witness: SupportRetentionSurvivalWitness,
        retention_record: SupportRetentionParticipationRecord,
        materialization: SubscriptionSupportRetentionMaterialization,
    ) -> Result<Self, StoreError> {
        let SubscriptionSupportRetentionMaterialization::Reclaimed(reclaimed_artifacts) =
            materialization
        else {
            return Err(classification_error(
                "subscription-support reclaim consequences require reclaimed support materialization",
            ));
        };
        Ok(Self {
            completed_action,
            survival_witness,
            retention_record,
            reclaimed_artifacts,
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn survival_witness(&self) -> &SupportRetentionSurvivalWitness {
        &self.survival_witness
    }

    pub fn retention_record(&self) -> &SupportRetentionParticipationRecord {
        &self.retention_record
    }

    pub fn reclaimed_artifacts(&self) -> &ReclaimedSupportArtifactSet {
        &self.reclaimed_artifacts
    }
}
