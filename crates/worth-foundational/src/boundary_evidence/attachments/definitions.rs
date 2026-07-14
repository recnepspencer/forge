use super::super::primitives::{definition, FoundationalBoundaryEvidencePrimitiveDefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceAttachmentTargetKind {
    BoundaryArtifact,
    TransitionArtifact,
    DiagnosticBundle,
}

pub const fn foundational_boundary_evidence_attachment_target_kind_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceAttachmentTargetKind,
>; 3] {
    [
        definition(
            FoundationalBoundaryEvidenceAttachmentTargetKind::BoundaryArtifact,
            "boundary_artifact",
            "an attachment bundle whose primary target is a boundary artifact surface",
            "a transition artifact or diagnostic bundle target",
        ),
        definition(
            FoundationalBoundaryEvidenceAttachmentTargetKind::TransitionArtifact,
            "transition_artifact",
            "an attachment bundle whose primary target is a transition boundary surface",
            "a boundary artifact or diagnostic bundle target",
        ),
        definition(
            FoundationalBoundaryEvidenceAttachmentTargetKind::DiagnosticBundle,
            "diagnostic_bundle",
            "an attachment bundle whose primary target is a diagnostic bundle surface",
            "a boundary artifact or transition target",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceContinuityAttachmentScope {
    ObjectLevel,
    LocatorLevel,
}

pub const fn foundational_boundary_evidence_continuity_attachment_scope_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
>; 2] {
    [
        definition(
            FoundationalBoundaryEvidenceContinuityAttachmentScope::ObjectLevel,
            "object_level",
            "continuity attached to the identity of the object or subject itself",
            "a pointer, path, field, or locator-only relocation claim",
        ),
        definition(
            FoundationalBoundaryEvidenceContinuityAttachmentScope::LocatorLevel,
            "locator_level",
            "continuity attached only to a pointer, path, field, or locator relation",
            "a whole-object identity continuity claim",
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceMaterializationProfile {
    FullDescriptiveRichness,
    ElideDiagnostics,
    ElideSupportAndDiagnostics,
}

pub const fn foundational_boundary_evidence_materialization_profile_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
    FoundationalBoundaryEvidenceMaterializationProfile,
>; 3] {
    [
        definition(
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
            "full_descriptive_richness",
            "materialize all optional descriptive surfaces that were present on the input bundle",
            "forced elision of optional support or diagnostics",
        ),
        definition(
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics,
            "elide_diagnostics",
            "preserve lineage, provenance, receipt, and support while removing optional diagnostic attachments",
            "changing authority truth or stripping support-richness together with diagnostics",
        ),
        definition(
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
            "elide_support_and_diagnostics",
            "preserve continuity, provenance, and receipt while eliding optional support and diagnostic attachments",
            "changing authority truth or silently dropping provenance or receipt",
        ),
    ]
}
