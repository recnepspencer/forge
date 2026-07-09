use worth_foundational::FoundationalDiagnosticOutcomeKind;

use crate::application::{
    WorthQueryDeclarationEntryContributionComposition,
    WorthQueryDeclarationEntryContributionEvidence, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQueryGraphObligationOrchestrationDispatch,
};

use super::composition::{
    WorthQueryContributionComposedClassification, WorthQueryContributionComposedComposition,
};
use super::intent_result::WorthQueryContributionComposedIntentResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedSummary {
    materialization_ready_digest: String,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    primary_code: String,
    required_row_count: usize,
    standard_row_count: usize,
    forensic_row_count: usize,
}

impl WorthQueryContributionComposedSummary {
    pub fn new(
        materialization_ready_digest: impl Into<String>,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        primary_code: impl Into<String>,
        required_row_count: usize,
        standard_row_count: usize,
        forensic_row_count: usize,
    ) -> Self {
        Self {
            materialization_ready_digest: materialization_ready_digest.into(),
            outcome_kind,
            primary_code: primary_code.into(),
            required_row_count,
            standard_row_count,
            forensic_row_count,
        }
    }

    pub fn materialization_ready_digest(&self) -> &str {
        &self.materialization_ready_digest
    }

    pub fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub fn primary_code(&self) -> &str {
        &self.primary_code
    }

    pub fn required_row_count(&self) -> usize {
        self.required_row_count
    }

    pub fn standard_row_count(&self) -> usize {
        self.standard_row_count
    }

    pub fn forensic_row_count(&self) -> usize {
        self.forensic_row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedContribution {
    evidence: WorthQueryDeclarationEntryContributionEvidence,
    contribution_category: crate::domain_capabilities::WorthQueryDomainCapabilityCategory,
    semantic_posture: crate::domain_capabilities::WorthQueryDomainCapabilitySemanticPosture,
    request_digest: String,
    summary: Option<WorthQueryContributionComposedSummary>,
}

impl WorthQueryContributionComposedContribution {
    pub fn new(
        evidence: WorthQueryDeclarationEntryContributionEvidence,
        contribution_category: crate::domain_capabilities::WorthQueryDomainCapabilityCategory,
        semantic_posture: crate::domain_capabilities::WorthQueryDomainCapabilitySemanticPosture,
        request_digest: impl Into<String>,
        summary: Option<WorthQueryContributionComposedSummary>,
    ) -> Self {
        Self {
            evidence,
            contribution_category,
            semantic_posture,
            request_digest: request_digest.into(),
            summary,
        }
    }

    pub fn evidence(&self) -> &WorthQueryDeclarationEntryContributionEvidence {
        &self.evidence
    }

    pub fn contribution_category(
        &self,
    ) -> crate::domain_capabilities::WorthQueryDomainCapabilityCategory {
        self.contribution_category
    }

    pub fn semantic_posture(
        &self,
    ) -> crate::domain_capabilities::WorthQueryDomainCapabilitySemanticPosture {
        self.semantic_posture
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn summary(&self) -> Option<&WorthQueryContributionComposedSummary> {
        self.summary.as_ref()
    }

    pub fn support_outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.summary
            .as_ref()
            .map(WorthQueryContributionComposedSummary::outcome_kind)
            .unwrap_or(FoundationalDiagnosticOutcomeKind::Advisory)
    }
}

pub struct WorthQueryContributionComposedOrchestration<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    envelope: WorthQueryDeclarationEnvelope<D, I>,
    contribution_composition: WorthQueryDeclarationEntryContributionComposition,
    contributions: Vec<WorthQueryContributionComposedContribution>,
    intent_results: Vec<WorthQueryContributionComposedIntentResult>,
    composition: WorthQueryContributionComposedComposition,
    graph_obligation_dispatch: Option<WorthQueryGraphObligationOrchestrationDispatch>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContributionComposedOrchestration<D, I>
{
    pub fn new(
        envelope: WorthQueryDeclarationEnvelope<D, I>,
        contribution_composition: WorthQueryDeclarationEntryContributionComposition,
        contributions: Vec<WorthQueryContributionComposedContribution>,
        intent_results: Vec<WorthQueryContributionComposedIntentResult>,
        composition: WorthQueryContributionComposedComposition,
    ) -> Self {
        Self {
            envelope,
            contribution_composition,
            contributions,
            intent_results,
            composition,
            graph_obligation_dispatch: None,
        }
    }

    pub(crate) fn with_graph_obligation_dispatch(
        mut self,
        dispatch: Option<WorthQueryGraphObligationOrchestrationDispatch>,
    ) -> Self {
        self.graph_obligation_dispatch = dispatch;
        self
    }

    pub fn envelope(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn declaration_artifact(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn contribution_composition(&self) -> &WorthQueryDeclarationEntryContributionComposition {
        &self.contribution_composition
    }

    pub fn intent_results(&self) -> &[WorthQueryContributionComposedIntentResult] {
        &self.intent_results
    }

    pub fn contributions(&self) -> &[WorthQueryContributionComposedContribution] {
        &self.contributions
    }

    pub fn admitted_contributions(&self) -> &[WorthQueryContributionComposedContribution] {
        &self.contributions
    }

    pub fn rejected_intents(&self) -> Vec<&WorthQueryContributionComposedIntentResult> {
        self.intent_results
            .iter()
            .filter(|value| !value.is_admitted())
            .collect()
    }

    pub fn composition(&self) -> &WorthQueryContributionComposedComposition {
        &self.composition
    }

    pub fn classification(&self) -> WorthQueryContributionComposedClassification {
        self.composition.classification()
    }

    pub fn materialized_artifacts(&self) -> Vec<&WorthQueryContributionComposedSummary> {
        self.intent_results
            .iter()
            .filter_map(|value| {
                value
                    .contribution()
                    .and_then(WorthQueryContributionComposedContribution::summary)
            })
            .collect()
    }

    pub fn composed_digest(&self) -> &str {
        self.composition.composition_for_reporting()
    }

    pub fn composition_for_reporting(&self) -> &str {
        self.composition.composition_for_reporting()
    }

    pub fn composition_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        self.composition.composition_identity()
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryGraphObligationOrchestrationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }
}
