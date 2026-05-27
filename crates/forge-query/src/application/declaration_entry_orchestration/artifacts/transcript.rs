use crate::application::{
    ForgeQueryDeclarationBridgeAuthorityAspectSummary,
    ForgeQueryDeclarationRelationalAuthorityAspectSummary,
    ForgeQueryDeclarationSignalAuthorityAspectSummary,
};
use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::identity::hash_parts;

use super::super::materialization::{
    ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
};
use super::super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationBoundary;
use super::outcome::ForgeQueryDeclarationEntryOrchestrationOutcome;
use super::plan::ForgeQueryDeclarationEntryOrchestrationPlan;
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStageRecord;

pub struct ForgeQueryDeclarationEntryOrchestrationProof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
    stage_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    orchestration_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationProof<D, I>
{
    pub(crate) fn new(
        plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
        outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
        step_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
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

    pub fn plan(&self) -> &ForgeQueryDeclarationEntryOrchestrationPlan<D, I> {
        &self.plan
    }

    pub fn outcome(&self) -> &ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn step_records(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStageRecord] {
        &self.stage_records
    }

    pub fn stage_records(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStageRecord] {
        self.step_records()
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn automation_boundary(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.plan.automation_boundary()
    }

    pub fn materialization_policy(
        &self,
    ) -> &ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
        self.plan.materialization_policy()
    }

    pub fn cost_posture(&self) -> ForgeQueryDeclarationEntryOrchestrationCostPosture {
        self.plan.cost_posture()
    }

    pub fn relational_authority_summary(
        &self,
    ) -> &ForgeQueryDeclarationRelationalAuthorityAspectSummary {
        self.plan.relational_authority_summary()
    }

    pub fn bridge_authority_summary(&self) -> &ForgeQueryDeclarationBridgeAuthorityAspectSummary {
        self.plan.bridge_authority_summary()
    }

    pub fn signal_authority_summary(&self) -> &ForgeQueryDeclarationSignalAuthorityAspectSummary {
        self.plan.signal_authority_summary()
    }
}

pub type ForgeQueryDeclarationEntryOrchestrationTranscript<D, I> =
    ForgeQueryDeclarationEntryOrchestrationProof<D, I>;
