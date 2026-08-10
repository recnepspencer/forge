use super::super::{
    cost_surface_for_program_path, CompletedSupportProgramAction,
    SubscriptionSupportOperationalBasis, SubscriptionSupportPlanFamily,
    SubscriptionSupportResultCostSurface, SupportProgramPathPlan,
};
use super::decision::SubscriptionSupportRetentionDecisionKind;
use super::materialization::SubscriptionSupportRetentionMaterialization;
use super::participation_record::SupportRetentionParticipationRecord;
use super::survival_witness::SupportRetentionSurvivalWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPostActionReport {
    completed_action: CompletedSupportProgramAction,
    translation_basis: SubscriptionSupportOperationalBasis,
    survival_witness: SupportRetentionSurvivalWitness,
    retention_record: SupportRetentionParticipationRecord,
    materialization: SubscriptionSupportRetentionMaterialization,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportPostActionReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        translation_basis: SubscriptionSupportOperationalBasis,
        survival_witness: SupportRetentionSurvivalWitness,
        materialization: SubscriptionSupportRetentionMaterialization,
        decision_kind: SubscriptionSupportRetentionDecisionKind,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let retention_record = SupportRetentionParticipationRecord::new(
            &completed_action,
            &survival_witness,
            &materialization,
            decision_kind,
        )?;
        Ok(Self {
            completed_action,
            translation_basis,
            survival_witness,
            retention_record,
            materialization,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::RetentionParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn translation_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.translation_basis
    }

    pub fn survival_witness(&self) -> &SupportRetentionSurvivalWitness {
        &self.survival_witness
    }

    pub fn retention_record(&self) -> &SupportRetentionParticipationRecord {
        &self.retention_record
    }

    pub fn materialization(&self) -> &SubscriptionSupportRetentionMaterialization {
        &self.materialization
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}
