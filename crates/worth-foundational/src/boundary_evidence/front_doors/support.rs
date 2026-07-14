use super::BoundaryEvidenceFrontDoor;
use crate::boundary_evidence::primitives::FoundationalBoundaryEvidencePrimitiveDefinition;
use crate::boundary_evidence::support::{
    foundational_boundary_evidence_support_basis_disclosure_definitions,
    foundational_boundary_evidence_support_recovery_posture_definitions,
    foundational_boundary_evidence_support_residual_debt_kind_definitions,
    foundational_boundary_evidence_support_truth_kind_definitions,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
    FoundationalBoundaryEvidenceSupportTruthKind,
};
use crate::boundary_evidence::support_front_doors::FoundationalBoundaryEvidenceSupportFrontDoor;

impl BoundaryEvidenceFrontDoor {
    pub const fn support_truth_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceSupportTruthKind,
    >; 7] {
        foundational_boundary_evidence_support_truth_kind_definitions()
    }

    pub const fn support_recovery_posture_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceSupportRecoveryPosture,
    >; 4] {
        foundational_boundary_evidence_support_recovery_posture_definitions()
    }

    pub const fn support_basis_disclosure_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceSupportBasisDisclosure,
    >; 4] {
        foundational_boundary_evidence_support_basis_disclosure_definitions()
    }

    pub const fn support_residual_debt_kind_definitions(
        self,
    ) -> [FoundationalBoundaryEvidencePrimitiveDefinition<
        FoundationalBoundaryEvidenceSupportResidualDebtKind,
    >; 4] {
        foundational_boundary_evidence_support_residual_debt_kind_definitions()
    }

    pub const fn support(self) -> FoundationalBoundaryEvidenceSupportFrontDoor {
        FoundationalBoundaryEvidenceSupportFrontDoor
    }
}
