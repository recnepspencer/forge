use super::super::{
    classification_error, cost_surface_for_program_path, CompletedSupportProgramAction,
    SubscriptionSupportOperationalBasis, SubscriptionSupportPlanFamily,
    SubscriptionSupportResultCostSurface, SupportProgramPathPlan,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::capsule_manifest::CapsuleSupportManifest;
use super::decision::SubscriptionSupportPortabilityDecision;
use super::outcome::SubscriptionSupportPortabilityOutcome;
use super::outcome_materialization::outcome_from_decision;
use super::participation_record::SupportPortabilityParticipationRecord;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPortabilityReport {
    completed_action: CompletedSupportProgramAction,
    translation_basis: SubscriptionSupportOperationalBasis,
    participation_record: SupportPortabilityParticipationRecord,
    manifest: CapsuleSupportManifest,
    outcome: SubscriptionSupportPortabilityOutcome,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportPortabilityReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportPortabilityAffectedSet,
        manifest: CapsuleSupportManifest,
        decision: &SubscriptionSupportPortabilityDecision,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let decision_kind = decision.kind();
        let participation_record = SupportPortabilityParticipationRecord::new(
            &completed_action,
            &affected_set,
            &manifest,
            decision_kind,
        )?;
        let outcome = outcome_from_decision(&affected_set, &manifest, decision)?;
        if outcome.outcome_kind() != decision_kind {
            return Err(classification_error(
                "subscription-support portability outcome kind must match decision kind",
            ));
        }
        Ok(Self {
            completed_action,
            translation_basis: affected_set.primary_basis().clone(),
            participation_record,
            manifest,
            outcome,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::PortabilityParticipationPlan,
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

    pub fn participation_record(&self) -> &SupportPortabilityParticipationRecord {
        &self.participation_record
    }

    pub fn manifest(&self) -> &CapsuleSupportManifest {
        &self.manifest
    }

    pub fn outcome(&self) -> &SubscriptionSupportPortabilityOutcome {
        &self.outcome
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}
