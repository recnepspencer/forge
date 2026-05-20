use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::attachment_front_doors::FoundationalBoundaryEvidenceAttachmentFrontDoor;
use crate::boundary_evidence::attachments::{
    foundational_boundary_evidence_attachment_target_kind_definitions,
    foundational_boundary_evidence_continuity_attachment_scope_definitions,
    foundational_boundary_evidence_materialization_profile_definitions,
    FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceMaterializationProfile,
};
use crate::boundary_evidence::primitives::FoundationalBoundaryEvidencePrimitiveDefinition;

impl BoundaryEvidenceFrontDoor {
    pub const fn attachment_target_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceAttachmentTargetKind,
    >; 3] {
        foundational_boundary_evidence_attachment_target_kind_definitions()
    }

    pub const fn continuity_attachment_scope_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceContinuityAttachmentScope,
    >; 2] {
        foundational_boundary_evidence_continuity_attachment_scope_definitions()
    }

    pub const fn materialization_profile_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceMaterializationProfile,
    >; 3] {
        foundational_boundary_evidence_materialization_profile_definitions()
    }

    pub const fn attachment(self) -> FoundationalBoundaryEvidenceAttachmentFrontDoor {
        FoundationalBoundaryEvidenceAttachmentFrontDoor
    }
}
