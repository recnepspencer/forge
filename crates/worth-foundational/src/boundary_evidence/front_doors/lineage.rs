use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::lineage::{
    foundational_boundary_evidence_branch_divergence_posture_definitions,
    foundational_boundary_evidence_lineage_outcome_kind_definitions,
    foundational_boundary_evidence_lineage_partiality_posture_definitions,
    foundational_boundary_evidence_promotion_posture_definitions,
    FoundationalBoundaryEvidenceBranchDivergencePosture,
    FoundationalBoundaryEvidenceLineageOutcomeKind,
    FoundationalBoundaryEvidenceLineagePartialityPosture,
    FoundationalBoundaryEvidencePromotionPosture,
};
use crate::boundary_evidence::lineage_front_doors::FoundationalBoundaryEvidenceLineageFrontDoor;
use crate::boundary_evidence::primitives::FoundationalBoundaryEvidencePrimitiveDefinition;

impl BoundaryEvidenceFrontDoor {
    pub const fn lineage_outcome_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceLineageOutcomeKind,
    >; 13] {
        foundational_boundary_evidence_lineage_outcome_kind_definitions()
    }

    pub const fn branch_divergence_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceBranchDivergencePosture,
    >; 2] {
        foundational_boundary_evidence_branch_divergence_posture_definitions()
    }

    pub const fn promotion_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidencePromotionPosture,
    >; 2] {
        foundational_boundary_evidence_promotion_posture_definitions()
    }

    pub const fn lineage_partiality_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceLineagePartialityPosture,
    >; 3] {
        foundational_boundary_evidence_lineage_partiality_posture_definitions()
    }

    pub const fn lineage(self) -> FoundationalBoundaryEvidenceLineageFrontDoor {
        FoundationalBoundaryEvidenceLineageFrontDoor
    }
}
