use forge_foundational::{
    claim_support_only_boundary_surface, materialize_descriptive_boundary_surface, AspectValue,
    BoundarySourceLocator, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryReportSurface,
    FoundationalMaterializedBoundaryArtifact,
};

use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::canonical_basis::{full_profile_set, materialized_profile_set};

pub type MaterializedRecoveryPhysicsReport =
    FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReportSurface<AspectValue>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsReport {
    payload: Vec<AspectValue>,
    source_locator: BoundarySourceLocator,
    materialized: MaterializedRecoveryPhysicsReport,
}

impl RecoveryPhysicsReport {
    pub fn from_executed_source(source: &RecoveryPhysicsEvidenceSource) -> Self {
        let surface = FoundationalBoundaryReportSurface::new(source.payload().to_vec(), 1)
            .expect("executed recovery report always has rows");
        let profile = materialized_profile_set(
            full_profile_set().expect("full recovery evidence profile is coherent"),
        )
        .expect("full recovery evidence profile materializes");
        let materialized = materialize_descriptive_boundary_surface(
            claim_support_only_boundary_surface(surface),
            FoundationalBoundaryMaterializationSource::NativeAuthority,
            FoundationalBoundaryMaterializationSeam::BoundaryExchange,
            profile,
        )
        .expect("executed recovery report materializes through Foundational");
        Self {
            payload: source.payload().to_vec(),
            source_locator: source.source_locator().clone(),
            materialized,
        }
    }

    pub fn payload(&self) -> &[AspectValue] {
        &self.payload
    }

    pub const fn source_locator(&self) -> &BoundarySourceLocator {
        &self.source_locator
    }

    pub const fn materialized(&self) -> &MaterializedRecoveryPhysicsReport {
        &self.materialized
    }
}
