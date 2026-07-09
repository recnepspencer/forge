use worth_foundational::{
    admit_authoritative_current_boundary_surface, claim_derived_projection_boundary_surface,
    claim_receipt_evidence_boundary_surface, claim_support_only_boundary_surface,
    foundational_boundary_authority_admission, FoundationalAuthoritativeBoundaryClaim,
    FoundationalBoundaryArtifactSurface, FoundationalBoundaryReceiptSurface,
    FoundationalBoundaryReportSurface, FoundationalDerivedProjectionBoundaryClaim,
    FoundationalReceiptEvidenceBoundaryClaim, FoundationalSupportOnlyBoundaryClaim,
};

use super::{
    PhysicalReadStabilityAuthority, PhysicalSnapshotCorrelation, SemanticVisibilityReference,
};

pub type SemanticVisibilitySupportClaim = FoundationalSupportOnlyBoundaryClaim<
    FoundationalBoundaryReportSurface<SemanticVisibilityReference>,
>;
pub type SemanticVisibilityProjectionClaim = FoundationalDerivedProjectionBoundaryClaim<
    FoundationalBoundaryReportSurface<SemanticVisibilityReference>,
>;
pub type PhysicalSnapshotCorrelationReceiptClaim =
    FoundationalReceiptEvidenceBoundaryClaim<FoundationalBoundaryReceiptSurface>;
pub type StorePhysicalAuthorityRoleClaim = FoundationalAuthoritativeBoundaryClaim<
    FoundationalBoundaryArtifactSurface<PhysicalReadStabilityAuthority>,
>;

pub struct PhysicalSemanticBoundaryRoleEvidence {
    semantic_support: SemanticVisibilitySupportClaim,
    semantic_projection: SemanticVisibilityProjectionClaim,
    correlation_receipt: PhysicalSnapshotCorrelationReceiptClaim,
    store_physical_authority: StorePhysicalAuthorityRoleClaim,
}

impl PhysicalSemanticBoundaryRoleEvidence {
    pub fn from_correlation_and_authority(
        correlation: &PhysicalSnapshotCorrelation,
        authority: &PhysicalReadStabilityAuthority,
    ) -> Self {
        let semantic_rows = vec![correlation.semantic().clone()];
        let semantic_support = claim_support_only_boundary_surface(
            FoundationalBoundaryReportSurface::new(semantic_rows.clone(), 1)
                .expect("semantic visibility support report has one row"),
        );
        let semantic_projection = claim_derived_projection_boundary_surface(
            FoundationalBoundaryReportSurface::new(semantic_rows, 1)
                .expect("semantic visibility projection report has one row"),
        );
        let correlation_receipt = claim_receipt_evidence_boundary_surface(
            FoundationalBoundaryReceiptSurface::new("physical snapshot correlation", 1)
                .expect("correlation receipt names completed boundary"),
        );
        let store_physical_authority = admit_authoritative_current_boundary_surface(
            FoundationalBoundaryArtifactSurface::new(authority.clone(), 0),
            foundational_boundary_authority_admission(),
        );
        Self {
            semantic_support,
            semantic_projection,
            correlation_receipt,
            store_physical_authority,
        }
    }

    pub const fn semantic_support(&self) -> &SemanticVisibilitySupportClaim {
        &self.semantic_support
    }

    pub const fn semantic_projection(&self) -> &SemanticVisibilityProjectionClaim {
        &self.semantic_projection
    }

    pub const fn correlation_receipt(&self) -> &PhysicalSnapshotCorrelationReceiptClaim {
        &self.correlation_receipt
    }

    pub const fn store_physical_authority(&self) -> &StorePhysicalAuthorityRoleClaim {
        &self.store_physical_authority
    }
}
