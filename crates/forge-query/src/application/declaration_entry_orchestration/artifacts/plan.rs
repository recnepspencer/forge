use crate::application::ForgeQueryDeclarationFamilyMarker;
use crate::application::ForgeQueryDeclarationRouteIntent;
use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationBridgeAuthorityAspectSummary, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRelationalAuthorityAspectSummary,
    ForgeQueryDeclarationSignalAuthorityAspectSummary, ForgeQueryDomainEntryMarker,
};
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
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
};
use super::input::ForgeQueryDeclarationEntryOrchestrationInput;
use super::plan_build::{
    automation_steps_for, explicit_caller_handoff_steps_for, materialization_tier_for_product,
    step_plan_for,
};
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
    relational_authority_summary: ForgeQueryDeclarationRelationalAuthorityAspectSummary,
    bridge_authority_summary: ForgeQueryDeclarationBridgeAuthorityAspectSummary,
    signal_authority_summary: ForgeQueryDeclarationSignalAuthorityAspectSummary,
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
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionAdmitted,
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
                input.aspect_contract(),
                input.aspect_coverage(),
            );
        let step_plan = step_plan_for(product, starting_artifact_stage);
        let automation_steps = automation_steps_for(&step_plan);
        let explicit_caller_handoff_steps = explicit_caller_handoff_steps_for(product);
        let materialization_tier =
            materialization_tier_for_product(&materialization_policy, product);
        let descriptive_materialization_cost = Some(descriptive_materialization_cost_for_tier(
            materialization_tier,
        ));
        let relational_authority_summary =
            crate::application::relational_authority_summary_from_publication(
                input.aspect_contract(),
                materialization_policy.envelope_aspect_publication(),
                I::Family::relational_truth_contract().as_ref(),
            );
        let bridge_authority_summary =
            crate::application::bridge_authority_summary_from_publication(
                input.aspect_contract(),
                materialization_policy.envelope_aspect_publication(),
                I::Family::bridge_continuation_contract().as_ref(),
            );
        let signal_authority_summary =
            crate::application::signal_authority_summary_from_publication(
                input.aspect_contract(),
                materialization_policy.envelope_aspect_publication(),
                I::Family::signal_compatibility_contract().as_ref(),
            );
        let orchestration_identity_digest = hash_parts(&[
            format!("family:{}", input.declaration_family_key()),
            format!("handle:{}", input.handle_identity_digest()),
            format!(
                "operating_context:{}",
                input.operating_context_identity_digest()
            ),
            format!("product:{}", product.as_str()),
            format!("exposure_level:{}", input.exposure_level().as_str()),
            format!("artifact_policy:{}", input.artifact_policy().as_str()),
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
            format!("aspect_contract:{:?}", input.aspect_contract()),
            format!("aspect_coverage:{:?}", input.aspect_coverage()),
            format!("aspect_coverage_basis:{:?}", input.aspect_coverage_basis()),
            format!(
                "foundational_aspect_publication:{:?}",
                materialization_policy.foundational_aspect_publication()
            ),
            format!(
                "receipt_aspect_publication:{:?}",
                materialization_policy.receipt_aspect_publication()
            ),
            format!(
                "envelope_aspect_publication:{:?}",
                materialization_policy.envelope_aspect_publication()
            ),
            format!("relational_authority_summary:{relational_authority_summary:?}"),
            format!("bridge_authority_summary:{bridge_authority_summary:?}"),
            format!("signal_authority_summary:{signal_authority_summary:?}"),
            format!(
                "foundational_evidence_profile:{:?}",
                materialization_policy.foundational_evidence_profile()
            ),
            format!("receipt_tier:{:?}", materialization_policy.receipt_tier()),
            format!("envelope_tier:{:?}", materialization_policy.envelope_tier()),
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
            relational_authority_summary,
            bridge_authority_summary,
            signal_authority_summary,
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

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        self.input.aspect_contract()
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        self.input.aspect_coverage()
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.input.aspect_coverage_basis()
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

    pub fn foundational_aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        self.materialization_policy
            .foundational_aspect_publication()
    }

    pub fn receipt_aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        self.materialization_policy.receipt_aspect_publication()
    }

    pub fn envelope_aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        self.materialization_policy.envelope_aspect_publication()
    }

    pub fn relational_authority_summary(
        &self,
    ) -> &ForgeQueryDeclarationRelationalAuthorityAspectSummary {
        &self.relational_authority_summary
    }

    pub fn bridge_authority_summary(&self) -> &ForgeQueryDeclarationBridgeAuthorityAspectSummary {
        &self.bridge_authority_summary
    }

    pub fn signal_authority_summary(&self) -> &ForgeQueryDeclarationSignalAuthorityAspectSummary {
        &self.signal_authority_summary
    }

    pub fn descriptive_materialization_cost(&self) -> Option<FoundationalMaterializationCost> {
        self.descriptive_materialization_cost
    }

    pub fn orchestration_identity_digest(&self) -> &str {
        &self.orchestration_identity_digest
    }
}
