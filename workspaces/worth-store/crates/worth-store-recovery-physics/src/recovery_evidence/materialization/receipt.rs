use worth_foundational::{
    claim_receipt_evidence_boundary_surface, materialize_descriptive_boundary_surface,
    BoundaryArtifactLocator, BoundaryEpoch, BoundaryHandle,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryReceiptSurface, FoundationalMaterializedBoundaryArtifact,
};

use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::canonical_basis::{full_profile_set, materialized_profile_set};

pub type MaterializedRecoveryPhysicsReceipt =
    FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsReceipt {
    handle: BoundaryHandle,
    epoch: BoundaryEpoch,
    artifact_locator: BoundaryArtifactLocator,
    recovered_physical_root: String,
    materialized: MaterializedRecoveryPhysicsReceipt,
}

impl RecoveryPhysicsReceipt {
    pub fn from_executed_source(source: &RecoveryPhysicsEvidenceSource) -> Self {
        let surface = FoundationalBoundaryReceiptSurface::new(
            "store recovery executed and materialized for S.4 evidence",
            source.counters().replayed_frames(),
        )
        .expect("static recovery receipt boundary is named");
        let profile = materialized_profile_set(
            full_profile_set().expect("full recovery evidence profile is coherent"),
        )
        .expect("full recovery evidence profile materializes");
        let materialized = materialize_descriptive_boundary_surface(
            claim_receipt_evidence_boundary_surface(surface),
            FoundationalBoundaryMaterializationSource::NativeAuthority,
            FoundationalBoundaryMaterializationSeam::BoundaryExchange,
            profile,
        )
        .expect("executed recovery receipt materializes through Foundational");
        Self {
            handle: source.authority().handle(),
            epoch: source.authority().epoch(),
            artifact_locator: source.artifact_locator(),
            recovered_physical_root: source
                .recovered_state()
                .recovered_physical_root()
                .to_string(),
            materialized,
        }
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.handle
    }

    pub const fn epoch(&self) -> BoundaryEpoch {
        self.epoch
    }

    pub const fn artifact_locator(&self) -> BoundaryArtifactLocator {
        self.artifact_locator
    }

    pub fn recovered_physical_root(&self) -> &str {
        &self.recovered_physical_root
    }

    pub const fn materialized(&self) -> &MaterializedRecoveryPhysicsReceipt {
        &self.materialized
    }
}
