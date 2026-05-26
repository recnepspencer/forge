use crate::application::ForgeQueryDeclarationRouteIntent;
use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::identity::hash_parts;
use forge_foundational::facade::{
    FoundationalBoundaryEvidenceMaterializationProfile, FoundationalMaterializationCost,
};

use super::super::materialization::{
    descriptive_materialization_cost_for_tier, ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationMaterializationGate,
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};
use super::super::sequencing::{
    envelope_ceiling_automation_steps, ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
};
use super::input::ForgeQueryDeclarationEntryOrchestrationInput;
use super::product::ForgeQueryDeclarationEntryOrchestrationProduct;
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStage;

pub struct ForgeQueryDeclarationEntryOrchestrationPlan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>,
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
    requested_route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    starting_artifact_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    ceiling_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    step_plan: Vec<ForgeQueryDeclarationEntryOrchestrationStage>,
    automation_boundary: ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    automation_steps: Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep>,
    explicit_caller_handoff_steps: Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep>,
    materialization_policy: ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    materialization_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    descriptive_materialization_cost: Option<FoundationalMaterializationCost>,
    orchestration_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationPlan<D, I>
{
    pub(crate) fn new(input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>) -> Self {
        Self::for_product(
            input,
            ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
            None,
            ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
        )
    }

    pub(crate) fn from_progressed(
        input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>,
        product: ForgeQueryDeclarationEntryOrchestrationProduct,
        requested_route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    ) -> Self {
        Self::for_product(
            input,
            product,
            requested_route_intent,
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
        )
    }

    fn for_product(
        input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>,
        product: ForgeQueryDeclarationEntryOrchestrationProduct,
        requested_route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        starting_artifact_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    ) -> Self {
        let automation_boundary =
            ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling;
        let materialization_policy =
            ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy::default_for_lane(
                input.exposure_level(),
                input.artifact_policy(),
            );
        let step_plan = step_plan_for(product, starting_artifact_stage);
        let automation_steps = automation_steps_for(&step_plan);
        let explicit_caller_handoff_steps = explicit_caller_handoff_steps_for(product);
        let materialization_tier =
            materialization_tier_for_product(&materialization_policy, product);
        let descriptive_materialization_cost = Some(descriptive_materialization_cost_for_tier(
            materialization_tier,
        ));
        let orchestration_identity_digest = hash_parts(&[
            format!("family:{}", input.declaration_family_key()),
            format!("handle:{}", input.handle_identity_digest()),
            format!(
                "operating_context:{}",
                input.operating_context_identity_digest()
            ),
            format!("product:{}", product.as_str()),
            format!("starting_stage:{}", starting_artifact_stage.as_str()),
            "ceiling:envelope_constructed".to_string(),
            format!(
                "route_intent:{}",
                requested_route_intent.map_or("none", |intent| intent.as_str())
            ),
            format!(
                "steps:{}",
                step_plan
                    .iter()
                    .map(|stage| stage.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!("automation_boundary:{}", automation_boundary.as_str()),
            format!(
                "automation_steps:{}",
                automation_steps
                    .iter()
                    .map(|step| step.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            input,
            product,
            requested_route_intent,
            starting_artifact_stage,
            ceiling_stage: ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            step_plan,
            automation_boundary,
            automation_steps,
            explicit_caller_handoff_steps,
            materialization_policy,
            materialization_tier,
            descriptive_materialization_cost,
            orchestration_identity_digest,
        }
    }

    pub fn product(&self) -> ForgeQueryDeclarationEntryOrchestrationProduct {
        self.product
    }

    pub fn requested_route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
        self.requested_route_intent
    }

    pub fn starting_artifact_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.starting_artifact_stage
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.input.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.input.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.input.operating_context_identity_digest()
    }

    pub fn exposure_level(
        &self,
    ) -> super::exposure::ForgeQueryDeclarationEntryOrchestrationExposureLevel {
        self.input.exposure_level()
    }

    pub fn artifact_policy(
        &self,
    ) -> super::policy::ForgeQueryDeclarationEntryOrchestrationArtifactPolicy {
        self.input.artifact_policy()
    }

    pub fn ceiling_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.ceiling_stage
    }

    pub fn step_plan(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStage] {
        &self.step_plan
    }

    pub fn automation_boundary(&self) -> ForgeQueryDeclarationEntryOrchestrationAutomationBoundary {
        self.automation_boundary
    }

    pub fn automation_steps(&self) -> &[ForgeQueryDeclarationEntryOrchestrationAutomationStep] {
        &self.automation_steps
    }

    pub fn explicit_caller_handoff_steps(
        &self,
    ) -> &[ForgeQueryDeclarationEntryOrchestrationAutomationStep] {
        &self.explicit_caller_handoff_steps
    }

    pub fn materialization_policy(
        &self,
    ) -> &ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
        &self.materialization_policy
    }

    pub fn materialization_tier(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.materialization_tier
    }

    pub fn receipt_materialization_tier(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.materialization_policy.receipt_tier()
    }

    pub fn envelope_materialization_tier(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
        self.materialization_policy.envelope_tier()
    }

    pub fn cost_posture(&self) -> ForgeQueryDeclarationEntryOrchestrationCostPosture {
        self.materialization_policy.cost_posture()
    }

    pub fn materialization_gate(
        &self,
    ) -> ForgeQueryDeclarationEntryOrchestrationMaterializationGate {
        self.materialization_policy.materialization_gate()
    }

    pub fn foundational_evidence_profile(
        &self,
    ) -> FoundationalBoundaryEvidenceMaterializationProfile {
        self.materialization_policy.foundational_evidence_profile()
    }

    pub fn descriptive_materialization_cost(&self) -> Option<FoundationalMaterializationCost> {
        self.descriptive_materialization_cost
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        &self.orchestration_identity_digest
    }
}

fn step_plan_for(
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
    starting_artifact_stage: ForgeQueryDeclarationEntryOrchestrationStage,
) -> Vec<ForgeQueryDeclarationEntryOrchestrationStage> {
    let full = match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => vec![
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => {
            if starting_artifact_stage
                == ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle
            {
                vec![
                    ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                ]
            } else {
                vec![
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                ]
            }
        }
    };
    full.into_iter()
        .skip_while(|stage| *stage != starting_artifact_stage)
        .collect()
}

fn automation_steps_for(
    step_plan: &[ForgeQueryDeclarationEntryOrchestrationStage],
) -> Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep> {
    let full = envelope_ceiling_automation_steps();
    let first_step = step_plan.first().map(|stage| match stage {
        ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle
        }
        ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration
        }
        ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Legality
        }
        ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Progression
        }
        ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence
        }
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan
        }
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt
        }
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed => {
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope
        }
    });
    full.into_iter()
        .skip_while(|step| Some(*step) != first_step)
        .collect()
}

fn explicit_caller_handoff_steps_for(
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
) -> Vec<ForgeQueryDeclarationEntryOrchestrationAutomationStep> {
    match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => vec![
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
        ],
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => {
            vec![ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope]
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => Vec::new(),
    }
}

fn materialization_tier_for_product(
    policy: &ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
) -> ForgeQueryDeclarationEntryOrchestrationMaterializationTier {
    match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::from(
                policy.foundational_evidence_profile(),
            )
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => policy.receipt_tier(),
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => policy.envelope_tier(),
    }
}

impl From<FoundationalBoundaryEvidenceMaterializationProfile>
    for ForgeQueryDeclarationEntryOrchestrationMaterializationTier
{
    fn from(value: FoundationalBoundaryEvidenceMaterializationProfile) -> Self {
        match value {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
                Self::OperationalLean
            }
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
                Self::SupportReady
            }
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
                Self::FullDescriptive
            }
        }
    }
}
