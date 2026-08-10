use super::super::classification_error;
use super::super::{
    cost_surface_for_program_path, CompletedSupportProgramAction,
    SubscriptionSupportOperationalBasis, SubscriptionSupportPlanFamily,
    SubscriptionSupportResultCostSurface, SupportProgramPathPlan,
};
use super::affected_set::SupportCompatibilityAffectedSet;
use super::decision::SubscriptionSupportCompatibilityDecision;
use super::decoded_row_access::SupportDecodedRowSemanticAccess;
use super::manifest_admission::SupportManifestAdmissionWitness;
use super::outcome::{outcome_from_decision, SubscriptionSupportCompatibilityOutcome};
use super::participation_record::SupportCompatibilityParticipationRecord;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCompatibilityReport {
    completed_action: CompletedSupportProgramAction,
    translation_basis: SubscriptionSupportOperationalBasis,
    participation_record: SupportCompatibilityParticipationRecord,
    outcome: SubscriptionSupportCompatibilityOutcome,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportCompatibilityReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportCompatibilityAffectedSet,
        path_plan: &SupportProgramPathPlan,
        manifest_admission: SupportManifestAdmissionWitness,
        semantic_access: SupportDecodedRowSemanticAccess,
        decision: &SubscriptionSupportCompatibilityDecision,
    ) -> Result<Self, StoreError> {
        let decision_kind = decision.kind();
        let translation_basis = affected_set.primary_basis().clone();
        let participation_record = SupportCompatibilityParticipationRecord::new(
            &completed_action,
            &affected_set,
            &manifest_admission,
            &semantic_access,
            decision_kind,
        )?;
        let outcome = outcome_from_decision(affected_set, manifest_admission, decision)?;
        if outcome.outcome_kind() != decision_kind {
            return Err(classification_error(
                "subscription-support compatibility outcome kind must match decision kind",
            ));
        }
        Ok(Self {
            completed_action,
            translation_basis,
            participation_record,
            outcome,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::CompatibilityParticipationPlan,
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

    pub fn participation_record(&self) -> &SupportCompatibilityParticipationRecord {
        &self.participation_record
    }

    pub fn outcome(&self) -> &SubscriptionSupportCompatibilityOutcome {
        &self.outcome
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}
