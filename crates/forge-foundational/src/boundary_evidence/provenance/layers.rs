use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceProvenanceLayerKind {
    SourceBasis,
    AuthorityPath,
    StrategyBasis,
    ProfileBasis,
    ComparisonBasis,
    CanonicalDigestBasis,
    SupportContextAttachment,
}

pub const fn foundational_boundary_evidence_provenance_layer_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceProvenanceLayerKind,
>; 7] {
    [
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::SourceBasis,
            "source_basis",
            "the retained or named source basis from which provenance starts",
            "execution attestation, continuity verdicts, or support closeout",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::AuthorityPath,
            "authority_path",
            "the transition or authority-path locator that explains how the artifact was reached",
            "source basis, continuity truth, or support-grade debt claims",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::StrategyBasis,
            "strategy_basis",
            "the named strategy identity that shaped the producing path",
            "execution receipt truth or support-grade parity evidence",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::ProfileBasis,
            "profile_basis",
            "the admitted profile identity governing descriptive richness or posture",
            "source basis, continuity verdicts, or execution attestation",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::ComparisonBasis,
            "comparison_basis",
            "the comparison or equivalence basis used to interpret paired meaning",
            "source retention context or durable continuity law",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::CanonicalDigestBasis,
            "canonical_digest_basis",
            "canonical or digest identity context attached to the provenance result",
            "source basis itself or proof-bearing equivalence law",
        ),
        definition(
            FoundationalBoundaryEvidenceProvenanceLayerKind::SupportContextAttachment,
            "support_context_attachment",
            "support-context scope or code attachments that help explain the provenance result",
            "support-grade truth, execution attestation, or continuity verdicts",
        ),
    ]
}
