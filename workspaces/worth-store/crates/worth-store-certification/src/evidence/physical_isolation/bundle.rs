use super::{
    canonical::S5CanonicalMaterializationDenial, ExecutedPhysicalIsolationEvidenceSource,
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationSourceDenial,
    S5FoundationalCanonicalBasis, S5FoundationalDiagnostics, S5FoundationalPerformanceReceipts,
    S5PhysicalIsolationProofTrace,
};
use worth_foundational::FoundationalDiagnosticMaterializationDenial;
use worth_store_physical_isolation::{
    reject_foundational_projection_as_physical_isolation_store_authority,
    reject_log_or_json_projection_as_physical_isolation_store_authority,
    reject_planned_or_support_projection_as_physical_isolation_store_authority,
    reject_projection_as_latch_order_proof_authority,
    reject_projection_as_physical_epoch_basis_authority,
    reject_projection_as_reclaim_eligibility_proof_authority,
    reject_projection_as_stable_physical_read_plan_authority,
    reject_proof_projection_as_physical_isolation_store_authority, ProjectionArtifactKind,
    ProjectionAuthorityDenial, StorePhysicalAuthoritySurface,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5ExecutedIsolationEvidenceBundle {
    source_finding: ExecutedPhysicalIsolationFinding,
    diagnostics: S5FoundationalDiagnostics,
    performance: S5FoundationalPerformanceReceipts,
    canonical: S5FoundationalCanonicalBasis,
    proof: S5PhysicalIsolationProofTrace,
}

impl S5ExecutedIsolationEvidenceBundle {
    pub const fn source_finding(&self) -> &ExecutedPhysicalIsolationFinding {
        &self.source_finding
    }

    pub const fn diagnostics(&self) -> &S5FoundationalDiagnostics {
        &self.diagnostics
    }

    pub const fn performance(&self) -> &S5FoundationalPerformanceReceipts {
        &self.performance
    }

    pub const fn canonical(&self) -> &S5FoundationalCanonicalBasis {
        &self.canonical
    }

    pub const fn proof(&self) -> &S5PhysicalIsolationProofTrace {
        &self.proof
    }

    pub fn reject_foundational_as_store_authority(
        &self,
        projection: ProjectionArtifactKind,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Result<(), ProjectionAuthorityDenial> {
        reject_foundational_projection_as_physical_isolation_store_authority(
            projection,
            requested_surface,
        )
    }

    pub fn reject_proof_as_store_authority(
        &self,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Result<(), ProjectionAuthorityDenial> {
        reject_proof_projection_as_physical_isolation_store_authority(requested_surface)
    }

    pub fn reject_log_or_json_as_store_authority(
        &self,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Result<(), ProjectionAuthorityDenial> {
        reject_log_or_json_projection_as_physical_isolation_store_authority(requested_surface)
    }

    pub fn reject_planned_or_support_as_store_authority(
        &self,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Result<(), ProjectionAuthorityDenial> {
        reject_planned_or_support_projection_as_physical_isolation_store_authority(
            requested_surface,
        )
    }

    pub fn reject_projection_as_store_authority(
        &self,
        projection: ProjectionArtifactKind,
        requested_surface: StorePhysicalAuthoritySurface,
    ) -> Result<(), ProjectionAuthorityDenial> {
        match requested_surface {
            StorePhysicalAuthoritySurface::StablePhysicalReadPlan => {
                reject_projection_as_stable_physical_read_plan_authority(projection)
            }
            StorePhysicalAuthoritySurface::LatchOrderProof => {
                reject_projection_as_latch_order_proof_authority(projection)
            }
            StorePhysicalAuthoritySurface::PhysicalEpochBasis => {
                reject_projection_as_physical_epoch_basis_authority(projection)
            }
            StorePhysicalAuthoritySurface::ReclaimEligibilityProof => {
                reject_projection_as_reclaim_eligibility_proof_authority(projection)
            }
        }
    }
}

pub fn materialize_physical_isolation_executed_isolation_evidence(
    source: ExecutedPhysicalIsolationEvidenceSource,
) -> Result<S5ExecutedIsolationEvidenceBundle, S5ExecutedIsolationMaterializationDenial> {
    let finding = source.finding().clone();
    let diagnostics = S5FoundationalDiagnostics::from_finding(&finding)?;
    let performance = S5FoundationalPerformanceReceipts::from_finding(&finding)?;
    let canonical = S5FoundationalCanonicalBasis::from_finding(&finding)?;
    let proof = S5PhysicalIsolationProofTrace::from_finding(&finding);
    Ok(S5ExecutedIsolationEvidenceBundle {
        source_finding: finding,
        diagnostics,
        performance,
        canonical,
        proof,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S5ExecutedIsolationMaterializationDenial {
    Source(ExecutedPhysicalIsolationSourceDenial),
    Diagnostics(FoundationalDiagnosticMaterializationDenial),
    Performance(crate::FoundationalBoundaryEvidenceDenial),
    Canonical(S5CanonicalMaterializationDenial),
}

impl From<ExecutedPhysicalIsolationSourceDenial> for S5ExecutedIsolationMaterializationDenial {
    fn from(denial: ExecutedPhysicalIsolationSourceDenial) -> Self {
        Self::Source(denial)
    }
}

impl From<FoundationalDiagnosticMaterializationDenial>
    for S5ExecutedIsolationMaterializationDenial
{
    fn from(denial: FoundationalDiagnosticMaterializationDenial) -> Self {
        Self::Diagnostics(denial)
    }
}

impl From<crate::FoundationalBoundaryEvidenceDenial> for S5ExecutedIsolationMaterializationDenial {
    fn from(denial: crate::FoundationalBoundaryEvidenceDenial) -> Self {
        Self::Performance(denial)
    }
}

impl From<S5CanonicalMaterializationDenial> for S5ExecutedIsolationMaterializationDenial {
    fn from(denial: S5CanonicalMaterializationDenial) -> Self {
        Self::Canonical(denial)
    }
}
