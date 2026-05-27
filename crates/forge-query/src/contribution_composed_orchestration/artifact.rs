use forge_foundational::FoundationalDiagnosticOutcomeKind;

use crate::application::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionComposition,
    ForgeQueryDeclarationEntryContributionEvidence, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilitySemanticPosture,
};
use crate::identity::hash_parts;

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
    pub(crate) fn new(
        materialization_ready_digest: String,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        primary_code: String,
        required_row_count: usize,
        standard_row_count: usize,
        forensic_row_count: usize,
    ) -> Self {
        Self {
            materialization_ready_digest,
            outcome_kind,
            primary_code,
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
    contribution_category: ForgeQueryDomainCapabilityCategory,
    semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
    request_digest: String,
    summary: Option<ForgeQueryContributionComposedSummary>,
}

impl ForgeQueryContributionComposedContribution {
    pub(crate) fn new(
        evidence: ForgeQueryDeclarationEntryContributionEvidence,
        contribution_category: ForgeQueryDomainCapabilityCategory,
        semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
        request_digest: String,
        summary: Option<ForgeQueryContributionComposedSummary>,
    ) -> Self {
        Self {
            evidence,
            contribution_category,
            semantic_posture,
            request_digest,
            summary,
        }
    }

    pub fn evidence(&self) -> &ForgeQueryDeclarationEntryContributionEvidence {
        &self.evidence
    }

    pub fn category_family(&self) -> ForgeQueryDeclarationEntryContributionCategoryFamily {
        self.evidence.category_family()
    }

    pub fn contribution_category(&self) -> ForgeQueryDomainCapabilityCategory {
        self.contribution_category
    }

    pub fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture {
        self.semantic_posture
    }

    pub fn support_outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.semantic_posture.outcome_kind()
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn summary(&self) -> Option<&ForgeQueryContributionComposedSummary> {
        self.summary.as_ref()
    }
}

pub struct ForgeQueryContributionComposedOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    contribution_composition: ForgeQueryDeclarationEntryContributionComposition,
    contributions: Vec<ForgeQueryContributionComposedContribution>,
    composed_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestration<D, I>
{
    pub(crate) fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        contribution_composition: ForgeQueryDeclarationEntryContributionComposition,
        contributions: Vec<ForgeQueryContributionComposedContribution>,
    ) -> Self {
        let mut digest_parts = vec![
            format!("envelope:{:?}", envelope.envelope_digest()),
            format!(
                "composition:{}",
                contribution_composition.contribution_digest()
            ),
        ];
        digest_parts.extend(
            contributions
                .iter()
                .map(|value| value.evidence().evidence_digest().to_string()),
        );
        digest_parts.extend(contributions.iter().filter_map(|value| {
            value.summary().map(|summary| {
                format!(
                    "summary:{}:{}:{}:{}:{}",
                    summary.materialization_ready_digest(),
                    summary.primary_code(),
                    summary.required_row_count(),
                    summary.standard_row_count(),
                    summary.forensic_row_count(),
                )
            })
        }));
        let composed_digest = hash_parts(&digest_parts);
        Self {
            envelope,
            contribution_composition,
            contributions,
            composed_digest,
        }
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn contribution_composition(&self) -> &ForgeQueryDeclarationEntryContributionComposition {
        &self.contribution_composition
    }

    pub fn contributions(&self) -> &[ForgeQueryContributionComposedContribution] {
        &self.contributions
    }

    pub fn materialized_artifacts(&self) -> Vec<&ForgeQueryContributionComposedSummary> {
        self.contributions
            .iter()
            .filter_map(ForgeQueryContributionComposedContribution::summary)
            .collect()
    }

    pub fn composed_digest(&self) -> &str {
        &self.composed_digest
    }
}
