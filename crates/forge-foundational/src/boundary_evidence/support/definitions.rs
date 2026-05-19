use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceSupportTruthKind {
    EvidenceBundle,
    CertificationSummary,
    ParityArtifact,
    DegradedRecoveryReport,
    StaleBasisDisclosure,
    TransientLifecycleEvidence,
    ResidualDebtStatement,
}

pub const fn foundational_boundary_evidence_support_truth_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceSupportTruthKind>;
       7] {
    [
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::EvidenceBundle,
            "evidence_bundle",
            "a support-grade bundle of retained evidence and descriptive recovery context",
            "stronger authority truth or a completed execution receipt",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::CertificationSummary,
            "certification_summary",
            "a descriptive support summary about certification posture or parity",
            "a proof-bearing certification artifact or current-basis readmission",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::ParityArtifact,
            "parity_artifact",
            "a support-grade artifact describing parity or comparison posture",
            "canonical basis authority or a digest identity by itself",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::DegradedRecoveryReport,
            "degraded_recovery_report",
            "a support report describing blocked, denied, stale, or degraded recovery posture",
            "an executed authority boundary or retained current-fresh continuity",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::StaleBasisDisclosure,
            "stale_basis_disclosure",
            "a support artifact whose primary meaning is freshness or retained-basis limitation disclosure",
            "fresh retained authority truth or a missing explanation",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::TransientLifecycleEvidence,
            "transient_lifecycle_evidence",
            "support-grade evidence for a participant that opened and closed within one executed boundary",
            "durable lineage continuity or surviving authority state",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportTruthKind::ResidualDebtStatement,
            "residual_debt_statement",
            "a support-grade statement of remaining rebuild, quarantine, freshness, or retention debt",
            "a stronger proof artifact or a hidden TODO",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceSupportRecoveryPosture {
    CheckpointResumed,
    ReplayReconstructed,
    RebuildRequired,
    Quarantined,
}

pub const fn foundational_boundary_evidence_support_recovery_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
>; 4] {
    [
        definition(
            FoundationalBoundaryEvidenceSupportRecoveryPosture::CheckpointResumed,
            "checkpoint_resumed",
            "support truth derived after a checkpoint or resume boundary",
            "fresh uninterrupted execution with no recovery seam",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportRecoveryPosture::ReplayReconstructed,
            "replay_reconstructed",
            "support truth reconstructed from replay, snapshots, or retained slices",
            "direct retained authority continuity",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportRecoveryPosture::RebuildRequired,
            "rebuild_required",
            "support truth that remains usable only while rebuild debt is still outstanding",
            "fully restored parity or complete support closure",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportRecoveryPosture::Quarantined,
            "quarantined",
            "support truth preserved under an explicitly quarantined recovery posture",
            "normal healthy support publication",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceSupportBasisDisclosure {
    CompleteBasis,
    StaleBasis,
    ReducedBasis,
    ReducedAndStaleBasis,
}

pub const fn foundational_boundary_evidence_support_basis_disclosure_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
>; 4] {
    [
        definition(
            FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
            "complete_basis",
            "support truth produced from a complete retained basis with no reduced-basis disclosure",
            "stale, reduced, or replay-only basis hidden behind a fresh-looking summary",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis,
            "stale_basis",
            "support truth produced from basis that is retained but no longer fresh",
            "fresh complete support truth",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis,
            "reduced_basis",
            "support truth produced from a reduced retained basis with explicit scope limits",
            "complete parity or silent omission of missing basis",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedAndStaleBasis,
            "reduced_and_stale_basis",
            "support truth produced from basis that is both reduced and stale",
            "fresh complete support truth or a one-axis disclosure",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceSupportResidualDebtKind {
    RebuildRequired,
    QuarantineRequired,
    ReducedBasisLimitsParity,
    StaleBasisLimitsFreshness,
}

pub const fn foundational_boundary_evidence_support_residual_debt_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
>; 4] {
    [
        definition(
            FoundationalBoundaryEvidenceSupportResidualDebtKind::RebuildRequired,
            "rebuild_required",
            "support truth still depends on a future rebuild to restore fuller parity",
            "a fully closed and parity-complete support state",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportResidualDebtKind::QuarantineRequired,
            "quarantine_required",
            "support truth remains under quarantine and cannot claim normal recovery posture",
            "a normal non-quarantined support publication",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportResidualDebtKind::ReducedBasisLimitsParity,
            "reduced_basis_limits_parity",
            "reduced retained basis prevents full parity or continuity coverage",
            "complete retained support basis",
        ),
        definition(
            FoundationalBoundaryEvidenceSupportResidualDebtKind::StaleBasisLimitsFreshness,
            "stale_basis_limits_freshness",
            "stale retained basis prevents fresh support claims",
            "fresh retained support basis",
        ),
    ]
}
