use crate::application::{
    WorthQueryDeclarationBridgeAuthorityAspectSummary,
    WorthQueryDeclarationRelationalAuthorityAspectSummary,
    WorthQueryDeclarationSignalAuthorityAspectSummary,
};
use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
#[cfg(test)]
use crate::identity::hash_parts;

use super::super::materialization::{
    WorthQueryDeclarationEntryOrchestrationCostPosture,
    WorthQueryDeclarationEntryOrchestrationMaterializationPolicy,
};
use super::super::sequencing::WorthQueryDeclarationEntryOrchestrationAutomationBoundary;
use super::outcome::WorthQueryDeclarationEntryOrchestrationOutcome;
use super::plan::WorthQueryDeclarationEntryOrchestrationPlan;
use super::step_record::WorthQueryDeclarationEntryOrchestrationStageRecord;

pub struct WorthQueryDeclarationEntryOrchestrationProof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    plan: WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    outcome: WorthQueryDeclarationEntryOrchestrationOutcome<D, I>,
    stage_records: Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    orchestration_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryOrchestrationProof<D, I>
{
    #[cfg(test)]
    pub(crate) fn new(
        plan: WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
        outcome: WorthQueryDeclarationEntryOrchestrationOutcome<D, I>,
        step_records: Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    ) -> Self {
        let orchestration_digest = hash_parts(&[
            format!("plan:{}", plan.orchestration_identity_digest()),
            format!("outcome:{}", outcome.outcome_identity_digest()),
            format!(
                "steps:{}",
                step_records
                    .iter()
                    .map(|record| {
                        format!(
                            "{}:{}:{}:{}",
                            record.stage().as_str(),
                            record.disposition().as_str(),
                            record.retained_digest().unwrap_or("none"),
                            record.reason().unwrap_or("none")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            plan,
            outcome,
            stage_records: step_records,
            orchestration_digest,
        }
    }

    pub fn plan(&self) -> &WorthQueryDeclarationEntryOrchestrationPlan<D, I> {
        &self.plan
    }

    pub fn outcome(&self) -> &WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn step_records(&self) -> &[WorthQueryDeclarationEntryOrchestrationStageRecord] {
        &self.stage_records
    }

    pub fn stage_records(&self) -> &[WorthQueryDeclarationEntryOrchestrationStageRecord] {
        self.step_records()
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn automation_boundary(&self) -> WorthQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.plan.automation_boundary()
    }

    pub fn materialization_policy(
        &self,
    ) -> &WorthQueryDeclarationEntryOrchestrationMaterializationPolicy {
        self.plan.materialization_policy()
    }

    pub fn cost_posture(&self) -> WorthQueryDeclarationEntryOrchestrationCostPosture {
        self.plan.cost_posture()
    }

    pub fn relational_authority_summary(
        &self,
    ) -> &WorthQueryDeclarationRelationalAuthorityAspectSummary {
        self.plan.relational_authority_summary()
    }

    pub fn bridge_authority_summary(&self) -> &WorthQueryDeclarationBridgeAuthorityAspectSummary {
        self.plan.bridge_authority_summary()
    }

    pub fn signal_authority_summary(&self) -> &WorthQueryDeclarationSignalAuthorityAspectSummary {
        self.plan.signal_authority_summary()
    }
}

pub type WorthQueryDeclarationEntryOrchestrationTranscript<D, I> =
    WorthQueryDeclarationEntryOrchestrationProof<D, I>;
