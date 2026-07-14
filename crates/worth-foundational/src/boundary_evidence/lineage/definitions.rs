use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceLineageOutcomeKind {
    SingularContinuity,
    PluralSuccessorPredecessor,
    MergeSuccessor,
    BranchLocalReplacement,
    RestoredContinuity,
    ReconstructedEquivalence,
    NamedGapPartialContinuity,
    WithheldRedactedContinuity,
    TransientWithinBoundaryClosure,
    AdvisoryCorrespondenceCandidate,
    Ambiguity,
    IdentityBreak,
    Denial,
}

pub const fn foundational_boundary_evidence_lineage_outcome_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceLineageOutcomeKind>;
       13] {
    [
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::SingularContinuity,
            "singular_continuity",
            "one retained or directly attested surviving continuity claim",
            "replay-only reconstruction, ambiguity, or a support-grade hint",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::PluralSuccessorPredecessor,
            "plural_successor_predecessor",
            "a retained plurality of successor or predecessor continuity subjects",
            "one exact continuity node or a merge-specific outcome by default",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::MergeSuccessor,
            "merge_successor",
            "a continuity outcome where multiple retained paths converge into one successor",
            "an arbitrary plurality or a branch-local replacement alone",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::BranchLocalReplacement,
            "branch_local_replacement",
            "a branch-local replacement or supersession outcome prior to global promotion",
            "globally admitted continuity or replay-only reconstruction",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::RestoredContinuity,
            "restored_continuity",
            "direct continuity re-established through restoration or checkpoint resume",
            "reconstructed equivalence or ordinary current-fresh continuity",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::ReconstructedEquivalence,
            "reconstructed_equivalence",
            "a likely or reconstructed equivalent continuity claim that is not direct restored truth",
            "retained direct continuity or restored attestation",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::NamedGapPartialContinuity,
            "named_gap_partial_continuity",
            "a continuity outcome with explicit named missing or elided detail",
            "complete continuity or silent omission",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::WithheldRedactedContinuity,
            "withheld_redacted_continuity",
            "a continuity outcome intentionally withheld or redacted at descriptive level",
            "complete retained lineage or producer-private silence",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::TransientWithinBoundaryClosure,
            "transient_within_boundary_closure",
            "a subject that opened and closed within one completed boundary without becoming durable lineage",
            "a surviving successor, predecessor, or current continuity node",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::AdvisoryCorrespondenceCandidate,
            "advisory_correspondence_candidate",
            "a descriptive candidate correspondence that does not rise to direct continuity truth",
            "retained direct continuity or completed authority attestation by itself",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::Ambiguity,
            "ambiguity",
            "an explicit multi-candidate or unresolved continuity ambiguity",
            "one exact surviving continuity answer",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::IdentityBreak,
            "identity_break",
            "an attested break where continuity does not survive across the boundary",
            "mere absence of evidence or replay uncertainty",
        ),
        definition(
            FoundationalBoundaryEvidenceLineageOutcomeKind::Denial,
            "denial",
            "an explicit denied continuity claim",
            "a missing row or silent default false",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceBranchDivergencePosture {
    BranchLocalOnly,
    SupersededBeforePromotion,
}

pub const fn foundational_boundary_evidence_branch_divergence_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceBranchDivergencePosture,
>; 2] {
    [
        definition(
            FoundationalBoundaryEvidenceBranchDivergencePosture::BranchLocalOnly,
            "branch_local_only",
            "a branch-local continuity result not yet promoted into global authority",
            "globally admitted continuity or replay-derived equivalence",
        ),
        definition(
            FoundationalBoundaryEvidenceBranchDivergencePosture::SupersededBeforePromotion,
            "superseded_before_promotion",
            "a branch-local replacement outcome superseded before global promotion occurred",
            "a surviving globally promoted continuity result",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidencePromotionPosture {
    PromotedToGlobalContinuity,
    PromotionDenied,
}

pub const fn foundational_boundary_evidence_promotion_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidencePromotionPosture>;
       2] {
    [
        definition(
            FoundationalBoundaryEvidencePromotionPosture::PromotedToGlobalContinuity,
            "promoted_to_global_continuity",
            "a branch-local result that was later promoted into globally admitted continuity",
            "an ordinary branch-local result or a silent ambient promotion",
        ),
        definition(
            FoundationalBoundaryEvidencePromotionPosture::PromotionDenied,
            "promotion_denied",
            "a branch-local result whose promotion was explicitly denied",
            "a completed global promotion",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceLineagePartialityPosture {
    NamedGap,
    WithheldRedacted,
    Denied,
}

pub const fn foundational_boundary_evidence_lineage_partiality_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceLineagePartialityPosture,
>; 3] {
    [
        definition(
            FoundationalBoundaryEvidenceLineagePartialityPosture::NamedGap,
            "named_gap",
            "lineage detail is partially present with explicit named missing seams",
            "complete retained continuity or silent omission",
        ),
        definition(
            FoundationalBoundaryEvidenceLineagePartialityPosture::WithheldRedacted,
            "withheld_redacted",
            "lineage detail is intentionally withheld or redacted",
            "complete retained continuity or producer-private absence",
        ),
        definition(
            FoundationalBoundaryEvidenceLineagePartialityPosture::Denied,
            "denied",
            "lineage detail is explicitly denied rather than absent by accident",
            "a positive continuity claim or a missing explanation",
        ),
    ]
}
