#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorePhysicalAuthoritySurface {
    StablePhysicalReadPlan,
    LatchOrderProof,
    PhysicalEpochBasis,
    ReclaimEligibilityProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionArtifactKind {
    FoundationalAuthoritativeCurrent,
    FoundationalDerivedProjection,
    FoundationalSupportOnly,
    FoundationalPlannedWork,
    FoundationalReceiptEvidence,
    FoundationalDiagnostic,
    FoundationalPerformanceReceipt,
    FoundationalCanonicalBasis,
    ProofProgressionTrace,
    LogOrJsonProjection,
    PlannedOrSupportArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionAuthorityDenial {
    projection: ProjectionArtifactKind,
    requested_surface: StorePhysicalAuthoritySurface,
}

impl ProjectionAuthorityDenial {
    pub const fn new(
        projection: ProjectionArtifactKind,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Self {
        Self {
            projection,
            requested_surface,
        }
    }

    pub const fn projection(self) -> ProjectionArtifactKind {
        self.projection
    }

    pub const fn requested_surface(self) -> StorePhysicalAuthoritySurface {
        self.requested_surface
    }
}

pub fn reject_foundational_projection_as_physical_isolation_store_authority(
    projection: ProjectionArtifactKind,
    requested_surface: StorePhysicalAuthoritySurface,
) -> Result<(), ProjectionAuthorityDenial> {
    Err(ProjectionAuthorityDenial::new(
        projection,
        requested_surface,
    ))
}

pub fn reject_proof_projection_as_physical_isolation_store_authority(
    requested_surface: StorePhysicalAuthoritySurface,
) -> Result<(), ProjectionAuthorityDenial> {
    Err(ProjectionAuthorityDenial::new(
        ProjectionArtifactKind::ProofProgressionTrace,
        requested_surface,
    ))
}

pub fn reject_log_or_json_projection_as_physical_isolation_store_authority(
    requested_surface: StorePhysicalAuthoritySurface,
) -> Result<(), ProjectionAuthorityDenial> {
    Err(ProjectionAuthorityDenial::new(
        ProjectionArtifactKind::LogOrJsonProjection,
        requested_surface,
    ))
}

pub fn reject_planned_or_support_projection_as_physical_isolation_store_authority(
    requested_surface: StorePhysicalAuthoritySurface,
) -> Result<(), ProjectionAuthorityDenial> {
    Err(ProjectionAuthorityDenial::new(
        ProjectionArtifactKind::PlannedOrSupportArtifact,
        requested_surface,
    ))
}

pub fn reject_projection_as_stable_physical_read_plan_authority(
    projection: ProjectionArtifactKind,
) -> Result<(), ProjectionAuthorityDenial> {
    reject_foundational_projection_as_physical_isolation_store_authority(
        projection,
        StorePhysicalAuthoritySurface::StablePhysicalReadPlan,
    )
}

pub fn reject_projection_as_latch_order_proof_authority(
    projection: ProjectionArtifactKind,
) -> Result<(), ProjectionAuthorityDenial> {
    reject_foundational_projection_as_physical_isolation_store_authority(
        projection,
        StorePhysicalAuthoritySurface::LatchOrderProof,
    )
}

pub fn reject_projection_as_physical_epoch_basis_authority(
    projection: ProjectionArtifactKind,
) -> Result<(), ProjectionAuthorityDenial> {
    reject_foundational_projection_as_physical_isolation_store_authority(
        projection,
        StorePhysicalAuthoritySurface::PhysicalEpochBasis,
    )
}

pub fn reject_projection_as_reclaim_eligibility_proof_authority(
    projection: ProjectionArtifactKind,
) -> Result<(), ProjectionAuthorityDenial> {
    reject_foundational_projection_as_physical_isolation_store_authority(
        projection,
        StorePhysicalAuthoritySurface::ReclaimEligibilityProof,
    )
}
