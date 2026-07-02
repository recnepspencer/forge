use forge_foundational::{
    claim_planned_work_boundary_surface, claim_support_only_boundary_surface,
    FoundationalBoundaryReportSurface, FoundationalDiagnosticArtifactKind,
    FoundationalDiagnosticBreachClass, FoundationalPlannedWorkBoundaryClaim,
    FoundationalSupportOnlyBoundaryClaim,
};
use forge_proof::{
    AssumptionBasis, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CurrentValidity, FreshnessScopedBasis, Recipe, Resolved, Unresolved,
};

use super::{
    ChunkMigrationReadInterlockPlan, FutureBlobMigrationNonClaimReport, FutureChunkStabilityBasis,
    TierMovementStabilityDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierMovementStabilityCapability {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FutureChunkStabilityResolutionAuthority {
    _private: (),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalTierMovementNonClaimEvidence {
    support: FoundationalSupportOnlyBoundaryClaim<
        FoundationalBoundaryReportSurface<FutureBlobMigrationNonClaimReport>,
    >,
    planned: FoundationalPlannedWorkBoundaryClaim<
        FoundationalBoundaryReportSurface<FutureBlobMigrationNonClaimReport>,
    >,
    artifact_kind: FoundationalDiagnosticArtifactKind,
    breach_class: FoundationalDiagnosticBreachClass,
}

pub type FutureChunkStabilityRecipe = Recipe<
    Resolved,
    ChunkMigrationReadInterlockPlan,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<FutureChunkStabilityBasis>>,
>;

impl CapabilityMarker for TierMovementStabilityCapability {}
impl AuthorityMarker for FutureChunkStabilityResolutionAuthority {}

pub fn tier_movement_stability_capability() -> CapabilityWitness<TierMovementStabilityCapability> {
    CapabilityWitness::from_capability_marker(TierMovementStabilityCapability { _private: () })
}

pub(super) fn resolve_future_chunk_stability_recipe(
    plan: ChunkMigrationReadInterlockPlan,
) -> FutureChunkStabilityRecipe {
    Recipe::<Unresolved, _>::new(plan).resolve_with_authority(
        plan.placeholder().basis(),
        AuthorityWitness::from_authority_marker(FutureChunkStabilityResolutionAuthority {
            _private: (),
        }),
    )
}

impl FoundationalTierMovementNonClaimEvidence {
    pub fn from_non_claim_report(report: FutureBlobMigrationNonClaimReport) -> Self {
        let reports = vec![report];
        Self {
            support: claim_support_only_boundary_surface(
                FoundationalBoundaryReportSurface::new(reports.clone(), 1)
                    .expect("tier movement support report has one row"),
            ),
            planned: claim_planned_work_boundary_surface(
                FoundationalBoundaryReportSurface::new(reports, 1)
                    .expect("tier movement planned report has one row"),
            ),
            artifact_kind: FoundationalDiagnosticArtifactKind::SupportReport,
            breach_class: FoundationalDiagnosticBreachClass::CoverageOmission,
        }
    }

    pub const fn support(
        &self,
    ) -> &FoundationalSupportOnlyBoundaryClaim<
        FoundationalBoundaryReportSurface<FutureBlobMigrationNonClaimReport>,
    > {
        &self.support
    }

    pub const fn planned(
        &self,
    ) -> &FoundationalPlannedWorkBoundaryClaim<
        FoundationalBoundaryReportSurface<FutureBlobMigrationNonClaimReport>,
    > {
        &self.planned
    }

    pub const fn artifact_kind(&self) -> FoundationalDiagnosticArtifactKind {
        self.artifact_kind
    }

    pub const fn breach_class(&self) -> FoundationalDiagnosticBreachClass {
        self.breach_class
    }

    pub const fn deny_blob_authority_promotion(&self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::FoundationalSurfaceCannotPromoteToBlobAuthority)
    }

    pub const fn deny_cold_tier_qos_promotion(&self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::ColdTierQosRemainsS6Scope)
    }
}
