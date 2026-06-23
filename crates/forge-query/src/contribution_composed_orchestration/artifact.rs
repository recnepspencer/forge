use forge_foundational::FoundationalDiagnosticOutcomeKind;

use crate::application::{
    ForgeQueryDeclarationEntryContributionComposition,
    ForgeQueryDeclarationEntryContributionEvidence, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryGraphObligationOrchestrationDispatch,
};

use super::composition::{
    ForgeQueryContributionComposedClassification, ForgeQueryContributionComposedComposition,
};
use super::intent_result::ForgeQueryContributionComposedIntentResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedSummary {
    materialization_ready_digest: String,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    primary_code: String,
    required_row_count: usize,
    standard_row_count: usize,
    forensic_row_count: usize,
}

impl ForgeQueryContributionComposedSummary {
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
pub struct ForgeQueryContributionComposedContribution {
    evidence: ForgeQueryDeclarationEntryContributionEvidence,
    contribution_category: crate::domain_capabilities::ForgeQueryDomainCapabilityCategory,
    semantic_posture: crate::domain_capabilities::ForgeQueryDomainCapabilitySemanticPosture,
    request_digest: String,
    summary: Option<ForgeQueryContributionComposedSummary>,
}

impl ForgeQueryContributionComposedContribution {
    pub fn new(
        evidence: ForgeQueryDeclarationEntryContributionEvidence,
        contribution_category: crate::domain_capabilities::ForgeQueryDomainCapabilityCategory,
        semantic_posture: crate::domain_capabilities::ForgeQueryDomainCapabilitySemanticPosture,
        request_digest: impl Into<String>,
        summary: Option<ForgeQueryContributionComposedSummary>,
    ) -> Self {
        Self {
            evidence,
            contribution_category,
            semantic_posture,
            request_digest: request_digest.into(),
            summary,
        }
    }

    pub fn evidence(&self) -> &ForgeQueryDeclarationEntryContributionEvidence {
        &self.evidence
    }

    pub fn contribution_category(
        &self,
    ) -> crate::domain_capabilities::ForgeQueryDomainCapabilityCategory {
        self.contribution_category
    }

    pub fn semantic_posture(
        &self,
    ) -> crate::domain_capabilities::ForgeQueryDomainCapabilitySemanticPosture {
        self.semantic_posture
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn summary(&self) -> Option<&ForgeQueryContributionComposedSummary> {
        self.summary.as_ref()
    }

    pub fn support_outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.summary
            .as_ref()
            .map(ForgeQueryContributionComposedSummary::outcome_kind)
            .unwrap_or(FoundationalDiagnosticOutcomeKind::Advisory)
    }
}

pub struct ForgeQueryContributionComposedOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    contribution_composition: ForgeQueryDeclarationEntryContributionComposition,
    contributions: Vec<ForgeQueryContributionComposedContribution>,
    intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
    composition: ForgeQueryContributionComposedComposition,
    graph_obligation_dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestration<D, I>
{
    pub fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        contribution_composition: ForgeQueryDeclarationEntryContributionComposition,
        contributions: Vec<ForgeQueryContributionComposedContribution>,
        intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
        composition: ForgeQueryContributionComposedComposition,
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
        dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    ) -> Self {
        self.graph_obligation_dispatch = dispatch;
        self
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn declaration_artifact(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn contribution_composition(&self) -> &ForgeQueryDeclarationEntryContributionComposition {
        &self.contribution_composition
    }

    pub fn intent_results(&self) -> &[ForgeQueryContributionComposedIntentResult] {
        &self.intent_results
    }

    pub fn contributions(&self) -> &[ForgeQueryContributionComposedContribution] {
        &self.contributions
    }

    pub fn admitted_contributions(&self) -> &[ForgeQueryContributionComposedContribution] {
        &self.contributions
    }

    pub fn rejected_intents(&self) -> Vec<&ForgeQueryContributionComposedIntentResult> {
        self.intent_results
            .iter()
            .filter(|value| !value.is_admitted())
            .collect()
    }

    pub fn composition(&self) -> &ForgeQueryContributionComposedComposition {
        &self.composition
    }

    pub fn classification(&self) -> ForgeQueryContributionComposedClassification {
        self.composition.classification()
    }

    pub fn materialized_artifacts(&self) -> Vec<&ForgeQueryContributionComposedSummary> {
        self.intent_results
            .iter()
            .filter_map(|value| {
                value
                    .contribution()
                    .and_then(ForgeQueryContributionComposedContribution::summary)
            })
            .collect()
    }

    pub fn composed_digest(&self) -> &str {
        self.composition.composition_for_reporting()
    }

    pub fn composition_for_reporting(&self) -> &str {
        self.composition.composition_for_reporting()
    }

    pub fn composition_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        self.composition.composition_identity()
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }
}
