use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::primitives::FoundationalBoundaryEvidencePrimitiveDefinition;
use crate::boundary_evidence::receipt_front_doors::FoundationalBoundaryEvidenceReceiptFrontDoor;
use crate::boundary_evidence::receipts::{
    foundational_boundary_evidence_closeout_disposition_definitions,
    foundational_boundary_evidence_receipt_kind_definitions,
    FoundationalBoundaryEvidenceCloseoutDisposition, FoundationalBoundaryEvidenceReceiptKind,
};

impl BoundaryEvidenceFrontDoor {
    pub const fn receipt_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceReceiptKind>; 8]
    {
        foundational_boundary_evidence_receipt_kind_definitions()
    }

    pub const fn closeout_disposition_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceCloseoutDisposition,
    >; 2] {
        foundational_boundary_evidence_closeout_disposition_definitions()
    }

    pub const fn receipt(self) -> FoundationalBoundaryEvidenceReceiptFrontDoor {
        FoundationalBoundaryEvidenceReceiptFrontDoor
    }
}
