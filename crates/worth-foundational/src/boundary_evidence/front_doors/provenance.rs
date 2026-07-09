use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::provenance::{
    foundational_boundary_evidence_provenance_layer_definitions,
    foundational_boundary_evidence_source_basis_kind_definitions,
    FoundationalBoundaryEvidenceProvenanceLayerKind, FoundationalBoundaryEvidenceSourceBasisKind,
};
use crate::boundary_evidence::provenance_front_doors::FoundationalBoundaryEvidenceProvenanceFrontDoor;
use crate::boundary_evidence::FoundationalBoundaryEvidencePrimitiveDefinition;

impl BoundaryEvidenceFrontDoor {
    pub const fn provenance_layer_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceProvenanceLayerKind,
    >; 7] {
        foundational_boundary_evidence_provenance_layer_definitions()
    }

    pub const fn source_basis_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceSourceBasisKind,
    >; 2] {
        foundational_boundary_evidence_source_basis_kind_definitions()
    }

    pub const fn provenance(self) -> FoundationalBoundaryEvidenceProvenanceFrontDoor {
        FoundationalBoundaryEvidenceProvenanceFrontDoor
    }
}
