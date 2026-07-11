use crate::catalog::PhysicalArtifactFamily;
use crate::materialization::S8LayoutCoverageWitness;

use super::maintenance_mode::S8IndexMaintenanceMode;
use super::publication_protocol::S8IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LagReason {
    DeferredPublication,
    BackgroundCatchUp,
    RebuildRequired,
    LazyMaterialization,
    AdvisoryResidue,
    MigrationCutover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8IndexLagWitness {
    family: PhysicalArtifactFamily,
    coverage: S8LayoutCoverageWitness,
    maintenance_mode: S8IndexMaintenanceMode,
    publication_protocol: S8IndexPublicationProtocol,
    reason: S8LagReason,
}

impl S8IndexLagWitness {
    pub const fn new(
        family: PhysicalArtifactFamily,
        coverage: S8LayoutCoverageWitness,
        maintenance_mode: S8IndexMaintenanceMode,
        publication_protocol: S8IndexPublicationProtocol,
        reason: S8LagReason,
    ) -> Self {
        Self {
            family,
            coverage,
            maintenance_mode,
            publication_protocol,
            reason,
        }
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }

    pub const fn maintenance_mode(self) -> S8IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication_protocol(self) -> S8IndexPublicationProtocol {
        self.publication_protocol
    }

    pub const fn reason(self) -> S8LagReason {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexLagOutcome {
    Exact,
    Lagged(S8IndexLagWitness),
    NonExact(S8IndexMaintenanceMode),
}
