use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactFamilyLifecycleAdmission, DurableArtifactMigrationPosture,
    PhysicalArtifactFamily,
};
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::materialization::S8LayoutCoverageWitness;
use crate::strategy::{S8AdmittedLayoutStrategy, S8LayoutStrategyFamily};
use forge_store_physical_format::RootPublicationValidationWitness;

use super::lag::S8IndexLagWitness;
use super::maintenance_mode::S8IndexMaintenanceMode;
use super::publication_protocol::S8IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PhysicalMutationShape {
    ObservationOnly,
    PointRewrite,
    LogStructuredAppend,
    CompactionRewrite,
}

impl S8PhysicalMutationShape {
    pub const fn requires_write_ordering_proof(self) -> bool {
        !matches!(self, Self::ObservationOnly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ExactPublicationAuthoritySource {
    CurrentRootPublication(RootPublicationValidationWitness),
}

impl S8ExactPublicationAuthoritySource {
    pub const fn current_root_publication(validation: RootPublicationValidationWitness) -> Self {
        Self::CurrentRootPublication(validation)
    }

    pub const fn publication_protocol(self) -> S8IndexPublicationProtocol {
        match self {
            Self::CurrentRootPublication(_) => S8IndexPublicationProtocol::StableRootSwap,
        }
    }

    pub const fn supports_exact_coverage(self, _coverage: S8LayoutCoverageWitness) -> bool {
        match self {
            // Root-publication freshness alone does not prove the exact root-epoch
            // identity consumed by the layout layer.
            Self::CurrentRootPublication(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LiveMaintenanceRequest {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
    requested_lane: ArtifactFamilyAccessLane,
    maintenance_mode: S8IndexMaintenanceMode,
    mutation_shape: S8PhysicalMutationShape,
    publication_protocol: S8IndexPublicationProtocol,
    exact_publication_authority: Option<S8ExactPublicationAuthoritySource>,
    exact_coverage: Option<S8LayoutCoverageWitness>,
    lag_witness: Option<S8IndexLagWitness>,
    required_migration_posture: Option<DurableArtifactMigrationPosture>,
}

impl S8LiveMaintenanceRequest {
    pub const fn new(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        family: S8LayoutStrategyFamily,
        requested_lane: ArtifactFamilyAccessLane,
        maintenance_mode: S8IndexMaintenanceMode,
        mutation_shape: S8PhysicalMutationShape,
        publication_protocol: S8IndexPublicationProtocol,
    ) -> Self {
        Self {
            lifecycle,
            key_domain,
            family,
            requested_lane,
            maintenance_mode,
            mutation_shape,
            publication_protocol,
            exact_publication_authority: None,
            exact_coverage: None,
            lag_witness: None,
            required_migration_posture: None,
        }
    }

    pub const fn with_exact_publication_authority(
        mut self,
        authority: S8ExactPublicationAuthoritySource,
    ) -> Self {
        self.exact_publication_authority = Some(authority);
        self
    }

    pub const fn with_exact_coverage(mut self, coverage: S8LayoutCoverageWitness) -> Self {
        self.exact_coverage = Some(coverage);
        self
    }

    pub const fn with_lag_witness(mut self, witness: S8IndexLagWitness) -> Self {
        self.lag_witness = Some(witness);
        self
    }

    pub const fn require_migration_posture(
        mut self,
        posture: DurableArtifactMigrationPosture,
    ) -> Self {
        self.required_migration_posture = Some(posture);
        self
    }

    pub const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        self.key_domain
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.family
    }

    pub const fn requested_lane(self) -> ArtifactFamilyAccessLane {
        self.requested_lane
    }

    pub const fn maintenance_mode(self) -> S8IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn mutation_shape(self) -> S8PhysicalMutationShape {
        self.mutation_shape
    }

    pub const fn publication_protocol(self) -> S8IndexPublicationProtocol {
        self.publication_protocol
    }

    pub const fn exact_publication_authority(self) -> Option<S8ExactPublicationAuthoritySource> {
        self.exact_publication_authority
    }

    pub const fn exact_coverage(self) -> Option<S8LayoutCoverageWitness> {
        self.exact_coverage
    }

    pub const fn lag_witness(self) -> Option<S8IndexLagWitness> {
        self.lag_witness
    }

    pub const fn required_migration_posture(self) -> Option<DurableArtifactMigrationPosture> {
        self.required_migration_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutMutationPlan {
    admitted_strategy: S8AdmittedLayoutStrategy,
    request: S8LiveMaintenanceRequest,
}

impl S8LayoutMutationPlan {
    pub(crate) const fn new(
        admitted_strategy: S8AdmittedLayoutStrategy,
        request: S8LiveMaintenanceRequest,
    ) -> Self {
        Self {
            admitted_strategy,
            request,
        }
    }

    pub const fn admitted_strategy(self) -> S8AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn request(self) -> S8LiveMaintenanceRequest {
        self.request
    }

    pub const fn maintenance_mode(self) -> S8IndexMaintenanceMode {
        self.request.maintenance_mode()
    }

    pub const fn mutation_shape(self) -> S8PhysicalMutationShape {
        self.request.mutation_shape()
    }

    pub const fn publication_protocol(self) -> S8IndexPublicationProtocol {
        self.request.publication_protocol()
    }

    pub const fn exact_coverage(self) -> Option<S8LayoutCoverageWitness> {
        self.request.exact_coverage()
    }

    pub const fn exact_publication_authority(self) -> Option<S8ExactPublicationAuthoritySource> {
        self.request.exact_publication_authority()
    }

    pub const fn lag_witness(self) -> Option<S8IndexLagWitness> {
        self.request.lag_witness()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LiveExactMaintenanceWitness {
    family: PhysicalArtifactFamily,
    exact_coverage: S8LayoutCoverageWitness,
    maintenance_mode: S8IndexMaintenanceMode,
    publication_authority: S8ExactPublicationAuthoritySource,
}

impl S8LiveExactMaintenanceWitness {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        exact_coverage: S8LayoutCoverageWitness,
        maintenance_mode: S8IndexMaintenanceMode,
        publication_authority: S8ExactPublicationAuthoritySource,
    ) -> Self {
        Self {
            family,
            exact_coverage,
            maintenance_mode,
            publication_authority,
        }
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn exact_coverage(self) -> S8LayoutCoverageWitness {
        self.exact_coverage
    }

    pub const fn maintenance_mode(self) -> S8IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication_protocol(self) -> S8IndexPublicationProtocol {
        self.publication_authority.publication_protocol()
    }

    pub const fn publication_authority(self) -> S8ExactPublicationAuthoritySource {
        self.publication_authority
    }
}
