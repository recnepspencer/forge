use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::legality::{
    evaluate_boundary_evidence_primitive_legality,
    FoundationalBoundaryEvidencePrimitiveLegalityDenial,
};
use crate::boundary_evidence::primitives::{
    foundational_boundary_evidence_category_definitions,
    foundational_boundary_evidence_descriptive_role_definitions,
    foundational_boundary_evidence_execution_posture_definitions,
    foundational_boundary_evidence_freshness_posture_definitions,
    foundational_boundary_evidence_locality_definitions, FoundationalBoundaryEvidenceCategory,
    FoundationalBoundaryEvidenceDescriptiveRole, FoundationalBoundaryEvidenceExecutionPosture,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
    FoundationalBoundaryEvidencePrimitiveDefinition,
};

impl BoundaryEvidenceFrontDoor {
    pub const fn category_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceCategory>; 4]
    {
        foundational_boundary_evidence_category_definitions()
    }

    pub const fn locality_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceLocality>; 7]
    {
        foundational_boundary_evidence_locality_definitions()
    }

    pub const fn execution_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceExecutionPosture,
    >; 3] {
        foundational_boundary_evidence_execution_posture_definitions()
    }

    pub const fn descriptive_role_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceDescriptiveRole,
    >; 2] {
        foundational_boundary_evidence_descriptive_role_definitions()
    }

    pub const fn freshness_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceFreshnessPosture,
    >; 5] {
        foundational_boundary_evidence_freshness_posture_definitions()
    }

    pub fn primitive_legality(
        self,
        category: FoundationalBoundaryEvidenceCategory,
        locality: FoundationalBoundaryEvidenceLocality,
        execution_posture: FoundationalBoundaryEvidenceExecutionPosture,
        descriptive_role: FoundationalBoundaryEvidenceDescriptiveRole,
        freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
    ) -> Result<(), FoundationalBoundaryEvidencePrimitiveLegalityDenial> {
        evaluate_boundary_evidence_primitive_legality(
            category,
            locality,
            execution_posture,
            descriptive_role,
            freshness_posture,
        )
    }
}
