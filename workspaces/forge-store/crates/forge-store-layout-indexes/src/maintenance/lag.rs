use crate::catalog::PhysicalArtifactFamily;
use crate::materialization::LayoutCoverageWitness;

use super::maintenance_mode::IndexMaintenanceMode;
use super::publication_protocol::IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LagReason {
    DeferredPublication,
    BackgroundCatchUp,
    RebuildRequired,
    LazyMaterialization,
    AdvisoryResidue,
    MigrationCutover,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexLagWitness {
    family: PhysicalArtifactFamily,
    coverage: LayoutCoverageWitness,
    maintenance_mode: IndexMaintenanceMode,
    publication_protocol: IndexPublicationProtocol,
    reason: LagReason,
}

impl IndexLagWitness {
    pub const fn new(
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        maintenance_mode: IndexMaintenanceMode,
        publication_protocol: IndexPublicationProtocol,
        reason: LagReason,
    ) -> Self {
        Self {
            family,
            coverage,
            maintenance_mode,
            publication_protocol,
            reason,
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn coverage(&self) -> &LayoutCoverageWitness {
        &self.coverage
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        self.publication_protocol
    }

    pub const fn reason(&self) -> LagReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexLagOutcome {
    Exact,
    Lagged(IndexLagWitness),
    NonExact(IndexMaintenanceMode),
}
